//! 外字(gaiji)注記の解決。
//!
//! 青空文庫の中間テキストに現れる外字注記を、表示可能な文字へ解決する。
//! - 名前付き外字: `※［＃米印、1-2-8］`, `※［＃始め二重山括弧］` など。
//! - 面区点指定: `［＃…、N-N-N］`。
//!
//! 方針(grill Q5): まず Unicode へマッピングし、未解決は判読可能な代替文字で出力する。
//! 外字画像フォールバックは初版スコープ外。空欄・文字化けは作らない。

/// 未解決外字の代替文字（欠字記号）。
pub const FALLBACK_CHAR: char = '〓';

/// 名前付き外字注記の内側文字列（`※［＃` と `］` の間、または `［＃` と `］` の間）を解決する。
///
/// 解決できた場合は対応文字列、できなければ `None`（呼び出し側で代替）を返す。
pub fn resolve_named(inner: &str) -> Option<String> {
    // 「米印、1-2-8」のように "、" 以降に面区点が付くことがある。名前部分で先に判定する。
    let name = inner.split('、').next().unwrap_or(inner).trim();
    let by_name = match name {
        "米印" => Some('\u{203B}'),                 // ※
        "始め二重山括弧" => Some('\u{300A}'),        // 《
        "終わり二重山括弧" => Some('\u{300B}'),      // 》
        "二の字点" => Some('\u{303B}'),              // 〻
        "ファイナルシグマ" => Some('\u{03C2}'),      // ς
        _ => None,
    };
    if let Some(c) = by_name {
        return Some(c.to_string());
    }

    // 面区点 (N-N-N) によるマッピング。
    if let Some(menku) = extract_menku(inner) {
        if let Some(c) = resolve_menku(menku) {
            return Some(c.to_string());
        }
    }
    None
}

/// 内側文字列から面区点 `(men, ku, ten)` を取り出す。
fn extract_menku(inner: &str) -> Option<(u32, u32, u32)> {
    let tail = inner.rsplit('、').next().unwrap_or(inner).trim();
    let mut parts = tail.split('-');
    let men = parts.next()?.trim().parse().ok()?;
    let ku = parts.next()?.trim().parse().ok()?;
    let ten = parts.next()?.trim().parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((men, ku, ten))
}

/// 面区点 → Unicode の対応表（頻出のみ。順次拡充）。
fn resolve_menku(menku: (u32, u32, u32)) -> Option<char> {
    let c = match menku {
        (1, 2, 8) => '\u{203B}',  // ※ 米印
        (1, 2, 22) => '\u{301C}', // 〜 波ダッシュ
        (1, 2, 54) => '\u{301A}', // 〚
        (1, 2, 55) => '\u{301B}', // 〛
        _ => return None,
    };
    Some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_named_gaiji() {
        assert_eq!(resolve_named("米印、1-2-8").as_deref(), Some("\u{203B}"));
        assert_eq!(resolve_named("始め二重山括弧").as_deref(), Some("\u{300A}"));
        assert_eq!(resolve_named("終わり二重山括弧").as_deref(), Some("\u{300B}"));
    }

    #[test]
    fn resolves_menku_only() {
        assert_eq!(resolve_named("第3水準、1-2-8").as_deref(), Some("\u{203B}"));
    }

    #[test]
    fn unknown_returns_none() {
        assert_eq!(resolve_named("謎の外字、9-99-99"), None);
        assert_eq!(resolve_named("まったく未知"), None);
    }
}
