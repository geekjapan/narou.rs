//! EPUB3 パッケージング: OPF / container.xml の生成と、ZIP 書き出し。
//!
//! - `mimetype` は無圧縮(stored)で先頭に格納する。
//! - OPF パスは `item/standard.opf` 固定（既存 `add_dc_subject_to_epub` 後処理互換）。
//! - `dc:identifier` は `sha2` 由来の決定論的 `urn:uuid:`。

use std::io::Write;
use std::path::Path;

use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::error::{NarouError, Result};

/// OPF の zip 内パス。
pub const OPF_ZIP_PATH: &str = "item/standard.opf";

/// manifest / spine の 1 項目。
pub struct ManifestItem {
    pub id: String,
    /// OPF(item/) から見た相対 href（例: `xhtml/0001.xhtml`）。
    pub href: String,
    pub media_type: String,
    pub properties: Option<String>,
    pub in_spine: bool,
    /// zip 内のフルパス（例: `item/xhtml/0001.xhtml`）。
    pub zip_path: String,
    pub data: Vec<u8>,
}

/// OPF メタデータ。
pub struct OpfMeta {
    pub title: String,
    pub author: String,
    pub identifier: String, // urn:uuid:...
    pub modified: String,   // YYYY-MM-DDTHH:MM:SSZ
}

/// container.xml（固定）。
pub fn container_xml() -> &'static str {
    "<?xml version=\"1.0\"?>\n\
<container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\">\n\
\t<rootfiles>\n\
\t\t<rootfile full-path=\"item/standard.opf\" media-type=\"application/oebps-package+xml\"/>\n\
\t</rootfiles>\n\
</container>\n"
}

/// OPF を生成する。`vertical` が false なら綴じ方向を ltr にする（横書き）。
pub fn build_opf(meta: &OpfMeta, items: &[ManifestItem], vertical: bool) -> String {
    let mut manifest = String::new();
    for item in items {
        let props = item
            .properties
            .as_ref()
            .map(|p| format!(" properties=\"{}\"", p))
            .unwrap_or_default();
        manifest.push_str(&format!(
            "\t\t<item id=\"{id}\" href=\"{href}\" media-type=\"{mt}\"{props}/>\n",
            id = escape_xml(&item.id),
            href = escape_xml(&item.href),
            mt = escape_xml(&item.media_type),
            props = props,
        ));
    }

    let mut spine = String::new();
    for item in items.iter().filter(|i| i.in_spine) {
        spine.push_str(&format!(
            "\t\t<itemref idref=\"{id}\" linear=\"yes\"/>\n",
            id = escape_xml(&item.id)
        ));
    }

    // dc:creator は空なら省略（空要素は EPUB3 で不正）。
    let creator = if meta.author.trim().is_empty() {
        String::new()
    } else {
        format!(
            "\t\t<dc:creator id=\"creator01\">{}</dc:creator>\n",
            escape_xml(&meta.author)
        )
    };

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" xml:lang=\"ja\" unique-identifier=\"unique-id\">\n\
\t<metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n\
\t\t<dc:title id=\"title\">{title}</dc:title>\n\
{creator}\
\t\t<dc:language>ja</dc:language>\n\
\t\t<dc:identifier id=\"unique-id\">{ident}</dc:identifier>\n\
\t\t<meta property=\"dcterms:modified\">{modified}</meta>\n\
\t</metadata>\n\
\t<manifest>\n{manifest}\t</manifest>\n\
\t<spine page-progression-direction=\"{ppd}\">\n{spine}\t</spine>\n\
</package>\n",
        title = escape_xml(&meta.title),
        creator = creator,
        ident = escape_xml(&meta.identifier),
        modified = escape_xml(&meta.modified),
        ppd = if vertical { "rtl" } else { "ltr" },
        manifest = manifest,
        spine = spine,
    )
}

/// EPUB を ZIP として書き出す。`mimetype` を先頭・無圧縮で格納し、残りは Deflate。
pub fn write_epub(
    output_path: &Path,
    container: &str,
    opf: &str,
    items: &[ManifestItem],
) -> Result<()> {
    if output_path.exists() {
        std::fs::remove_file(output_path)?;
    }
    let file = std::fs::File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    let deflated = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let map_err = |e: zip::result::ZipError| NarouError::Conversion(format!("EPUB ZIP error: {e}"));

    zip.start_file("mimetype", stored).map_err(map_err)?;
    zip.write_all(b"application/epub+zip")?;

    zip.start_file("META-INF/container.xml", deflated)
        .map_err(map_err)?;
    zip.write_all(container.as_bytes())?;

    zip.start_file(OPF_ZIP_PATH, deflated).map_err(map_err)?;
    zip.write_all(opf.as_bytes())?;

    for item in items {
        zip.start_file(&item.zip_path, deflated).map_err(map_err)?;
        zip.write_all(&item.data)?;
    }

    zip.finish().map_err(map_err)?;
    Ok(())
}

/// シードから決定論的な UUID(v5 風) を生成し `urn:uuid:` 形式で返す。
pub fn deterministic_urn_uuid(seed: &str) -> String {
    let digest = Sha256::digest(seed.as_bytes());
    let mut b = [0u8; 16];
    b.copy_from_slice(&digest[..16]);
    // version 5, variant RFC4122。
    b[6] = (b[6] & 0x0f) | 0x50;
    b[8] = (b[8] & 0x3f) | 0x80;
    format!(
        "urn:uuid:{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13],
        b[14], b[15],
    )
}

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_is_deterministic_and_well_formed() {
        let a = deterministic_urn_uuid("https://ncode.syosetu.com/n0500hx/");
        let b = deterministic_urn_uuid("https://ncode.syosetu.com/n0500hx/");
        assert_eq!(a, b);
        assert!(a.starts_with("urn:uuid:"));
        // version nibble = 5
        assert_eq!(&a[23..24], "5");
    }

    #[test]
    fn opf_has_version3_metadata_close_and_rtl() {
        let meta = OpfMeta {
            title: "題".into(),
            author: "著".into(),
            identifier: "urn:uuid:x".into(),
            modified: "2026-01-01T00:00:00Z".into(),
        };
        let items = vec![ManifestItem {
            id: "sec0001".into(),
            href: "xhtml/0001.xhtml".into(),
            media_type: "application/xhtml+xml".into(),
            properties: None,
            in_spine: true,
            zip_path: "item/xhtml/0001.xhtml".into(),
            data: Vec::new(),
        }];
        let opf = build_opf(&meta, &items, true);
        assert!(opf.contains("version=\"3.0\""));
        assert!(opf.contains("</metadata>"));
        assert!(opf.contains("page-progression-direction=\"rtl\""));
        assert!(opf.contains("<itemref idref=\"sec0001\""));

        let opf_h = build_opf(&meta, &items, false);
        assert!(opf_h.contains("page-progression-direction=\"ltr\""));
    }
}
