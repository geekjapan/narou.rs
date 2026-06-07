use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::error::Result;

use super::site_setting::SiteSetting;

pub struct NovelInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub story: Option<String>,
    pub novel_type: Option<u8>,
    pub end: Option<bool>,
    pub general_firstup: Option<DateTime<Utc>>,
    pub general_lastup: Option<DateTime<Utc>>,
    pub novelupdated_at: Option<DateTime<Utc>>,
    pub length: Option<i64>,
    pub tags: Option<String>,
    pub sitename: Option<String>,
    pub raw_captures: HashMap<String, String>,
}

impl NovelInfo {
    fn empty() -> Self {
        Self {
            title: None,
            author: None,
            story: None,
            novel_type: None,
            end: None,
            general_firstup: None,
            general_lastup: None,
            novelupdated_at: None,
            length: None,
            tags: None,
            sitename: None,
            raw_captures: HashMap::new(),
        }
    }

    pub fn load(
        setting: &SiteSetting,
        client: &reqwest::blocking::Client,
        toc_source: &str,
        url_captures: &HashMap<String, String>,
    ) -> Result<Self> {
        if let Some(novel_info_url) = &setting.novel_info_url {
            let resolved_url = setting
                .novel_info_url_with_captures(url_captures)
                .unwrap_or_else(|| setting.interpolate(novel_info_url));
            let response = client.get(&resolved_url).send()?;
            if !response.status().is_success() {
                return Ok(Self::empty());
            }
            let mut body = response.text()?;
            crate::downloader::pretreatment_source(&mut body, setting.encoding(), Some(setting));

            Ok(Self::from_novel_info_source(setting, &body))
        } else {
            Ok(Self::from_toc_source(setting, toc_source))
        }
    }

    pub fn from_novel_info_source(setting: &SiteSetting, source: &str) -> Self {
        let mut info = Self::empty();
        let keys = [
            "t", "w", "s", "nt", "ga", "gf", "nu", "gl", "l", "tags", "sitename",
        ];
        info.raw_captures = setting.multi_match(source, &keys);
        if info.raw_captures.is_empty() {
            return info;
        }

        info.title = info.raw_captures.get("t").cloned();
        info.author = info.raw_captures.get("w").cloned();
        info.story = info.raw_captures.get("s").cloned();
        info.tags = info.raw_captures.get("tags").cloned();
        info.sitename = info.raw_captures.get("sitename").cloned();

        if let Some(nt_text) = info.raw_captures.get("nt") {
            let (novel_type, is_end) = setting.get_novel_type_from_string(nt_text);
            info.novel_type = Some(novel_type);
            info.end = Some(is_end);
        }

        let timezone = setting.site_timezone();

        info.general_firstup = info
            .raw_captures
            .get("gf")
            .and_then(|s| parse_narou_date_with_timezone(s, timezone));
        info.general_lastup = info
            .raw_captures
            .get("gl")
            .and_then(|s| parse_narou_date_with_timezone(s, timezone));
        info.novelupdated_at = info
            .raw_captures
            .get("nu")
            .and_then(|s| parse_narou_date_with_timezone(s, timezone));
        info.length = info.raw_captures.get("l").and_then(|s| {
            s.replace(',', "").trim().parse().ok()
        });

        info
    }

    /// Fill empty/missing core display fields from a fallback source (e.g. the
    /// TOC page) without overwriting any value already extracted from the
    /// primary novel_info page.
    pub fn fill_missing_from(&mut self, fallback: NovelInfo) {
        let fill = |target: &mut Option<String>, source: Option<String>| {
            if target.as_deref().is_none_or(str::is_empty) {
                if let Some(v) = source.filter(|s| !s.is_empty()) {
                    *target = Some(v);
                }
            }
        };
        fill(&mut self.title, fallback.title);
        fill(&mut self.author, fallback.author);
        fill(&mut self.story, fallback.story);
        fill(&mut self.tags, fallback.tags);
    }

    /// Whether any core display field (title/author/story/tags) is missing and
    /// could be recovered from a fallback source.
    pub fn has_missing_core_fields(&self) -> bool {
        [&self.title, &self.author, &self.story, &self.tags]
            .iter()
            .any(|field| field.as_deref().is_none_or(str::is_empty))
    }

    pub fn from_toc_source(setting: &SiteSetting, toc_source: &str) -> Self {
        let mut info = Self::empty();
        let keys = ["title", "author", "story", "tags"];
        info.raw_captures = setting.multi_match(toc_source, &keys);
        info.title = info.raw_captures.get("title").cloned();
        info.author = info.raw_captures.get("author").cloned();
        info.story = info.raw_captures.get("story").cloned();
        info.tags = info.raw_captures.get("tags").cloned();
        info
    }
}

#[cfg(test)]
fn parse_narou_date(s: &str) -> Option<DateTime<Utc>> {
    parse_narou_date_with_timezone(s, super::site_timezone(None))
}

fn parse_narou_date_with_timezone(
    s: &str,
    timezone: super::SiteTimezone,
) -> Option<DateTime<Utc>> {
    super::parse_loose_datetime_with_timezone(s, timezone)
}

#[cfg(test)]
mod tests {
    use super::parse_narou_date;
    use super::NovelInfo;
    use crate::downloader::site_setting::SiteSetting;
    use chrono::{Datelike, Timelike};

    #[test]
    fn syosetu_org_title_falls_back_to_toc_when_detail_page_blocked() {
        let settings = SiteSetting::load_all().unwrap();
        let setting = settings
            .iter()
            .find(|s| s.domain == "syosetu.org")
            .unwrap();

        // Anti-bot interstitial served with HTTP 200: none of the `t:` / `w:`
        // novel_info patterns match, so from_novel_info_source yields nothing.
        let challenge = "<!DOCTYPE html><html><head><title>Just a moment...</title>\
            </head><body>checking your browser</body></html>";
        let detail_info = NovelInfo::from_novel_info_source(setting, challenge);
        assert!(detail_info.title.as_deref().unwrap_or("").is_empty());

        // The TOC page (reliably fetched during body DL) carries the title/author
        // via the `title:` / `author:` patterns.
        let toc = "<br>\n\
            <span style=\"font-size:150%\" itemprop=\"name\">テスト小説</span>\n\
            <div align=\"right\">作者：<span itemprop=\"author\">著者名</span></div>";

        let mut info = detail_info;
        info.fill_missing_from(NovelInfo::from_toc_source(setting, toc));

        assert_eq!(info.title.as_deref(), Some("テスト小説"));
        assert_eq!(info.author.as_deref(), Some("著者名"));
    }

    #[test]
    fn fill_missing_from_does_not_overwrite_existing_values() {
        let mut info = NovelInfo::empty();
        info.title = Some("primary".to_string());
        info.author = Some(String::new());

        let mut fallback = NovelInfo::empty();
        fallback.title = Some("fallback-title".to_string());
        fallback.author = Some("fallback-author".to_string());
        fallback.story = Some("fallback-story".to_string());

        info.fill_missing_from(fallback);

        assert_eq!(info.title.as_deref(), Some("primary"));
        assert_eq!(info.author.as_deref(), Some("fallback-author"));
        assert_eq!(info.story.as_deref(), Some("fallback-story"));
    }

    #[test]
    fn partial_detail_parse_still_backfills_missing_author_from_toc() {
        // The detail page yielded a title but not the author (e.g. markup change
        // or a user-edited YAML pattern that only breaks author extraction).
        // has_missing_core_fields must still trigger so the TOC author is kept.
        let mut info = NovelInfo::empty();
        info.title = Some("詳細ページのタイトル".to_string());

        assert!(info.has_missing_core_fields());

        let mut toc = NovelInfo::empty();
        toc.title = Some("TOCタイトル".to_string());
        toc.author = Some("TOC著者".to_string());

        info.fill_missing_from(toc);

        assert_eq!(info.title.as_deref(), Some("詳細ページのタイトル"));
        assert_eq!(info.author.as_deref(), Some("TOC著者"));
    }

    #[test]
    fn parse_narou_date_accepts_kakuyomu_rfc3339() {
        let date = parse_narou_date("2021-01-10T16:13:02Z").expect("date");

        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 10);
        assert_eq!(date.hour(), 16);
        assert_eq!(date.minute(), 13);
        assert_eq!(date.second(), 2);
    }

    #[test]
    fn parse_narou_date_accepts_japanese_datetime_with_weekday() {
        let date = parse_narou_date("2026年04月17日(金) 07:00").expect("date");
        let local = date.with_timezone(&chrono_tz::Asia::Tokyo);

        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 4);
        assert_eq!(date.day(), 16);
        assert_eq!(date.hour(), 22);
        assert_eq!(date.minute(), 0);
        assert_eq!(date.second(), 0);
        assert_eq!(local.year(), 2026);
        assert_eq!(local.month(), 4);
        assert_eq!(local.day(), 17);
        assert_eq!(local.hour(), 7);
    }
}
