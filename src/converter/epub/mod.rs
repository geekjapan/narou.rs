//! Java/AozoraEpub3 非依存のネイティブ EPUB3 生成。
//!
//! 入力は `render_novel_text` が出力する青空文庫形式の中間テキスト（AozoraEpub3 に渡すものと同一）。
//! これを解析し、縦書き EPUB3 を Rust 単体で生成する。
//!
//! 構成: parser(中間表現) → xhtml(本文/目次/タイトル) → package(OPF/container/ZIP)。assets(CSS/font)。

pub mod assets;
pub mod gaiji;
pub mod package;
pub mod parser;
pub mod xhtml;

use std::path::{Path, PathBuf};

use regex::Regex;

use crate::error::{NarouError, Result};

use package::{ManifestItem, OpfMeta};
use xhtml::{IllustMap, NavEntry};

/// EPUB 生成オプション。
#[derive(Debug, Clone)]
pub struct EpubOptions {
    /// フォント埋め込み（grill 既定 OFF）。
    pub embed_font: bool,
    /// 挿絵の埋め込み。
    pub include_illust: bool,
    /// 行高（global setting `line-height`、既定 1.8）。
    pub line_height: f64,
}

impl Default for EpubOptions {
    fn default() -> Self {
        Self {
            embed_font: false,
            include_illust: true,
            line_height: 1.8,
        }
    }
}

/// 青空文庫形式中間テキストファイルから EPUB3 を生成する。
///
/// 出力名は既存経路と同一（`output_dir/{stem}{output_ext}`）。
pub fn build_epub(
    input_txt: &Path,
    output_dir: &Path,
    output_ext: &str,
    opts: &EpubOptions,
) -> Result<PathBuf> {
    let text = std::fs::read_to_string(input_txt)?;
    let stem = input_txt
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| NarouError::Conversion("Invalid input filename".into()))?;
    let output_path = output_dir.join(format!("{stem}{output_ext}"));

    let doc = parser::parse_document(&text);

    // dc:title / <title> は空にできないためフォールバックする。
    let display_title = if doc.title.trim().is_empty() {
        "無題".to_string()
    } else {
        doc.title.clone()
    };

    let mut items: Vec<ManifestItem> = Vec::new();

    // 目次(nav) はあとで本文項目が揃ってから差し込むため、プレースホルダのインデックスを記録。
    // CSS。
    items.push(ManifestItem {
        id: "book-style".into(),
        href: "style/book-style.css".into(),
        media_type: "text/css".into(),
        properties: None,
        in_spine: false,
        zip_path: "item/style/book-style.css".into(),
        data: assets::book_style_css(opts.line_height, opts.embed_font).into_bytes(),
    });

    // フォント（埋め込み時のみ）。
    if opts.embed_font {
        items.push(ManifestItem {
            id: "font-mincho".into(),
            href: assets::FONT_FILE.into(),
            media_type: assets::media_type_for(assets::FONT_FILE).into(),
            properties: None,
            in_spine: false,
            zip_path: format!("item/{}", assets::FONT_FILE),
            data: assets::DMINCHO_TTF.to_vec(),
        });
    }

    // 挿絵。中間テキストを走査し、実在する画像のみ埋め込む。
    let illust_map = collect_illustrations(&text, output_dir, opts.include_illust, &mut items)?;

    // 目次(nav)。
    items.push(ManifestItem {
        id: "nav".into(),
        href: "nav.xhtml".into(),
        media_type: "application/xhtml+xml".into(),
        properties: Some("nav".into()),
        in_spine: false,
        zip_path: "item/nav.xhtml".into(),
        data: Vec::new(), // 後で埋める
    });
    let nav_index = items.len() - 1;

    // タイトルページ（spine 先頭）。
    items.push(ManifestItem {
        id: "title-page".into(),
        href: "xhtml/title.xhtml".into(),
        media_type: "application/xhtml+xml".into(),
        properties: None,
        in_spine: true,
        zip_path: "item/xhtml/title.xhtml".into(),
        data: xhtml::render_title_xhtml(&display_title, &doc.author).into_bytes(),
    });

    // 本文ページ。
    let mut nav_entries: Vec<NavEntry> = vec![NavEntry {
        href: "xhtml/title.xhtml".into(),
        label: display_title.clone(),
    }];
    for (idx, page) in doc.pages.iter().enumerate() {
        let file = format!("{:04}.xhtml", idx + 1);
        let id = format!("sec{:04}", idx + 1);
        let data = xhtml::render_page_xhtml(page, &display_title, &illust_map).into_bytes();
        if let Some(label) = &page.nav_label {
            nav_entries.push(NavEntry {
                href: format!("xhtml/{file}"),
                label: label.clone(),
            });
        }
        items.push(ManifestItem {
            id,
            href: format!("xhtml/{file}"),
            media_type: "application/xhtml+xml".into(),
            properties: None,
            in_spine: true,
            zip_path: format!("item/xhtml/{file}"),
            data,
        });
    }

    // nav の本文を確定。
    items[nav_index].data = xhtml::render_nav_xhtml(&display_title, &nav_entries).into_bytes();

    let seed = identifier_seed(&text, &display_title, &doc.author);
    let meta = OpfMeta {
        title: display_title.clone(),
        author: doc.author.clone(),
        identifier: package::deterministic_urn_uuid(&seed),
        modified: modified_timestamp(input_txt),
    };

    let opf = package::build_opf(&meta, &items);
    package::write_epub(&output_path, package::container_xml(), &opf, &items)?;

    Ok(output_path)
}

/// 中間テキストの挿絵注記を走査し、実在画像を manifest へ追加して `IllustMap` を返す。
fn collect_illustrations(
    text: &str,
    output_dir: &Path,
    include_illust: bool,
    items: &mut Vec<ManifestItem>,
) -> Result<IllustMap> {
    let mut map = IllustMap::new();
    if !include_illust {
        return Ok(map);
    }
    let re = Regex::new(r"［＃挿絵（(.+?)）入る］")?;
    let mut seq = 0usize;
    for caps in re.captures_iter(text) {
        let rel = caps[1].to_string();
        if map.contains_key(&rel) {
            continue;
        }
        // セキュリティ: 挿絵パスは DL コンテンツ由来のため、output_dir 配下に収まる
        // 安全な相対パスのみ許可する（絶対パス・`..`・ドライブ接頭辞・シンボリックリンク脱出を拒否）。
        let Some(src) = safe_asset_path(output_dir, &rel) else {
            continue;
        };
        let data = std::fs::read(&src)?;
        seq += 1;
        let ext = Path::new(&rel)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg");
        let file = format!("img{seq:04}.{ext}");
        let zip_path = format!("item/image/{file}");
        let href_from_xhtml = format!("../image/{file}");
        let media_type = assets::media_type_for(&file).to_string();
        items.push(ManifestItem {
            id: format!("img{seq:04}"),
            href: format!("image/{file}"),
            media_type,
            properties: None,
            in_spine: false,
            zip_path,
            data,
        });
        map.insert(rel, href_from_xhtml);
    }
    Ok(map)
}

/// 挿絵の相対パスを検証し、`output_dir` 配下に収まる安全な実体パスのみ `Some` で返す。
/// 中間テキストはダウンロードした小説本文由来のため、パストラバーサル
/// （絶対パス `/etc/passwd`・`../` 脱出・ドライブ接頭辞・シンボリックリンク脱出）を防ぐ。
fn safe_asset_path(output_dir: &Path, rel: &str) -> Option<PathBuf> {
    use std::path::Component;
    // セパレータ混在による検証回避を防ぐ（Aozora 形式は `/` 区切りのみ想定）。
    if rel.contains('\\') {
        return None;
    }
    let mut has_normal = false;
    for comp in Path::new(rel).components() {
        match comp {
            Component::Normal(_) => has_normal = true,
            Component::CurDir => {}
            // RootDir / Prefix（絶対・ドライブ） / ParentDir（..）はすべて拒否。
            _ => return None,
        }
    }
    if !has_normal {
        return None;
    }
    // 実体（シンボリックリンク解決後）が output_dir 配下に収まることを確認（多層防御）。
    let real = output_dir.join(rel).canonicalize().ok()?;
    let base = output_dir.canonicalize().ok()?;
    real.starts_with(&base).then_some(real)
}

/// `dc:identifier` 用の決定論シード。掲載ページ URL があればそれを、無ければ題名+著者。
fn identifier_seed(text: &str, title: &str, author: &str) -> String {
    if let Some(re) = Regex::new(r#"<a href="([^"]+)">"#).ok() {
        if let Some(caps) = re.captures(text) {
            return caps[1].to_string();
        }
    }
    format!("{title}\n{author}")
}

/// `dcterms:modified`。入力ファイルの最終更新時刻（決定論的）を UTC ISO8601 で返す。
fn modified_timestamp(input_txt: &Path) -> String {
    let systime = std::fs::metadata(input_txt)
        .and_then(|m| m.modified())
        .ok();
    // メタデータ取得失敗時も決定論を保つため、現在時刻ではなく UNIX_EPOCH へフォールバックする。
    let dt = systime
        .map(chrono::DateTime::<chrono::Utc>::from)
        .unwrap_or_else(|| chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::UNIX_EPOCH));
    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn read_zip_entry(epub: &Path, name: &str) -> Option<String> {
        let file = std::fs::File::open(epub).ok()?;
        let mut zip = zip::ZipArchive::new(file).ok()?;
        let mut entry = zip.by_name(name).ok()?;
        let mut s = String::new();
        entry.read_to_string(&mut s).ok()?;
        Some(s)
    }

    #[test]
    fn builds_minimal_epub() {
        let dir = std::env::temp_dir().join(format!("narou_epub_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let txt = dir.join("テスト小説.txt");
        let content = "テスト小説\n著者\n\n［＃区切り線］\nあらすじ：\n本文だよ\n掲載ページ:\n<a href=\"https://example.com/n0000aa/\">https://example.com/n0000aa/</a>\n［＃区切り線］\n［＃改ページ］\n［＃３字下げ］［＃中見出し］第一話［＃中見出し終わり］\n\n　｜本文《ほんぶん》です。\n";
        std::fs::write(&txt, content).unwrap();

        let out = build_epub(&txt, &dir, ".epub", &EpubOptions::default()).unwrap();
        assert!(out.exists());

        // mimetype 先頭・無圧縮を確認。
        let file = std::fs::File::open(&out).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        {
            let first = zip.by_index(0).unwrap();
            assert_eq!(first.name(), "mimetype");
            assert_eq!(first.compression(), zip::CompressionMethod::Stored);
        }

        let opf = read_zip_entry(&out, "item/standard.opf").unwrap();
        assert!(opf.contains("version=\"3.0\""));
        assert!(opf.contains("page-progression-direction=\"rtl\""));
        assert!(opf.contains("</metadata>"));

        let container = read_zip_entry(&out, "META-INF/container.xml").unwrap();
        assert!(container.contains("item/standard.opf"));

        let nav = read_zip_entry(&out, "item/nav.xhtml").unwrap();
        assert!(nav.contains("第一話"));

        let page2 = read_zip_entry(&out, "item/xhtml/0002.xhtml").unwrap();
        assert!(page2.contains("<ruby>本文<rt>ほんぶん</rt></ruby>"));
        assert!(page2.contains("class=\"vrtl\""));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn embeds_existing_illustration() {
        let dir = std::env::temp_dir().join(format!("narou_epub_illust_{}", std::process::id()));
        let illust_dir = dir.join("挿絵");
        std::fs::create_dir_all(&illust_dir).unwrap();
        std::fs::write(illust_dir.join("1.png"), b"\x89PNG\r\n\x1a\nfakeimage").unwrap();

        let txt = dir.join("挿絵テスト.txt");
        let content = "挿絵テスト\n著者\n本文\n［＃改ページ］\n挿絵の前\n［＃挿絵（挿絵/1.png）入る］\n挿絵の後\n";
        std::fs::write(&txt, content).unwrap();

        let out = build_epub(&txt, &dir, ".epub", &EpubOptions::default()).unwrap();

        // 画像が ZIP・manifest に入り、本文から参照される。
        let file = std::fs::File::open(&out).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        assert!(zip.by_name("item/image/img0001.png").is_ok());

        let opf = read_zip_entry(&out, "item/standard.opf").unwrap();
        assert!(opf.contains("image/img0001.png"));

        let page = read_zip_entry(&out, "item/xhtml/0002.xhtml").unwrap();
        assert!(page.contains("<img src=\"../image/img0001.png\""));
        // 挿絵はブロックレベルの <p class="illust"> として出力し、<p> をネストしない。
        assert!(page.contains("<p class=\"illust\"><img src=\"../image/img0001.png\" alt=\"\"/></p>"));
        assert!(!page.contains("<p><p"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn excludes_illustration_when_disabled() {
        let dir = std::env::temp_dir().join(format!("narou_epub_noillust_{}", std::process::id()));
        let illust_dir = dir.join("挿絵");
        std::fs::create_dir_all(&illust_dir).unwrap();
        std::fs::write(illust_dir.join("1.png"), b"fake").unwrap();
        let txt = dir.join("挿絵オフ.txt");
        std::fs::write(
            &txt,
            "挿絵オフ\n著者\n本文\n［＃挿絵（挿絵/1.png）入る］\n",
        )
        .unwrap();

        let opts = EpubOptions {
            include_illust: false,
            ..EpubOptions::default()
        };
        let out = build_epub(&txt, &dir, ".epub", &opts).unwrap();
        let file = std::fs::File::open(&out).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        assert!(zip.by_name("item/image/img0001.png").is_err());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn rejects_path_traversal_illustration() {
        let dir = std::env::temp_dir().join(format!("narou_epub_trav_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // output_dir の外に「機密」ファイルを置く。
        let secret = dir.parent().unwrap().join(format!("secret_{}.png", std::process::id()));
        std::fs::write(&secret, b"\x89PNG\r\n\x1a\ntopsecret").unwrap();

        let txt = dir.join("脱出.txt");
        let content = format!(
            "脱出\n著者\n本文\n［＃挿絵（../{}）入る］\n",
            secret.file_name().unwrap().to_str().unwrap()
        );
        std::fs::write(&txt, content).unwrap();

        let out = build_epub(&txt, &dir, ".epub", &EpubOptions::default()).unwrap();
        let file = std::fs::File::open(&out).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        // 親ディレクトリのファイルは EPUB に埋め込まれない。
        assert!(zip.by_name("item/image/img0001.png").is_err());

        std::fs::remove_file(&secret).ok();
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn safe_asset_path_rejects_unsafe_and_accepts_relative() {
        let dir = std::env::temp_dir().join(format!("narou_epub_safe_{}", std::process::id()));
        std::fs::create_dir_all(dir.join("挿絵")).unwrap();
        std::fs::write(dir.join("挿絵/1.png"), b"x").unwrap();

        assert!(safe_asset_path(&dir, "挿絵/1.png").is_some());
        assert!(safe_asset_path(&dir, "../etc/passwd").is_none());
        assert!(safe_asset_path(&dir, "/etc/passwd").is_none());
        assert!(safe_asset_path(&dir, "挿絵/../../escape").is_none());

        std::fs::remove_dir_all(&dir).ok();
    }
}
