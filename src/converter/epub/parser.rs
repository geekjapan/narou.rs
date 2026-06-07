//! 青空文庫形式中間テキストを、EPUB 生成用の中間表現(AST/イベント列)へ解析する。
//!
//! 入力は `render_novel_text` が出力する中間テキスト（AozoraEpub3 に渡しているものと同一）。
//! 1 行目=タイトル、2 行目=著者、以降を `［＃改ページ］` 境界でページに分割する。

/// 見出しの階層。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadingLevel {
    /// 大見出し（章）。
    Large,
    /// 中見出し（話タイトル）。
    Medium,
    /// 1 段階大きな文字（小章）。
    Sub,
}

/// ブロック範囲の種別（開始/終了でネストする）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DivKind {
    /// 前書き。
    Introduction,
    /// 後書き。
    Afterword,
    /// 地付き。
    Jizuke,
    /// N 字下げ。
    Indent(u32),
    /// 2 段階小さな文字。
    Small,
}

/// ページ内の要素（イベント列）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// 本文 1 行（インライン注記は描画時に処理）。
    Line(String),
    /// 空行。
    Blank,
    /// 区切り線。
    Hr,
    /// 見出し。
    Heading { level: HeadingLevel, text: String },
    /// 柱（running head）。
    RunningHead(String),
    /// ページ左右中央指定（章ページ）。
    PageCenter,
    /// ブロック範囲の開始。
    OpenDiv(DivKind),
    /// ブロック範囲の終了。
    CloseDiv(DivKind),
}

/// 1 ページ分（本文 XHTML 1 ファイルに対応）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub blocks: Vec<Block>,
    /// 目次ラベル（無ければ呼び出し側で代替）。
    pub nav_label: Option<String>,
}

/// 解析済みドキュメント。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub title: String,
    pub author: String,
    pub pages: Vec<Page>,
}

const PAGE_BREAK: &str = "［＃改ページ］";

/// 中間テキストを `Document` へ解析する。
pub fn parse_document(text: &str) -> Document {
    let mut lines = text.lines();
    let title = lines.next().unwrap_or("").to_string();
    let author = lines.next().unwrap_or("").to_string();

    // 残りを改ページで分割。
    let rest: Vec<&str> = lines.collect();
    let mut page_line_groups: Vec<Vec<&str>> = vec![Vec::new()];
    for line in rest {
        if line.trim() == PAGE_BREAK {
            page_line_groups.push(Vec::new());
            continue;
        }
        page_line_groups
            .last_mut()
            .expect("at least one group exists")
            .push(line);
    }

    let mut pages = Vec::new();
    for group in page_line_groups {
        // 空ページ（連続改ページ等）はスキップ。
        if group.iter().all(|l| l.trim().is_empty()) {
            continue;
        }
        let blocks = parse_lines(&group);
        let nav_label = derive_nav_label(&blocks);
        pages.push(Page { blocks, nav_label });
    }

    Document {
        title,
        author,
        pages,
    }
}

/// ページの見出しから目次ラベルを導出（中見出し > 大見出し > 1 段階大きな文字）。
fn derive_nav_label(blocks: &[Block]) -> Option<String> {
    let pick = |want: &HeadingLevel| {
        blocks.iter().find_map(|b| match b {
            Block::Heading { level, text } if level == want => Some(text.clone()),
            _ => None,
        })
    };
    pick(&HeadingLevel::Medium)
        .or_else(|| pick(&HeadingLevel::Large))
        .or_else(|| pick(&HeadingLevel::Sub))
}

fn parse_lines(lines: &[&str]) -> Vec<Block> {
    let mut blocks = Vec::new();
    for &line in lines {
        parse_line_into(line, &mut blocks);
    }
    blocks
}

fn parse_line_into(line: &str, out: &mut Vec<Block>) {
    let trimmed = line.trim_end();

    if trimmed.is_empty() {
        out.push(Block::Blank);
        return;
    }
    if trimmed == "［＃区切り線］" {
        out.push(Block::Hr);
        return;
    }
    if trimmed == "［＃ページの左右中央］" {
        out.push(Block::PageCenter);
        return;
    }

    // 単独のブロック範囲マーカー。
    if let Some(div) = match_open_div(trimmed) {
        out.push(Block::OpenDiv(div));
        return;
    }
    if let Some(div) = match_close_div(trimmed) {
        out.push(Block::CloseDiv(div));
        return;
    }

    // 地付き開始と終了が 1 行に同居（巻末メッセージ等）。
    if trimmed.contains("［＃ここから地付き］") && trimmed.contains("［＃ここで地付き終わり］") {
        let inner = trimmed
            .replace("［＃ここから地付き］", "")
            .replace("［＃ここで地付き終わり］", "");
        out.push(Block::OpenDiv(DivKind::Jizuke));
        if !inner.trim().is_empty() {
            out.push(Block::Line(inner));
        }
        out.push(Block::CloseDiv(DivKind::Jizuke));
        return;
    }

    // 字下げ系の行頭プレフィックスを除去してから見出し判定。
    let body = strip_indent_prefix(trimmed);

    // 柱。
    if let Some(inner) = between(body, "［＃ここから柱］", "［＃ここで柱終わり］") {
        out.push(Block::RunningHead(inner.to_string()));
        return;
    }
    // 見出し。
    if let Some(inner) = between(body, "［＃大見出し］", "［＃大見出し終わり］") {
        out.push(Block::Heading {
            level: HeadingLevel::Large,
            text: inner.to_string(),
        });
        return;
    }
    if let Some(inner) = between(body, "［＃中見出し］", "［＃中見出し終わり］") {
        out.push(Block::Heading {
            level: HeadingLevel::Medium,
            text: inner.to_string(),
        });
        return;
    }
    if let Some(inner) = between(body, "［＃１段階大きな文字］", "［＃大きな文字終わり］") {
        out.push(Block::Heading {
            level: HeadingLevel::Sub,
            text: inner.to_string(),
        });
        return;
    }

    out.push(Block::Line(trimmed.to_string()));
}

/// 行頭の `［＃N字下げ］` 系プレフィックスを 1 つ除去する。
fn strip_indent_prefix(line: &str) -> &str {
    if let Some(rest) = line.strip_prefix("［＃") {
        if let Some(end) = rest.find('］') {
            let tag = &rest[..end];
            if tag.ends_with("字下げ") && !tag.starts_with("ここ") {
                return &rest[end + '］'.len_utf8()..];
            }
        }
    }
    line
}

fn match_open_div(line: &str) -> Option<DivKind> {
    match line {
        "［＃ここから前書き］" => Some(DivKind::Introduction),
        "［＃ここから後書き］" => Some(DivKind::Afterword),
        "［＃ここから地付き］" => Some(DivKind::Jizuke),
        "［＃ここから２段階小さな文字］" => Some(DivKind::Small),
        _ => {
            if let Some(inner) = between(line, "［＃ここから", "字下げ］") {
                let n = parse_kanji_or_ascii_number(inner).unwrap_or(0);
                return Some(DivKind::Indent(n));
            }
            None
        }
    }
}

fn match_close_div(line: &str) -> Option<DivKind> {
    match line {
        "［＃ここで前書き終わり］" => Some(DivKind::Introduction),
        "［＃ここで後書き終わり］" => Some(DivKind::Afterword),
        "［＃ここで地付き終わり］" => Some(DivKind::Jizuke),
        "［＃ここで小さな文字終わり］" => Some(DivKind::Small),
        "［＃ここで字下げ終わり］" => Some(DivKind::Indent(0)),
        _ => None,
    }
}

/// `open` と `close` に挟まれた内側を返す（行全体がその形のときのみ）。
fn between<'a>(line: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(open)?;
    rest.strip_suffix(close)
}

/// 全角/半角数字を含む短い数値表現を解釈する（字下げ段数など）。
fn parse_kanji_or_ascii_number(s: &str) -> Option<u32> {
    let mut value = 0u32;
    let mut any = false;
    for c in s.chars() {
        let d = match c {
            '0'..='9' => c as u32 - '0' as u32,
            '０'..='９' => c as u32 - '０' as u32,
            _ => return if any { Some(value) } else { None },
        };
        value = value * 10 + d;
        any = true;
    }
    if any { Some(value) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_title_author_and_pages() {
        let text = "作品タイトル\n著者名\n本文先頭\n［＃改ページ］\n本文二ページ目\n";
        let doc = parse_document(text);
        assert_eq!(doc.title, "作品タイトル");
        assert_eq!(doc.author, "著者名");
        assert_eq!(doc.pages.len(), 2);
        assert_eq!(doc.pages[0].blocks, vec![Block::Line("本文先頭".into())]);
        assert_eq!(doc.pages[1].blocks, vec![Block::Line("本文二ページ目".into())]);
    }

    #[test]
    fn parses_blank_and_hr() {
        let text = "t\na\n\n［＃区切り線］\n";
        let doc = parse_document(text);
        assert_eq!(doc.pages[0].blocks, vec![Block::Blank, Block::Hr]);
    }

    #[test]
    fn parses_headings_with_indent_prefix() {
        let text = "t\na\n［＃３字下げ］［＃大見出し］第一章［＃大見出し終わり］\n［＃３字下げ］［＃中見出し］第一話［＃中見出し終わり］\n";
        let doc = parse_document(text);
        assert_eq!(
            doc.pages[0].blocks[0],
            Block::Heading {
                level: HeadingLevel::Large,
                text: "第一章".into()
            }
        );
        assert_eq!(
            doc.pages[0].blocks[1],
            Block::Heading {
                level: HeadingLevel::Medium,
                text: "第一話".into()
            }
        );
        assert_eq!(doc.pages[0].nav_label.as_deref(), Some("第一話"));
    }

    #[test]
    fn parses_introduction_block() {
        let text = "t\na\n［＃ここから前書き］\n前書き本文\n［＃ここで前書き終わり］\n";
        let doc = parse_document(text);
        assert_eq!(
            doc.pages[0].blocks,
            vec![
                Block::OpenDiv(DivKind::Introduction),
                Block::Line("前書き本文".into()),
                Block::CloseDiv(DivKind::Introduction),
            ]
        );
    }

    #[test]
    fn parses_inline_jizuke_line() {
        let text = "t\na\n［＃ここから地付き］（本を読み終わりました）［＃ここで地付き終わり］\n";
        let doc = parse_document(text);
        assert_eq!(
            doc.pages[0].blocks,
            vec![
                Block::OpenDiv(DivKind::Jizuke),
                Block::Line("（本を読み終わりました）".into()),
                Block::CloseDiv(DivKind::Jizuke),
            ]
        );
    }
}
