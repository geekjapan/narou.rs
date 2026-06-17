## 1. トリガ関数の OS 非依存化

- [x] 1.1 `src/converter/device.rs::should_use_aozora_temp_workspace` から先頭の `cfg!(windows) &&` を除去し、`path_contains_aozora_risky_chars(input_txt) || path_contains_aozora_risky_chars(output_dir)` の形にした。
- [x] 1.2 `path_contains_aozora_risky_chars` 内で、Unicode リスキー文字判定を無条件にし（先に return true）、`windows_31j_encode_has_errors` 判定だけを `cfg!(windows) &&` 条件付きにした。
- [x] 1.3 関数リネーム: `path_contains_windows_aozora_risky_chars` → `path_contains_aozora_risky_chars`、`is_windows_aozora_mapping_risky_char` → `is_aozora_mapping_risky_char`。呼び出し箇所・test import を更新（整形のみの差分は混ぜていない）。`windows_31j_encode_has_errors` は名称維持。

## 2. テスト

- [x] 2.1 既存テストをリネームに追従。`aozora_temp_workspace_is_used_for_windows_31j_unencodable_paths`（`title♠.txt`）は CP932 Windows 限定挙動を正しく検証するため不変のまま維持。
- [x] 2.2 `aozora_unicode_risky_chars_detected_cross_platform`（Unicode リスキー文字は全 OS 検出）と `aozora_temp_workspace_copies_assets_for_risky_paths`（`title〜.txt` で全 OS 退避）を追加/改修。`#[cfg(not(windows))]` の `aozora_cp932_unencodable_chars_not_detected_off_windows` で `is_aozora_mapping_risky_char('～')`=true / `should_use_aozora_temp_workspace("title～.txt")`=true を検証。
- [x] 2.3 `#[cfg(windows)] aozora_cp932_unencodable_chars_detected_on_windows` と `#[cfg(not(windows))] aozora_cp932_unencodable_chars_not_detected_off_windows` で、CP932 未定義かつ Unicode リスト外（`♠♡♣♢` / `𠮷`）が Windows でのみリスキー扱いされることを明示。

## 3. 検証（コード）

> rustup(scoop) で Rust 1.96.0 stable(MSVC) を導入し Windows 機で実行・確認済み。

- [x] 3.1 `cargo check --tests` 成功（exit 0、3m32s、MSVC リンカ自動検出、device.rs 警告なし）。
- [x] 3.2 `cargo test --lib converter::device` で 11/11 pass（`aozora_unicode_risky_chars_detected_cross_platform`、`aozora_temp_workspace_copies_assets_for_risky_paths`、`aozora_cp932_unencodable_chars_detected_on_windows` を含む）。`cargo test --lib` フルは 392 pass / 1 fail。唯一の失敗 `web::tests::safe_existing_novel_dir_keeps_suspicious_names_inside_archive_root` は **本変更と無関係の既存フレーク**（変更を stash したベースラインでも同一行 `src/web/mod.rs:793` で失敗、単独実行では pass＝並列実行時のテスト状態汚染）。converter 層の本変更は web 層に影響しない。`cfg(not(windows))` の Linux 専用テストは Windows 機ではコンパイルされないため CI/Linux で実行される。

## 4. 検証（実機 / Open Question 解消）

- [ ] 4.1 Linux 上で `～`(U+FF5E) を含むタイトルの既 DL 小説を `device=kobo` もしくは `convert.use-aozoraepub3=true` で `narou convert` し、EPUB(.kepub.epub) が生成され終了コード 0 になることを確認（`AozoraEpub3 did not create expected output` が出ないこと）。
- [ ] 4.2 生成物の最終ファイル名・拡張子・出力先が、リスキー文字を含む本来のタイトル由来名に復元されている（`input.epub` のまま残らない）ことを確認。
- [ ] 4.3 **Open Question Q1 解消**: 実機 Debian + AozoraEpub3-1.1.1b24Q で、CP932 未定義かつ Unicode リスト外の文字（`𠮷`/`♠` 等）のみを含むタイトルが落ちるかを 1 件検証。落ちる場合は `windows_31j_encode_has_errors` も全 OS 化するフォローアップ（タスク 1.2 の条件を見直し）を起票する。

## 5. ドキュメント

- [x] 5.1 `COMMANDS.md` の convert 節（旧 L268）を更新: Unicode リスキー文字の安全名退避を OS 非依存化、CP932 未定義文字は Windows 限定維持、native epub 経路が Java 非依存である旨を追記。
- [x] 5.2 `.serena/memories/porting_status/aozora_linux_safe_filename_2026-06-17.md` を追加し、`aozora_cp932_filename_2026-05-30` / `aozora_interrobang_filename_2026-05-27` へ相互リンク。
