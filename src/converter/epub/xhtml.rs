//! 中間表現(AST)から EPUB の XHTML を生成する。
//!
//! - インライン注記（ルビ・縦中横・傍点・太字・斜体・取消線・濁点・外字）を XHTML へ変換。
//! - ブロック（見出し・前書き等の範囲・区切り線・空行）を本文 XHTML へ変換。
//! - 本文ページ / タイトルページ / 目次(nav) の XHTML ドキュメントを生成。
//!
//! クラス語彙は参照 EPUB(AozoraEpub3) 互換（`vrtl`/`hltr`/`tcy`/`introduction`/`mt3` 等）に揃える。

use std::collections::HashMap;

use super::gaiji;
use super::parser::{Block, DivKind, HeadingLevel, Page};

/// 本文 CSS の相対パス（`item/xhtml/*.xhtml` から見て）。
const STYLE_HREF: &str = "../style/book-style.css";

/// 挿絵の相対パス（中間テキスト中の `挿絵/…`）→ 本文 XHTML から見た `src` href のマップ。
pub type IllustMap = HashMap<String, String>;

/// 目次の 1 項目。
pub struct NavEntry {
    pub href: String,
    pub label: String,
}

/// インライン注記を XHTML 断片へ変換する（挿絵解決なし）。
pub fn render_inline(s: &str) -> String {
    render_inline_ctx(s, None)
}

/// インライン注記を XHTML 断片へ変換する。未知注記は安全に除去する（フェイルセーフ）。
fn render_inline_ctx(s: &str, illust: Option<&IllustMap>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let n = chars.len();

    while i < n {
        // 既存の <a ...>...</a> アンカーはそのまま通す（属性・本文は健全化）。
        if starts_with(&chars, i, "<a ") {
            if let Some(end) = find_sub(&chars, i, "</a>") {
                let frag: String = chars[i..end + 4].iter().collect();
                out.push_str(&sanitize_anchor(&frag));
                i = end + 4;
                continue;
            }
        }

        // ※［＃ ... ］ 形式の名前付き外字。
        if starts_with(&chars, i, "※［＃") {
            if let Some(close) = find_char(&chars, i + 3, '］') {
                let inner: String = chars[i + 3..close].iter().collect();
                match gaiji::resolve_named(&inner) {
                    Some(c) => out.push_str(&escape_text(&c)),
                    None => out.push(gaiji::FALLBACK_CHAR),
                }
                i = close + 1;
                continue;
            }
        }

        // ［＃ ... ］ 形式（ペアのインライン注記 / 面区点外字 / 未知）。
        if starts_with(&chars, i, "［＃") {
            if let Some(close) = find_char(&chars, i + 2, '］') {
                let tag: String = chars[i + 2..close].iter().collect();

                // 挿絵 ［＃挿絵（PATH）入る］。
                if let Some(path) = tag
                    .strip_prefix("挿絵（")
                    .and_then(|r| r.strip_suffix("）入る"))
                {
                    // 行内に混在した場合は <p> をネストさせないよう裸の <img/> を出す。
                    // 行全体が挿絵のみの場合はブロック側 (render_illust_line) が処理する。
                    if let Some(href) = illust.and_then(|m| m.get(path)) {
                        out.push_str(&format!(
                            "<img src=\"{}\" alt=\"\"/>",
                            escape_attr(href)
                        ));
                    }
                    i = close + 1;
                    continue;
                }

                if let Some((closer, (open_html, close_html))) = inline_open_tag(&tag) {
                    let end_marker: Vec<char> = format!("［＃{closer}］").chars().collect();
                    if let Some(epos) = find_seq(&chars, close + 1, &end_marker) {
                        let inner: String = chars[close + 1..epos].iter().collect();
                        out.push_str(open_html);
                        out.push_str(&render_inline_ctx(&inner, illust));
                        out.push_str(close_html);
                        i = epos + end_marker.len();
                        continue;
                    }
                }

                if tag.contains('、') {
                    if let Some(c) = gaiji::resolve_named(&tag) {
                        out.push_str(&escape_text(&c));
                        i = close + 1;
                        continue;
                    }
                }

                // 未知注記はマーカーのみ除去し、本文は残す。
                i = close + 1;
                continue;
            }
        }

        // ルビ ｜親《ルビ》。
        if chars[i] == '｜' {
            if let Some(obrace) = find_char(&chars, i + 1, '《') {
                if let Some(cbrace) = find_char(&chars, obrace + 1, '》') {
                    let base: String = chars[i + 1..obrace].iter().collect();
                    let ruby: String = chars[obrace + 1..cbrace].iter().collect();
                    out.push_str("<ruby>");
                    out.push_str(&render_inline_ctx(&base, illust));
                    out.push_str("<rt>");
                    out.push_str(&escape_text(&ruby));
                    out.push_str("</rt></ruby>");
                    i = cbrace + 1;
                    continue;
                }
            }
        }

        push_escaped(&mut out, chars[i]);
        i += 1;
    }

    out
}

fn inline_open_tag(tag: &str) -> Option<(&'static str, (&'static str, &'static str))> {
    let m = match tag {
        "縦中横" => ("縦中横終わり", ("<span class=\"tcy\">", "</span>")),
        "傍点" => ("傍点終わり", ("<em class=\"em-sesame\">", "</em>")),
        "太字" => ("太字終わり", ("<b>", "</b>")),
        "斜体" => ("斜体終わり", ("<i>", "</i>")),
        "取消線" => ("取消線終わり", ("<del>", "</del>")),
        "濁点" => ("濁点終わり", ("<span class=\"dakuten\">", "</span>")),
        _ => return None,
    };
    Some(m)
}

/// 本文ページの XHTML ドキュメントを生成する。
pub fn render_page_xhtml(page: &Page, title: &str, illust: &IllustMap) -> String {
    let body = render_page_body(page, illust);
    format!(
        "{header}{body}\n</div>\n</body>\n</html>\n",
        header = page_header(title, "vrtl"),
        body = body,
    )
}

/// タイトルページの XHTML ドキュメントを生成する。
pub fn render_title_xhtml(title: &str, author: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE html>\n\
<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\" xml:lang=\"ja\" class=\"hltr\">\n\
<head>\n<meta charset=\"UTF-8\"/>\n<title>{t}</title>\n<link rel=\"stylesheet\" type=\"text/css\" href=\"{css}\"/>\n</head>\n\
<body class=\"p-titlepage\">\n<div class=\"main\">\n<div class=\"book-title\"><p>{t}</p></div>\n<div class=\"author\"><p>{a}</p></div>\n</div>\n</body>\n</html>\n",
        t = escape_text(title),
        a = escape_text(author),
        css = STYLE_HREF,
    )
}

/// 目次(nav) XHTML を生成する。
pub fn render_nav_xhtml(title: &str, entries: &[NavEntry]) -> String {
    let mut items = String::new();
    for e in entries {
        items.push_str(&format!(
            "\t\t<li><a href=\"{href}\">{label}</a></li>\n",
            href = escape_attr(&e.href),
            label = escape_text(&e.label),
        ));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE html>\n\
<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\" lang=\"ja\" xml:lang=\"ja\">\n\
<head>\n<meta charset=\"UTF-8\"/>\n<title>{t}</title>\n\
<style type=\"text/css\">\nhtml {{ writing-mode:horizontal-tb; }}\n</style>\n</head>\n\
<body>\n\
\t<nav epub:type=\"toc\" id=\"toc\">\n\t\t<h1>目　次</h1>\n\t\t<ol>\n{items}\t\t</ol>\n\t</nav>\n\
</body>\n</html>\n",
        t = escape_text(title),
        items = items,
    )
}

fn page_header(title: &str, html_class: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE html>\n\
<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\" xml:lang=\"ja\" class=\"{cls}\">\n\
<head>\n<meta charset=\"UTF-8\"/>\n<title>{t}</title>\n<link rel=\"stylesheet\" type=\"text/css\" href=\"{css}\"/>\n</head>\n\
<body>\n<div class=\"main\">\n",
        cls = html_class,
        t = escape_text(title),
        css = STYLE_HREF,
    )
}

fn render_page_body(page: &Page, illust: &IllustMap) -> String {
    let mut out = String::new();
    let mut open_div_depth = 0u32;

    for block in &page.blocks {
        match block {
            Block::Blank => out.push_str("<p><br/></p>\n"),
            Block::Hr => out.push_str("<hr/>\n"),
            Block::PageCenter => {} // 縦中央指定は v1 では描画上 no-op。
            Block::RunningHead(text) => {
                out.push_str(&format!(
                    "<div class=\"running_head\">{}</div>\n",
                    render_inline(text)
                ));
            }
            Block::Heading { level, text } => {
                out.push_str(&render_heading(level, text));
            }
            Block::OpenDiv(kind) => {
                out.push_str(&format!("<div class=\"{}\">\n", div_class(kind)));
                open_div_depth += 1;
            }
            Block::CloseDiv(_) => {
                if open_div_depth > 0 {
                    out.push_str("</div>\n");
                    open_div_depth -= 1;
                }
            }
            Block::Line(text) => {
                // 行全体が挿絵注記なら、ブロックレベルの <p class="illust"> を直接出力する
                // （通常行の <p> 包みと二重にならないようにする）。
                if let Some(frag) = render_illust_line(text, illust) {
                    out.push_str(&frag);
                } else {
                    out.push_str(&format!(
                        "<p>{}</p>\n",
                        render_inline_ctx(text, Some(illust))
                    ));
                }
            }
        }
    }

    // 未閉じの div を閉じる（フェイルセーフ）。
    for _ in 0..open_div_depth {
        out.push_str("</div>\n");
    }
    out
}

/// 行全体が単独の挿絵注記なら、ブロックレベルの `<p class="illust">` 断片を返す。
/// 解決できない（画像不在・埋め込み無効）場合も挿絵行として `Some("")` を返し、
/// 呼び出し側が空の `<p>` を出さない／二重に包まないようにする。
fn render_illust_line(text: &str, illust: &IllustMap) -> Option<String> {
    let path = text
        .trim()
        .strip_prefix("［＃挿絵（")?
        .strip_suffix("）入る］")?;
    match illust.get(path) {
        Some(href) => Some(format!(
            "<p class=\"illust\"><img src=\"{}\" alt=\"\"/></p>\n",
            escape_attr(href)
        )),
        None => Some(String::new()),
    }
}

fn render_heading(level: &HeadingLevel, text: &str) -> String {
    let inner = render_inline(text);
    match level {
        HeadingLevel::Large => {
            format!("<div class=\"mt3\"><h1 class=\"oo-midashi\">{inner}</h1></div>\n")
        }
        HeadingLevel::Medium => {
            format!("<div class=\"mt3\"><h2 class=\"naka-midashi\">{inner}</h2></div>\n")
        }
        HeadingLevel::Sub => format!("<h3 class=\"ko-midashi\">{inner}</h3>\n"),
    }
}

fn div_class(kind: &DivKind) -> &'static str {
    match kind {
        DivKind::Introduction => "introduction",
        DivKind::Afterword => "afterword",
        DivKind::Jizuke => "jizuke",
        DivKind::Indent(_) => "indent",
        DivKind::Small => "small",
    }
}

// --- 文字列ユーティリティ ---

fn escape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        push_escaped(&mut out, c);
    }
    out
}

fn escape_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

fn push_escaped(out: &mut String, c: char) {
    match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        _ => out.push(c),
    }
}

/// `<a href="URL">TEXT</a>` を健全な XHTML アンカーへ整形する。
fn sanitize_anchor(frag: &str) -> String {
    if let Some(rest) = frag.strip_prefix("<a href=\"") {
        if let Some(qpos) = rest.find('"') {
            let href = &rest[..qpos];
            if let Some(after) = rest[qpos..].strip_prefix("\">") {
                if let Some(text) = after.strip_suffix("</a>") {
                    return format!(
                        "<a href=\"{}\">{}</a>",
                        escape_attr(href),
                        escape_text(text)
                    );
                }
            }
        }
    }
    escape_text(frag)
}

fn starts_with(chars: &[char], at: usize, pat: &str) -> bool {
    // 文字走査ループから毎文字呼ばれるため、Vec 確保を避けてイテレータ比較する。
    pat.chars().enumerate().all(|(i, c)| chars.get(at + i) == Some(&c))
}

fn find_char(chars: &[char], from: usize, target: char) -> Option<usize> {
    (from..chars.len()).find(|&i| chars[i] == target)
}

fn find_seq(chars: &[char], from: usize, seq: &[char]) -> Option<usize> {
    if seq.is_empty() || from > chars.len() {
        return None;
    }
    (from..=chars.len().saturating_sub(seq.len())).find(|&i| chars[i..i + seq.len()] == seq[..])
}

fn find_sub(chars: &[char], from: usize, pat: &str) -> Option<usize> {
    let seq: Vec<char> = pat.chars().collect();
    find_seq(chars, from, &seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::epub::parser::parse_document;

    #[test]
    fn renders_ruby() {
        assert_eq!(
            render_inline("｜親文字《おやもじ》です"),
            "<ruby>親文字<rt>おやもじ</rt></ruby>です"
        );
    }

    #[test]
    fn renders_tcy_and_bouten() {
        assert_eq!(
            render_inline("［＃縦中横］10［＃縦中横終わり］話"),
            "<span class=\"tcy\">10</span>話"
        );
        assert_eq!(
            render_inline("［＃傍点］強調［＃傍点終わり］"),
            "<em class=\"em-sesame\">強調</em>"
        );
    }

    #[test]
    fn resolves_gaiji_inline() {
        assert_eq!(render_inline("本文※［＃米印、1-2-8］"), "本文\u{203B}");
        assert_eq!(
            render_inline("※［＃始め二重山括弧］注※［＃終わり二重山括弧］"),
            "\u{300A}注\u{300B}"
        );
    }

    #[test]
    fn escapes_xml_specials_but_keeps_anchor() {
        assert_eq!(render_inline("a<b>&c"), "a&lt;b&gt;&amp;c");
        assert_eq!(
            render_inline("<a href=\"https://x/?a=1&b=2\">https://x/?a=1&b=2</a>"),
            "<a href=\"https://x/?a=1&amp;b=2\">https://x/?a=1&amp;b=2</a>"
        );
    }

    #[test]
    fn unknown_annotation_is_dropped_safely() {
        assert_eq!(render_inline("前［＃謎の注記］後"), "前後");
    }

    #[test]
    fn renders_full_page_structure() {
        let text = "題\n著\n本文\n\n［＃区切り線］\n";
        let doc = parse_document(text);
        let xhtml = render_page_xhtml(&doc.pages[0], &doc.title, &IllustMap::new());
        assert!(xhtml.contains("class=\"vrtl\""));
        assert!(xhtml.contains("<p>本文</p>"));
        assert!(xhtml.contains("<p><br/></p>"));
        assert!(xhtml.contains("<hr/>"));
    }
}
