## Why

現在 `narou convert` で EPUB を生成するには、外部の `AozoraEpub3.jar` を Java(JRE) で起動する必要がある（`src/converter/device.rs` の `OutputManager` が `java -jar AozoraEpub3.jar` を実行）。Java と AozoraEpub3 の導入は利用者にとって障壁が大きく、環境構築の失敗や JRE 不在で EPUB 生成が丸ごと不能になる。Rust 単体で完結するネイティブ EPUB3 生成経路があれば、追加ランタイムなしで `narou convert` が EPUB を出力でき、配布・実行が大幅に容易になる。

## What Changes

- 青空文庫形式の中間テキスト（`render.rs::render_novel_text` の出力。現在 AozoraEpub3 に渡しているものと同一）を入力として、Rust ネイティブで EPUB3 ファイルを生成する変換経路を新設する。
- 対応注記: 改頁/区切り線/ページ左右中央、大・中見出しと柱(running head)、各種字下げ・地付き・地寄せ、前書き/後書き、傍点・傍線・太字・斜体・取消線、縦中横、ルビ（`｜親《ルビ》`）、外字(gaiji: 面区点 `［＃…、N-N-N］`・米印・二重山括弧)、挿絵 `［＃挿絵（…）入る］`、URL注記。
- 縦書き EPUB3（`writing-mode: vertical-rl`、spine `page-progression-direction="rtl"`）を既定とし、CSS は `preset/vertical_font*.css` を母体に EPUB3 同梱用へ整備する。
- EPUB3 パッケージ生成: 先頭・無圧縮 `mimetype` + `META-INF/container.xml` + OPF(`version="3.0"`) + `nav.xhtml`(目次) + 本文 XHTML(章/話単位分割) + 表紙(title) + CSS、フォント/外字画像の埋め込み（任意・設定で制御）。ZIP 生成は `zip` crate を使用。
- 経路選択（既定 vs ネイティブ）は後方互換を保って導入する。既存 AozoraEpub3 経路は当面残す。具体的な切替方式（自動フォールバック／設定項目／フラグ）は design で確定する。
- スコープは Device::Epub を最低ラインとする。mobi/kindle は kindlegen 依存が残るため本変更のスコープ外（design で明記）。Kobo(.kepub.epub)・Ibooks・Reader への拡張余地を設計段階で残す。

## Capabilities

### New Capabilities
- `native-epub3-output`: AozoraEpub3.jar / Java に依存せず、青空文庫形式中間テキストから EPUB3 ファイルを生成する変換経路。経路選択・後方互換、EPUB3 パッケージ構造、青空注記→XHTML マッピング、縦書き・目次・表紙、外字/挿絵/ルビの扱い、出力ファイル名・配置・CLI 挙動の互換、検証要件を含む。

### Modified Capabilities
<!-- 既存 spec は openspec/specs/ に未整備のため delta 対象なし。CLI 引数・出力ファイル名・配置の外部挙動は不変のため要件変更は発生しない。 -->

## Impact

- **影響コード**:
  - `src/converter/device.rs`（`OutputManager::convert_file` の経路分岐。新ネイティブ経路を追加。既存 `run_aozora_epub3` は温存）。
  - `src/converter/` 配下に新モジュール（青空注記パーサ + XHTML レンダラ + EPUB3 パッケージャ）を追加。
  - `src/converter/mod.rs`（`convert_novel` パイプラインからのネイティブ経路呼び出し）、`src/commands/convert.rs`（必要時のフラグ/設定読取）。
  - `preset/`（EPUB3 同梱用 CSS の追加・整備。`DMincho.ttf` 等の埋め込み資産参照）。
- **依存関係**: `zip` crate（既に device.rs で使用中）を EPUB ZIP 生成に利用。XML/XHTML はテンプレート文字列または既存依存で生成（新規重量依存は避ける）。`Cargo.toml` 直接編集はせず必要時 `cargo add` 経由。
- **外部互換性（AGENTS.md 厳守）**:
  - 出力ファイル名・配置・拡張子（`Device::Epub` = `.epub`）、CLI 引数・終了コード・エラーメッセージは既存／narou.rb と互換を維持。
  - AozoraEpub3 が生成する EPUB と章構成・本文・ルビ・縦書き・目次の互換を重視（バイト一致は不要）。`~/run/narou_rs/AozoraEpub3/` 経路の出力を比較基準にする。
  - サイト固有・注記処理のハードコードは増やさず、既存 `converter_base/` の中間テキストを再利用する。
- **ドキュメント**: `COMMANDS.md` の convert 節、`AGENTS.md` の Converter Pipeline 節を実態に合わせ更新。
- **検証**: 既 DL 済み小説で `convert` し、生成 EPUB を `unzip` で構造検証（mimetype 先頭・無圧縮、container.xml、OPF `version="3.0"`、nav.xhtml 目次、本文 XHTML、縦書き CSS）。epubcheck があれば通す。`cargo test`（変換まわり）。
