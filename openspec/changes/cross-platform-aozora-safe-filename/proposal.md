## Why

Rumia-Channel/narou.rs Issue #11。Debian(Linux) + openjdk25 + AozoraEpub3-1.1.1b24Q で、タイトルに `～`(U+FF5E) を含む小説を `narou convert` すると、縦書き中間テキスト生成までは成功するが AozoraEpub3 段階で

```text
Conversion error: AozoraEpub3 did not create expected output: .../n0287me ...～...
```

となり EPUB が生成されない。

原因は `src/converter/device.rs::should_use_aozora_temp_workspace`（`device.rs:1074`）が `cfg!(windows)` でゲートされており、AozoraEpub3 がファイル名正規化で取りこぼす文字（`～`/`〜` 等）を含むパスを **安全名の一時ワークスペースへ退避する処理が非 Windows ではコンパイル除外**されている点。その結果 Linux では AozoraEpub3 が正規化した別名で EPUB を書き、`device.rs:392` で算出した期待パスが見つからず `device.rs:458` の存在チェックでエラーになる。

本 fork は epub/reader/ibooks を **既定でネイティブ EPUB3 生成**するため、device 未指定（=epub）では本症状は再現しない。しかし AozoraEpub3 を実際に通る経路では Linux でも発生する:

- `Device::Kobo`（`.kepub.epub`、`device.rs:647`）
- `Device::Mobi`(kindle) の中間 `.epub`（`device.rs:654`）
- `convert.use-aozoraepub3=true` もしくは `NAROU_RS_EPUB_ENGINE=aozora`（`device.rs:497-508`）

CP932/Windows-31J 未定義文字（`𠮷`/`♠` 等）に基づく退避トリガ（`windows_31j_encode_has_errors`、`device.rs:1086`）は Windows のパス/コンソール encoding 固有問題のため Windows 限定で正しいが、Unicode コードポイント由来のリスキー文字リスト（`is_windows_aozora_mapping_risky_char`、`device.rs:1091`。`～`/`〜` を含む）は AozoraEpub3(Java) 自身の出力名正規化に起因し OS 非依存であるべきなのに、同じ `cfg!(windows)` ゲート下に置かれている。

## What Changes

- AozoraEpub3 へ渡すパスのリスキー文字検出と安全名ワークスペース退避を、**Unicode コードポイント由来のリスキー文字リストについては全プラットフォーム共通化**する。`～`(U+FF5E)/`〜`(U+301C)/`−`(U+2212)/`‼`/`⁇`/`⁈`/`⁉`/異体字セレクタ(FE0E/FE0F) 等を含むパスは Windows / Linux / macOS いずれでも安全名ワークスペースで変換する。
- `should_use_aozora_temp_workspace`（`device.rs:1074`）から先頭の `cfg!(windows) &&` ゲートを外す。
- `windows_31j_encode_has_errors`（CP932 エンコード不能判定、`device.rs:1086`）は Windows 固有問題への対処として **Windows 限定のまま残す**（`path_contains_aozora_risky_chars` 内で `cfg!(windows)` 条件付きにする）。非 Windows での CP932 未定義文字（`𠮷`/`♠`）も AozoraEpub3 が落とすかは実機検証で確定する（Open Question Q1）。
- リスキー判定関数から `windows_` プレフィックスを外す: `path_contains_windows_aozora_risky_chars` → `path_contains_aozora_risky_chars`、`is_windows_aozora_mapping_risky_char` → `is_aozora_mapping_risky_char`。挙動が Windows 限定でなくなる意図を名前に反映する。
- 退避時は既存の `AozoraInvocation::temporary`（`device.rs:1034`、安全名 `input.txt` への複写 + cover/挿絵の companion コピー）と `move_aozora_output`（`device.rs:472,1151`、本来の Unicode 名へ復元）を**そのまま再利用**する。最終出力ファイル名・拡張子・配置は不変。

## Capabilities

### New Capabilities
- `aozora-safe-filename`: AozoraEpub3 経路に渡すパスが AozoraEpub3 のファイル名正規化で問題になる Unicode 文字を含む場合に、実行 OS を問わず安全名の一時ワークスペースで変換し、生成物を本来のファイル名へ復元する。

### Modified Capabilities
<!-- 既存 spec は openspec/specs/ に未整備のため delta 対象なし。native-epub3-output の「Device::Mobi / Kobo の経路を変更しない」要件とは整合する（経路選択は変えず、AozoraEpub3 へ渡す直前のファイル名退避のみ追加する）。 -->

## Impact

- **影響コード**: `src/converter/device.rs` のみ。
  - `should_use_aozora_temp_workspace`（`device.rs:1074-1078`）の `cfg!(windows)` ゲート除去。
  - `path_contains_aozora_risky_chars` / `is_aozora_mapping_risky_char` へのリネームと、CP932 判定の Windows 限定化。
  - 非 Windows でも安全名ワークスペースが使われることを検証する `#[cfg(not(windows))]` テスト追加。
- **外部互換性（AGENTS.md 厳守）**:
  - 最終 EPUB / `.kepub.epub` のファイル名・拡張子・出力先は `move_aozora_output` による復元で**不変**。narou.rb 出力名互換を維持。
  - CLI 引数・終了コード・エラーメッセージは変更しない（失敗していたケースが成功に変わるのみ）。
  - device 別経路選択（Epub/Reader/Ibooks=ネイティブ、Kobo/Mobi=AozoraEpub3）は変更しない。`native-epub3-output` spec の Mobi/Kobo 経路不変要件と整合。
- **依存関係**: 新規 crate なし（`tempfile`・`encoding_rs` の `SHIFT_JIS` は既に device.rs で使用中）。`Cargo.toml` 直接編集なし。
- **ドキュメント**: `COMMANDS.md` の convert 節へ「Linux でも AozoraEpub3 経路のリスキー文字タイトルを安全に変換」「Debian での推奨回避策はネイティブ epub 経路（device=epub かつ `convert.use-aozoraepub3` 無効）」を追記。`.serena/memories/porting_status/` にメモ追加（既存 `aozora_cp932_filename_2026-05-30.md` / `aozora_interrobang_filename_2026-05-27.md` から相互リンク）。
- **検証**: Linux 上で `～` を含むタイトルの既 DL 小説を device=kobo もしくは `convert.use-aozoraepub3=true` で `convert` し EPUB 生成成功を確認。`cargo test`（非 windows でも temp workspace が使われる新テスト）。実機 Debian + AozoraEpub3-1.1.1b24Q での最終確認を推奨（`sample/narou` 未チェックアウトのため AozoraEpub3 の `～` 正規化挙動は現状未再検証）。
