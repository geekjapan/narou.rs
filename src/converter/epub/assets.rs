//! EPUB 同梱の静的資産（縦書き CSS・埋め込みフォント・メディアタイプ判定）。
//!
//! CSS は `preset/vertical_font*.css` を母体に、参照 EPUB 互換のクラス語彙へ集約する。
//! 単位は解像度非依存（em 等）を用いる。

/// 埋め込み用フォント（明朝）。`embed_font` 有効時のみ EPUB へ書き出す。
pub const DMINCHO_TTF: &[u8] = include_bytes!("../../../preset/DMincho.ttf");

/// EPUB 内のフォントファイル名。
pub const FONT_FILE: &str = "font/DMincho.ttf";

/// 本文 CSS を生成する。`embed_font` 有効時、または濁点フォント (`dakuten_font`)
/// 使用時は DMincho の @font-face を含める。濁点表示には DMincho の専用グリフが
/// 必須のため、`.dakuten` はフォント埋め込み時に DMincho を参照する。
pub fn book_style_css(line_height: f64, embed_font: bool, dakuten_font: bool) -> String {
    let need_font = embed_font || dakuten_font;
    let font_face = if need_font {
        format!(
            "@font-face {{\n  font-family: \"DMincho\";\n  src: url(\"../{font}\");\n}}\n",
            font = FONT_FILE
        )
    } else {
        String::new()
    };
    let body_font = if embed_font {
        "\"DMincho\", serif"
    } else {
        "serif"
    };
    // 濁点合成グリフは DMincho にしか無いため、フォント埋め込み時は .dakuten へ適用する。
    let dakuten_family = if need_font {
        "\"DMincho\", serif"
    } else {
        "serif"
    };

    format!(
        "@charset \"utf-8\";\n\
{font_face}\
html {{\n  -epub-writing-mode: vertical-rl;\n  -webkit-writing-mode: vertical-rl;\n  writing-mode: vertical-rl;\n}}\n\
html.hltr {{\n  -epub-writing-mode: horizontal-tb;\n  -webkit-writing-mode: horizontal-tb;\n  writing-mode: horizontal-tb;\n}}\n\
body {{\n  font-family: {body_font};\n  line-height: {lh}em;\n  margin: 0;\n  padding: 1em;\n}}\n\
.main {{\n  height: 100%;\n}}\n\
p {{\n  margin: 0;\n  text-indent: 0;\n  line-height: {lh}em;\n}}\n\
hr {{\n  border: none;\n  border-top: 1px solid currentColor;\n  margin: 1em 0;\n}}\n\
rt {{\n  font-size: 0.6em;\n}}\n\
.tcy {{\n  -webkit-text-combine: horizontal;\n  text-combine-upright: all;\n}}\n\
.em-sesame {{\n  font-style: normal;\n  -webkit-text-emphasis-style: sesame;\n  text-emphasis-style: sesame;\n}}\n\
.dakuten {{\n  font-family: {dakuten_family};\n}}\n\
h1.oo-midashi {{\n  font-size: 1.6em;\n  font-weight: bold;\n  margin: 0;\n}}\n\
h2.naka-midashi {{\n  font-size: 1.3em;\n  font-weight: bold;\n  margin: 0;\n}}\n\
h3.ko-midashi {{\n  font-size: 1.15em;\n  font-weight: bold;\n  margin: 0;\n}}\n\
.mt3 {{\n  margin-top: 3em;\n}}\n\
.introduction, .afterword {{\n  font-size: 0.9em;\n  margin: 1em 2em;\n}}\n\
.small {{\n  font-size: 0.8em;\n}}\n\
.indent {{\n  margin-top: 1em;\n}}\n\
.jizuke {{\n  text-align: end;\n}}\n\
.running_head {{\n  font-size: 0.8em;\n}}\n\
.book-title {{\n  font-size: 1.8em;\n  font-weight: bold;\n  text-align: center;\n  margin-top: 3em;\n}}\n\
.author {{\n  text-align: center;\n  margin-top: 2em;\n}}\n\
.clear {{\n  clear: both;\n}}\n",
        font_face = font_face,
        body_font = body_font,
        dakuten_family = dakuten_family,
        lh = line_height,
    )
}

/// 拡張子からメディアタイプを判定する。
pub fn media_type_for(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".ttf") {
        "application/font-sfnt"
    } else if lower.ends_with(".otf") {
        "application/vnd.ms-opentype"
    } else if lower.ends_with(".css") {
        "text/css"
    } else if lower.ends_with(".xhtml") {
        "application/xhtml+xml"
    } else {
        "application/octet-stream"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_contains_vertical_and_line_height() {
        let css = book_style_css(1.8, false, false);
        assert!(css.contains("writing-mode: vertical-rl"));
        assert!(css.contains("line-height: 1.8em"));
        assert!(!css.contains("@font-face"));
        assert!(css.contains(".dakuten {\n  font-family: serif;"));
    }

    #[test]
    fn css_includes_font_face_when_embedding() {
        let css = book_style_css(1.8, true, false);
        assert!(css.contains("@font-face"));
        assert!(css.contains("DMincho.ttf"));
    }

    #[test]
    fn css_embeds_dmincho_for_dakuten_without_body_embed() {
        // 濁点フォントのみ要求: @font-face は出力し、.dakuten は DMincho を参照するが、
        // 本文 body は serif のまま（フォント全体埋め込みではない）。
        let css = book_style_css(1.8, false, true);
        assert!(css.contains("@font-face"));
        assert!(css.contains(".dakuten {\n  font-family: \"DMincho\", serif;"));
        assert!(css.contains("body {\n  font-family: serif;"));
    }

    #[test]
    fn media_types() {
        assert_eq!(media_type_for("a.JPG"), "image/jpeg");
        assert_eq!(media_type_for("挿絵/1.png"), "image/png");
    }
}
