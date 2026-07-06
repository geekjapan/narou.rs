# narou.rs — Architecture Reference

本ファイルは `AGENTS.md` から分離した詳細アーキテクチャ情報。日常の作業ルールは `AGENTS.md` を参照。

## 互換性の要件レベル

- 外部から観測できる挙動の互換性は**妥協せず完璧に**追求する。これには以下が含まれる:
  - **設定ファイルの位置**: `.narou/local_setting.yaml`、`~/.narousetting/global_setting.yaml` など、Ruby 版と同一パスに配置する。
  - **設定ファイルの読み書き互換**: Rust が書いた YAML を Ruby が読め、Ruby が書いた YAML を Rust が読めること。`---` ヘッダの有無など形式の差は許容されるが、意味論（キー名・値の型・構造）は一致させる。
  - **全設定項目の読み書き**: Rust 側に未実装の機能の設定項目であっても、`narou setting` コマンドで読み取り・設定・削除が可能であること。`default.*`、`force.*`、`default_args.*` 系の動的変数名もすべて受け付けること。
  - **CLI の引数・戻り値・エラーメッセージ・終了コード**: Ruby 版と同一であること。
  - **`webnovel/*.yaml` や `.narou/` 配下のデータ構造**: Ruby 版が読める形式を維持すること。
  - **最終的な変換出力ファイル**: narou.rb の出力と同一であること。
- 「内部実装は異なってよい」方針は変更しない。上記の外部互換性を満たす限り、Rust 側のアルゴリズム・データ構造・処理順序は自由に選んでよい。

## YAML-Driven Site Definition Compatibility

- サイト別の取得・前処理・抽出ルールは narou.rb と同じく `webnovel/*.yaml` を主たる仕様として扱う。ユーザーが初期化フォルダ内の `webnovel/*.yaml` を編集・差し替えた場合、その内容で挙動を変えられることが互換性の重要要件である。
- Rust 側にサイト固有ロジックを直接ハードコードする実装は、最終的な互換方針としては不可。
- 2026-05 時点: ハードコードされた `kakuyomu_preprocess` は完全に除去され、`webnovel/kakuyomu.jp.yaml` の `preprocess:` DSL ブロックへ移行済み。pest 文法ベースの安全な DSL パーサー (`src/downloader/preprocess.pest`) + インタプリタ (`src/downloader/preprocess/interpreter.rs`) により、YAML 記述だけでカクヨム JSON → 中間テキストの展開が可能。
- pest 文法対応構文: `guard`/`let`/`set`/`if`/`else`/`for`/`emit`/`insert_at_match`, 文字列補間 `${...}`, 正規表現 JSON 抽出 `extract_json(/.../)`, メソッドチェイン `.map`/`.flat_map`/`.flatten`/`.compact`/`.join`/`.gsub`/`.replace`/`.is_array`/`.empty`, 論理演算 `&&`/`||`/`!`/`==`/`!=`。実行時に step budget / 文字列サイズ上限 / 配列要素数上限による防御あり。
- 新しいサイト対応では、まず YAML 表現で解決できるかを検討する。やむを得ず Rust に暫定処理を置く場合は、暫定であること、対応する YAML 意味論、将来 YAML 駆動へ戻す作業を明記する。
- Arcadia (`webnovel/www.mai-net.net.yaml`) に `encoding: UTF-8` は置かない。narou.rb の同梱 Arcadia 定義には無く、Rust 側は UTF-8 を既定として扱う。

## Init / Local Data Compatibility

- `narou init` は narou.rb の `Command::Init` / `Narou.init` / `Inventory` を参照して実装する。
- 新規初期化では `.narou/`、`小説データ/`、ユーザー編集用の `webnovel/` を作成し、同梱 `webnovel/*.yaml` を初期コピーする。
- `.narou/` 配下の `local_setting.yaml`、`database.yaml`、`database_index.yaml`、`alias.yaml`、`freeze.yaml`、`tag_colors.yaml`、`latest_convert.yaml`、`queue.yaml`、`notepad.txt` は narou.rb の Inventory 互換ファイルとして扱う。
- `local_setting.yaml` は Ruby 版と同じく任意設定の置き場であり、初期化時に大量のデフォルト値を書き込まない。
- `narou init -p/--path` は指定先に `AozoraEpub3.jar` がある場合だけ `~/.narousetting/global_setting.yaml` に保存する。`-p :keep` は既存の有効な `aozoraepub3dir` を再利用する。
- `narou init -l/--line-height` は AozoraEpub3 設定が保存される場合だけ保存し、未指定時は `1.8` を使う。

## Project Structure

```
src/
  main.rs                          - CLI entry point (thin dispatcher)
  cli.rs                           - clap定義 (Cli struct + Commands enum, 引数前処理)
  error.rs                         - NarouError enum + Result type
  queue.rs                         - PersistentQueue (YAMLベース永続化ジョブキュー)
  lib.rs                           - クレートルート (pub mod定義)
  commands/
    mod.rs                         - pub mod + resolve_target_to_id, resolve_alias_target
    init.rs                        - narou init (ディレクトリ作成, AozoraEpub3設定)
    download.rs                    - narou download
    update.rs                      - narou update
    convert.rs                     - narou convert
    web.rs                         - narou web (Axumサーバー起動)
    list.rs/manage.rs              - narou list (manage.rs に tag/freeze/remove も同居)
    tag.rs, freeze.rs, remove.rs   - (manage.rs 内に統合)
    setting.rs                     - narou setting
    diff.rs, send.rs, mail.rs      - diff / send / mail
    backup.rs, clean.rs            - backup / clean
    help.rs, version.rs            - help / version
    log.rs, trace.rs               - log / trace
    alias.rs, folder.rs, browser.rs - alias / folder / browser
    inspect.rs, csv.rs             - inspect / csv
    web_tray.rs                    - Windows タスクトレイ
  db/
    mod.rs                         - シングルトン (DATABASE static, init_database, with_database/mut)
    database.rs                    - Database struct (CRUD, sort, tag index)
    novel_record.rs                - NovelRecord struct (45フィールド, nilable bool対応)
    inventory.rs                   - Inventory (LRU cache, atomic write, Windows retry)
    index_store.rs                 - IndexStore (SHA256 fingerprint)
    paths.rs                       - novel_dir_for_record, create_subdirectory_name
    ruby_time.rs                   - Ruby互換日時フォーマット
  downloader/
    mod.rs                         - Downloader struct (DL pipeline orchestrator)
    types.rs                       - SectionElement, SectionFile, TocObject, DownloadResult 等
    fetch.rs                       - HttpFetcher (3-tier: curl crate → reqwest → wget fallback)
    toc.rs                         - fetch_toc, parse_subtitles, parse_subtitles_multipage
    section.rs                     - download_section, parse_section_html, section cache
    persistence.rs                 - save_section_file, save_raw_file, save_toc_file, ensure_default_files
    narou_api.rs                   - narou_api_batch_update (なろうAPI一括更新)
    util.rs                        - build_section_url, pretreatment_source, sanitize_filename 等
    site_setting/
      mod.rs                       - SiteSetting struct, accessor methods, compile, load_all, tests
      interpolate.rs               - \k<name> テンプレートエンジン
      info_extraction.rs           - resolve_info_pattern, multi_match, get_novel_type_from_string
      loader.rs                    - load_all_from_dirs, load_settings_from_dir, merge_site_setting
      serde_helpers.rs             - deserialize_yes_no_bool
    preprocess/
      mod.rs                       - PreprocessPipeline struct, run_preprocess
      ast.rs                       - Stmt, Expr, StrPart, Accessor 等 (AST型定義)
      parser.rs                    - PreprocessParser (pest grammar), parse_preprocess, build_*
      interpreter.rs               - Ctx, eval_expr, eval_stmt, eval_method
      preprocess.pest              - pest grammar file
    novel_info.rs                  - NovelInfo (from_toc_source / from_novel_info_source)
    html.rs                        - to_aozora (HTML→青空文庫形式変換)
    info_cache.rs                  - 小説情報キャッシュ
    rate_limit.rs                  - RateLimiter
    security.rs                    - URL検証、SSRF防止
  converter/
    mod.rs                         - NovelConverter struct, convert_novel pipeline, cache
    render.rs                      - render_novel_text (novel.txt.erb相当), ConvertedSection
    output.rs                      - create_output_text_path/filename, extract_domain/ncode_like
    ini.rs                         - IniData / IniValue (INI parser/serializer)
    settings.rs                    - NovelSettings (44 items, INI overlay, replace.txt)
    device.rs                      - OutputManager (端末別出力: epub, mobi, kindle等)
    epub/                          - ネイティブ EPUB3 生成 (Java/AozoraEpub3 非依存)
      mod.rs                       - build_epub オーケストレーション, EpubOptions
      parser.rs                    - 青空中間テキスト→中間表現(Block/Page/Document), 改ページ分割
      xhtml.rs                     - インライン注記/ブロック→XHTML, page/title/nav 生成
      gaiji.rs                     - 外字(面区点/米印/二重山括弧)→Unicode 解決
      package.rs                   - OPF(v3.0)/container.xml 生成, mimetype先頭無圧縮ZIP書出し, 決定論的UUID
      assets.rs                    - 縦書きCSS, 埋め込みフォント, メディアタイプ判定
    dakuten_font.rs                - 濁点フォント処理
    inspector.rs                   - 調査ログ生成 (Inspector)
    converter_base/
      mod.rs                       - ConverterBase struct, TextType, convert pipeline orchestrator
      character_conversion.rs      - 半角/全角変換, 数字→漢数字, TCY
      indentation.rs               - auto_indent, half_indent_bracket, insert_separate_space
      stash_rebuild.rs             - illust/URL/kome stash & rebuild
      ruby.rs                      - narou_ruby, find_ruby_base (ルビ注記処理)
      text_normalization.rs        - rstrip, ellipsis, page_break, dust_char, blank_line 等
    user_converter/
      mod.rs                       - UserConverter struct, load, apply_before/after, signature
      setting_override.rs          - apply_setting_override (converter.yaml設定オーバーライド)
  web/
    mod.rs                         - AppState, create_router (70+ エンドポイント)
    state.rs                       - ApiResponse, IdPath, ListParams 等 (DTO structs)
    novels.rs                      - index, novels_count, api_list, get/remove/freeze/unfreeze
    tags.rs                        - add_tag, remove_tag, update_tags, edit_tag
    batch.rs                       - batch_tag/untag/freeze/unfreeze/remove
    jobs.rs                        - api_download/update/convert, queue_status/clear, send/mail/backup
    novel_settings.rs              - get_settings, save_settings, list_devices
    misc.rs                        - version_current/latest, tag_list, notepad_read/save, recent_logs
    push.rs                        - PushServer, WebSocket, StreamingLogger
    worker.rs                      - バックグラウンドジョブ実行 (子プロセス管理)
    scheduler.rs                   - 自動更新スケジューラ
    frontend.rs                    - Web UI 静的ページ配信
    global_settings.rs             - グローバル設定 API
    sort_state.rs                  - 一覧ソート状態保存
    tag_colors.rs                  - タグ色管理
    update.rs                      - セルフアップデート API
    assets/                        - 静的アセット (CSS, JS)
sample/
  novel/                           - テスト用CWD (.narou/ + webnovel/*.yaml)
  narou/                           - Ruby参照ソース (git submodule的な位置, .gitignore)
```

## Reference Files (Ruby, 読取専用)

- `sample/narou/lib/converterbase.rb` — テキスト変換エンジン (1503行) — **最も重要な参照**
- `sample/narou/lib/novelconverter.rb` — コンバーター全体オーケストレータ (1209行)
- `sample/narou/lib/html.rb` — HTML→青空変換 (124行) — Rustの `html.rs` はこれに準拠
- `sample/narou/template/novel.txt.erb` — 最終テキスト組み立てERBテンプレート (93行)
- `sample/narou/lib/novelsetting.rb` — 設定定義
- `sample/narou/lib/command/*.rb` — 各コマンド実装 (help/CLI挙動の参照元)

## Converter Pipeline (Ruby準拠)

### `convert(text, text_type)` 全体フロー:
1. `rstrip_all_lines` — 全行の行末空白削除
2. user_converter `apply_before`
3. `before_hook`:
   - body/textfile: `convert_page_break` (閾値以上の連続空行→`［＃改頁］`)
   - non-story + pack_blank_line: `\n\n` → `\n`, 先頭3改行を2に制限
4. `convert_for_all_data` — 一括前処理:
   - hankakukana_to_zenkakukana
   - auto_join_in_brackets
   - auto_join_line (if enabled) — `、\n　` のみ結合
   - erase_comments_block
   - replace_illust_tag → `［＃挿絵＝N］`
   - replace_url → `［＃URL=N］`
   - replace_narou_tag — `【改ページ】` を削除
   - convert_numbers — subtitle/chapter/story は全角変換のみ
   - exception_reconvert_kanji_to_num, convert_kanji_num_with_unit, rebuild_kanji_num
   - insert_separate_space
   - stash_kome(`※`→`※※`), convert_double_angle_quotation_to_gaiji, convert_novel_rule, convert_head_half_spaces
   - convert_fraction_and_date, modify_kana_ni_to_kanji_ni, convert_prolonged_sound_mark_to_dash
5. `convert_main_loop` — 行単位処理 + 後処理:
   - zenkaku_rstrip, request_insert_blank, process_author_comment
   - insert_blank_before_line_and_behind_to_special_chapter
   - insert_blank_line_to_border_symbol (■等の前後に空行+4字下げ)
   - outputs(line) → join
   - rebuild_force_indent_chapter
   - rebuild_illust, rebuild_url, rebuild_hankaku_num_comma
   - rebuild_kome_to_gaiji (`※※` → `※［＃米印、1-2-8］`)
   - half_indent_bracket, auto_indent (E000 sentinel marker → `\u{3000}`)
   - narou_ruby, convert_horizontal_ellipsis, convert_double_angle_quotation_to_gaiji_post
   - delete_dust_char
6. user_converter `apply_after`
7. `replace_by_replace_txt` — replace.txt ユーザー定義置換

### `novel.txt.erb` テンプレート構造 (Rustの `render_novel_text` に実装済み):
```
Title\n
Author\n
cover_chuki\n
［＃区切り線］\n
(if story non-empty) あらすじ：\n{story}\n\n
掲載ページ:\n<a href="{toc_url}">{toc_url}</a>\n
［＃区切り線］\n
For each section:
  ［＃改ページ］\n
  (if chapter non-empty)
    ［＃ページの左右中央］\n
    ［＃ここから柱］{title}［＃ここで柱終わり］\n
    ［＃３字下げ］［＃大見出し］{chapter}［＃大見出し終わり］\n
    ［＃改ページ］\n
  (if subchapter non-empty)
    ［＃１字下げ］［＃１段階大きな文字］{subchapter}［＃大きな文字終わり］\n
  \n
  {indent}［＃中見出し］{subtitle}［＃中見出し終わり］\n
  \n\n
  {body}
  (if postscript) ...
(if enable_display_end_of_book) \n［＃ここから地付き］［＃小書き］（本を読み終わりました）［＃小書き終わり］［＃ここで地付き終わり］\n
```

## 技術スタック

- **Language**: Rust (edition 2024)
- **Web framework**: Axum 0.8
- **Async runtime**: Tokio (full features)
- **Serialization**: serde + serde_yaml + serde_json
- **HTTP client**: reqwest (blocking, cookies, gzip/brotli/deflate) + curl crate
- **CLI**: clap 4
- **Date/time**: chrono + chrono-tz
- **Regex**: regex
- **Hashing**: sha2 + hex
- **Error handling**: thiserror
- **Template**: askama
- **Logging**: tracing + tracing-subscriber
- **Sync**: parking_lot, dashmap, tokio::sync
- **Browser open**: open
- **WebSocket**: tokio-tungstenite
- **Random UA**: ua_generator
