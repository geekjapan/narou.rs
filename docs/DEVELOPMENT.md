# narou.rs — Development Status & Notes

本ファイルは `AGENTS.md` から分離した開発状況・既知課題。日常の作業ルールは `AGENTS.md` を参照。

## Edition 2024 注意事項

- `{}`フォーマット直後に文字列を書くとprefix扱いされるためスペースが必要
- 特に `regex::Regex::new(r"...").unwrap()` の直後に `.` で始まる式を書くとコンパイルエラーになる
- セミコロンで終わらせるか変数に代入すること

## Current Status (2026-05)

### 変換互換性
- **なろう**: narou.rb参照データと完全互換確認済み
- **カクヨム (ID=1177354055617350769)**: **完全互換達成** — 行数完全一致 (25,273/25,273)、行単位 diff 0件。`cargo test` の `tests/convert_parity.rs` で byte-for-byte fixture テスト通過
- ※米印変換、全角数字、ルビ、auto_join_line、各種文字変換も完全一致
- **ネイティブ EPUB3 生成 (2026-06)**: Java/AozoraEpub3 非依存の Rust ネイティブ経路 (`src/converter/epub/`) を実装。`epub`/`reader`/`ibooks` は既定でネイティブ生成。`convert.use-aozoraepub3` で AozoraEpub3 退避、`NAROU_RS_EPUB_ENGINE` で強制切替。8作品で epubcheck 5.3.0 を 0 エラー/0 警告で通過。

### ダウンロード互換性
- なろう (n8858hb, 24セクション) DL完走確認済み
- カクヨム (ID=2, 294セクション) DL完走確認済み
- syosetu.org（ハーメルン）: UAランダム化、HTTP/1.1/Cookie/圧縮/curl fallback による403回避対応済み。フルDL未検証
- Arcadia: `href` の `&amp;` デコード修正により本文取得修正済み

### Web UI
- 全APIエンドポイント実装済み (70+)
- Pure JS/CSS frontend (JP/EN切替、テーマ切替、レスポンシブ対応)
- WebSocket プッシュ通知 (ジョブ進捗、ログストリーミング)
- 自動更新スケジューラ (queue-backed, scheduler restart without server restart)
- キュー並列実行 (concurrency 有効時: primary lane DL/update + secondary lane convert/send)
- Basic認証、Host/Origin検証、CSRF対策、reverse proxy モード
- Windows タスクトレイ常駐 (`--hide-console`)

### コマンド実装状況 (詳細は `COMMANDS.md`)
- ✅ 完了 (18): init, list, tag, freeze, remove, setting, diff, send, backup, clean, help, version, log, folder, browser, alias, inspect, csv, trace
- 🟡 部分 (5): download, update, convert, web, mail
- ❌ 未実装 (0): 全コマンド何らかの実装あり

## 未解決の既知課題

### 2026-04: WEB UI の自動更新ボタンが出ない件
- 現象: v0.1.32 で `latest_version != current_version` にも関わらず `update_available: false`
- 該当コード: `src/web/misc.rs::version_latest`
- 仮説: 不可視文字混入、キャッシュ不整合、v0.1.32 固有のコードバグ
- 再現環境は喪失 (ユーザー側アップデート済み、2026-04-26)
- `841bec5` で `NAROU_RS_RELEASE_BUILD` フラグ焼き込み済み
- 対応指針: `version_latest` 防御的書き直し、JS 側フォールバック判定、生バイト列検査
