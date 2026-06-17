## Context

AozoraEpub3 経路の現在の流れ（`src/converter/device.rs`）:

- `OutputManager::convert_file` が device 別に分岐。`Device::Epub`/`Reader`/`Ibooks` は `build_epub_output`（`device.rs:483`）→ 既定 `run_native_epub`（Java 非依存）。`Device::Kobo`（`device.rs:647`）と `Device::Mobi`(kindle, `device.rs:654`) は `run_aozora_epub3` を必ず通る。`convert.use-aozoraepub3=true` / `NAROU_RS_EPUB_ENGINE=aozora` のとき epub 系も `run_aozora_epub3` へ退避（`device.rs:497-508`）。
- `run_aozora_epub3` は `java -jar AozoraEpub3.jar` を起動し、`output_path = output_dir.join(input_txt.file_stem() + output_ext)`（`device.rs:392` 付近）を期待出力パスとして算出。`AozoraEpub3 did not create expected output`（`device.rs:458-462`）で存在チェックする。
- リスキー文字を含む場合の退避は `AozoraInvocation::temporary`（`device.rs:1034`）が担う: `tempfile` で `narou-rs-aozora-` 一時 dir を作り、本文を安全名 `input.txt` として複写、`copy_aozora_companion_files`（`device.rs:1113`）で cover/挿絵を複製、`expected_output_path = temp_root/input.<ext>`。変換後 `needs_final_copy()` なら `move_aozora_output`（`device.rs:472,1151`）で本来の Unicode 名へ復元する。

退避を起動するかの判定が以下で、ここに OS ゲートの問題がある:

```rust
// device.rs:1074
fn should_use_aozora_temp_workspace(input_txt: &Path, output_dir: &Path) -> bool {
    cfg!(windows)
        && (path_contains_windows_aozora_risky_chars(input_txt)
            || path_contains_windows_aozora_risky_chars(output_dir))
}

fn path_contains_windows_aozora_risky_chars(path: &Path) -> bool {
    let path_text = path.to_string_lossy();
    windows_31j_encode_has_errors(&path_text)                 // CP932 エンコード不能（𠮷/♠ 等）
        || path_text.chars().any(is_windows_aozora_mapping_risky_char) // Unicode リスト（～/〜/⁉ 等）
}
```

`cfg!(windows)` により、Linux では `～`/`〜` を含むタイトルでも退避が起こらず、AozoraEpub3 の正規化した別名と期待パスが不一致になって失敗する。これが Issue #11 の Debian 再現の根本である。

制約: AGENTS.md の外部互換性要件（出力ファイル名・配置・CLI 挙動の不変）。`native-epub3-output` change の「`Device::Mobi`/`Kobo` 経路を変更しない」要件。`Cargo.toml` 直接編集禁止。

## Goals / Non-Goals

**Goals:**
- 非 Windows(Linux/macOS) でも、AozoraEpub3 経路に渡すパスが Unicode リスキー文字（`～`/`〜` 等）を含む場合に安全名ワークスペースで変換し、`AozoraEpub3 did not create expected output` を解消する。
- 最終出力ファイル名・配置・拡張子・CLI 挙動を一切変えない（失敗→成功のみ）。
- Windows の既存挙動を厳密に維持する（回帰ゼロ）。
- device 別経路選択（ネイティブ既定、Kobo/Mobi=AozoraEpub3）を変えない。

**Non-Goals:**
- ネイティブ EPUB3 経路の変更（本症状は元から無関係）。
- AozoraEpub3 のファイル名正規化規則そのものの完全模倣（取りこぼし文字リストの網羅は段階的に拡充）。
- Windows-31J 未定義文字（`𠮷`/`♠`）の非 Windows 対応の確定（Open Question Q1 で実機検証）。
- Web UI 変更。

## Decisions

### Decision 1: Unicode リスキー文字リストを OS 非依存トリガにする
`should_use_aozora_temp_workspace` から先頭の `cfg!(windows) &&` を外し、CP932 判定だけを Windows 限定に残す:

```rust
fn should_use_aozora_temp_workspace(input_txt: &Path, output_dir: &Path) -> bool {
    path_contains_aozora_risky_chars(input_txt)
        || path_contains_aozora_risky_chars(output_dir)
}

fn path_contains_aozora_risky_chars(path: &Path) -> bool {
    let path_text = path.to_string_lossy();
    // CP932 エンコード不能判定は Windows のパス/コンソール encoding 固有問題のため Windows 限定。
    let cp932_risky = cfg!(windows) && windows_31j_encode_has_errors(&path_text);
    // Unicode リスキー文字（AozoraEpub3 自身が出力名正規化で落とす）は OS 非依存。
    cp932_risky || path_text.chars().any(is_aozora_mapping_risky_char)
}
```

- 理由: `is_aozora_mapping_risky_char` が列挙する文字（`～`/`〜`/`−`/`‼`/`⁇`/`⁈`/`⁉`/FE0E/FE0F）は、Windows のパス encoding ではなく AozoraEpub3(Java) の出力ファイル名正規化に起因する。これは Java 側の挙動なので OS に依存しない。よって OS 非依存トリガが正しい。
- 代替案: `should_use_aozora_temp_workspace` 全体を無条件で全 OS に開放（CP932 判定も含む）→ 非 Windows の UTF-8 FS では CP932 round-trip 判定が無意味に多くのタイトルを退避させる過剰トリガになるため初版では却下。CP932 分の扱いは Q1 で実機検証して判断。

### Decision 2: 関数リネームで「Windows 限定でない」意図を明示
- `path_contains_windows_aozora_risky_chars` → `path_contains_aozora_risky_chars`
- `is_windows_aozora_mapping_risky_char` → `is_aozora_mapping_risky_char`
- `windows_31j_encode_has_errors` は名称維持（Windows-31J 固有判定であることを名前で示す）。

整形のみの差分を避けるため、リネームは呼び出し箇所（`should_use_aozora_temp_workspace`、既存テスト名）と定義の最小限に留める。

### Decision 3: 退避機構（temporary / move_aozora_output）は不改変で再利用
`AozoraInvocation::temporary`・`copy_aozora_companion_files`・`move_aozora_output`・`needs_final_copy` には手を入れない。トリガ条件だけを広げる。これにより最終ファイル名復元・companion コピーの実績ある挙動をそのまま流用でき、出力互換が保たれる。

### Decision 4: スコープは AozoraEpub3 を実際に通る経路のみ
影響は `run_aozora_epub3` を呼ぶ経路（Kobo / Mobi(kindle) 中間 epub / `convert.use-aozoraepub3` / `NAROU_RS_EPUB_ENGINE=aozora`）に限定。ネイティブ EPUB3（Epub/Reader/Ibooks 既定）は `should_use_aozora_temp_workspace` を経由しないため無影響。device 別経路選択ロジックは変更しない（`native-epub3-output` spec 整合）。

## Risks / Trade-offs

- [非 Windows での過剰トリガ] Unicode リスキー文字を含むタイトルで、Linux でも安全名ワークスペースを使う回数が増える。ただし退避は「一時 dir への複写 + 変換後 move」だけで副作用が無く、出力も不変なので**害は軽微なコピーコストのみ**。トレードオフとして正当。
- [AozoraEpub3 正規化挙動の未再検証] `sample/narou` 未チェックアウトのため、AozoraEpub3-1.1.1b24Q が Linux 上で `～` をどう正規化するかは Windows 観測からの推測。→ 実機 Debian で `～` タイトルの kobo 変換成功を最終確認（タスク 4.x）。
- [CP932 未定義文字の Linux 対応漏れ] `𠮷`/`♠` のみを含むタイトルは本変更後も非 Windows では退避されない。→ Open Question Q1 で実機検証し、落ちるなら `windows_31j_encode_has_errors` も全 OS 化（過剰トリガは許容）する後追い。
- [回帰] Windows 挙動は CP932 判定込みで完全維持（`cfg!(windows)` 分岐が CP932 側に残る）。既存 Windows テスト（`windows_aozora_risky_chars_are_detected` 等）は不変で通るはず。

## Migration Plan

1. `device.rs` のトリガ関数を Decision 1/2 の形へ修正（リネーム + ゲート移動）。既存 Windows テストがそのまま通ることを確認。
2. `#[cfg(not(windows))]` の新テストで、`～` を含むパスが `should_use_aozora_temp_workspace`=true、`is_aozora_mapping_risky_char('～')`=true、`普通のタイトル` が false になることを検証。
3. `cargo check` / `cargo test`（変換に影響しないため `convert_parity` は必須でないが、念のため実行）。
4. Linux 実機で `～` タイトル小説を device=kobo もしくは `convert.use-aozoraepub3=true` で `convert` し、EPUB 生成成功と最終ファイル名が本来名であることを確認。
5. `COMMANDS.md` convert 節と `.serena` メモを更新。
6. ロールバック: 変更は単一関数のトリガ条件のみ。問題時は `should_use_aozora_temp_workspace` に `cfg!(windows) &&` を戻すだけで従来挙動へ復帰。

## Open Questions

- **Q1**: Linux 上の AozoraEpub3-1.1.1b24Q は、Unicode リスキー文字リストに無いが CP932 未定義の文字（`𠮷`/`♠` 等）を含むタイトルでも `did not create expected output` を起こすか? → 実機 Debian で 1 件検証。落ちるなら `windows_31j_encode_has_errors` も全 OS 化する（害は過剰トリガのみ）。落ちないなら現状の Windows 限定を維持。
- **Q2（方針確認）**: Debian での第一推奨回避策は「ネイティブ epub 経路（device=epub かつ `convert.use-aozoraepub3` 無効）」で良いか? その場合 Java/AozoraEpub3 不要で本症状を構造的に回避できる。COMMANDS.md にそう明記してよいか。
- **Q3**: 取りこぼし文字リスト（`is_aozora_mapping_risky_char`）に追加すべき既知文字は他にあるか（Issue #5 で挙がった文字は網羅済み）。新規報告が来たらリストへ追記する運用で良いか。
