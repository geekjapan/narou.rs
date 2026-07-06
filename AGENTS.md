# Project Runtime Kernel
dev-gear: G3 — OpenSpec 導入済み。trivial かつ可逆な単発修正のみ G2 ループ可(gear 定義は user-scope CLAUDE.md の 3 ギア制)

このファイルは、この fork で AI エージェント（Claude Code / Codex 等）が作業するときの project-scope 初期化ルールと、`narou.rs` 固有の開発ルールをまとめたものです。`CLAUDE.md` はこのファイルをインポートするだけの薄いラッパーであり、Claude / Codex のどちらで作業しても本ファイルが唯一の正本となります。

詳細なアーキテクチャ・ソース構造は [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)、開発状況・既知課題は [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) を参照。

## Always-On Rules
- 日本語で応答する。明示指定がある場合だけ別言語にする。
- 非自明な実装、デバッグ、設計、レビュー、workflow 変更は、実ファイルや実サービスを確認してから結論を出す。
- 狭い単発変更は、現物確認、編集、最小検証、報告まで直接進める。
- 完了を主張する前に、対象に合った最小の検証を実行する。
- `rg` と token-efficient な確認手段を優先し、不要な大量出力を避ける。
- 秘密情報、API key、token、local auth、machine-local cache の値は表示・記録・コミットしない。
- repository 内では、より近い `AGENTS.md`、`CLAUDE.md`、domain docs、ADR を優先する。

## Workflow Routing
- change / spec / feature / 設計 / policy / architecture / multi-session 系の作業は、可能なら OPSX/OpenSpec を背骨にする。
- trivial で可逆な単発変更、またはドキュメントの狭い追記は、OPSX を省略して直接処理してよい。
- `diagnose`、検証、レビュー系など非変更系の skill は必要に応じて使ってよい。
- mattpocock 系 skill は OPSX と競合させず、要件整理、TDD、診断、設計深化の精度レイヤーとして使う。

## Response Economy
- 人間との思考・相談・最終説明は日本語で短く書く。
- エージェント運用、worklog、handoff、subagent report は英語で簡潔に書く。
- 前提、不確実性、file path、command、error、test result、risk、changed file、remaining task、next action は省略しない。
- サブエージェントは、広範囲の監査や明確に並列化できる作業だけに使う。1ファイル編集や軽微修正では使わない。

## Project Rules
- 以降の `narou.rs` 固有ルールは、この fork の実装・互換性・Git 運用の正本として扱う。
- user-scope の詳細ルールは各エージェントの user 設定側に置き、project に混ぜる必要があるものだけここへ昇格する。
- GitNexus は廃止済み。`.gitnexus/`、GitNexus hook、GitNexus skill、GitNexus 注入ブロックは再導入しない。

# narou.rs — Rust Port of narou.rb

## Overview
narou.rb（Ruby製の日本のWeb小説管理・電子書籍変換ソフトウェア）のサーバー実行部分をRustに移植するプロジェクト。実装状況のマスタードキュメントは `COMMANDS.md`。

| 完了度 | コマンド数 | 内訳 |
|:------:|:---------:|------|
| ✅ 完了 | 18 | init, list, tag, freeze, remove, setting, diff, send, backup, clean, help, version, log, folder, browser, alias, inspect, csv, trace |
| 🟡 部分 | 5 | download, update, convert, web, mail |

## Porting Policy
- 互換性の主対象は外部から観測できる挙動（CLI引数・戻り値・エラー、YAML構文理解、`.narou/` データ、出力ファイル）。
- 内部実装は Ruby と同一でなくてよい。Rust で保守しやすく、安全で、検証しやすい構成を優先する。
- 詳細な互換性要件は [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) を参照。

## Build & Run
```bash
cargo build              # Build (edition 2024)
cargo check              # Type-check
cargo test               # 全テスト
cargo test --test convert_parity   # 変換互換性テストのみ
cargo run -- convert 2   # カクヨム小説を変換（CWD: sample/novel/）
cargo local-build        # Release と同構成の narou/ フォルダを生成
```

**重要**: `cargo run` は `sample/novel/` をCWDとして実行する必要がある（`.narou/` ディレクトリが必要なため）。

バイナリは 3 つ: `narou_rs`(`src/main.rs`)、`narou_rs_updater`(`src/bin/updater.rs`)、`cargo-local-build`(`src/bin/cargo-local-build.rs`)。
テストは `tests/convert_parity.rs`（byte-for-byte fixture テスト）と各モジュール内のインラインユニットテスト。

## COMMANDS.md 同期ルール
- コマンドの新規実装・オプション追加・フラグ追加・挙動変更を行うたびに、必ず `COMMANDS.md` の該当箇所をリアルタイムに更新する。
- **完了判定の注意**: Ruby 版 `sample/narou/lib/command/*.rb` と、CLI オプション、help 文、Examples、設定項目、終了コード、エラー文を細かく突き合わせ、外部から観測できる挙動が一致していることを確認してから完了にする。

## コミット時のコード整形禁止ルール
- git diff に現れる変更は、機能的な意味を持つものだけにすること。
- 既存行の改行位置変更だけ、`use`/`import` の順番入替だけの変更を禁止する（機能変更に付随して不可避な場合のみ許容）。

## Git 運用ルール
- 作業は `main` から切った作業ブランチ上で行う。`main` に直接コミットしない。
- 機能ブランチ名は短い英数字・ハイフン形式（例: `fix-web-concurrency`）。
- commit メッセージは英語の短い命令形。無関係な変更をひとつの commit に混ぜない。
- **バージョン更新時は必ず `cargo check` を実行し、ビルドが通ることを確認してから commit。** `Cargo.toml` のバージョン更新と `cargo check` による `Cargo.lock` 更新は同じ commit に含める。
- `git reset --hard`、強制 push、履歴改変 rebase は、ユーザーが明示的に依頼した場合以外は行わない。
- タグ作成・ブランチ削除はユーザーが明示的に依頼した場合だけ行う。

## CSS ルール
- WEB UI の CSS で色・サイズ・間隔等を指定する際は `var(--xxx)` 形式の CSS 変数を使う。
- サイズ・間隔・余白・フォントサイズには `em`・`rem`・`%`・`vw`・`vh` などの相対単位のみ使う（`px` 禁止）。
- `@media` クエリのブレークポイントには `em` を使う。

## Dependency Policy
- `Cargo.toml` は原則として直接編集しない。依存クレートの追加・更新は `cargo add`/`cargo update` 経由で行う。
