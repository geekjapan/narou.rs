## 1. 調査と土台

- [x] 1.1 AozoraEpub3 参照 EPUB（`~/run/narou_rs/.../*.epub`）の OPF/nav/title/本文 XHTML/CSS を再確認し、ネイティブ生成の目標構造をメモ（`design.md` の構造と突合）。
- [x] 1.2 青空注記語彙の網羅表を `epub/aozora_parser.rs` のテスト用 fixture として固定（改頁/区切り線/見出し・柱/字下げ・地付き/前書き後書き/太字斜体取消線傍点/縦中横/ルビ/外字/挿絵/URL）。
- [x] 1.3 挿絵画像の実保存パスと中間テキスト中の `［＃挿絵（…）入る］` 相対パスの対応関係を実測し記録。

## 2. epub モジュール骨格

- [x] 2.1 `src/converter/epub/mod.rs` を作成し公開 API `build_epub(...)` の型シグネチャ（入力: 中間テキスト・メタ・オプション・出力先）を定義（スタブ）。
- [x] 2.2 `converter/mod.rs` に `pub mod epub;` を追加し、`cargo check` が通ることを確認。
- [x] 2.3 オプション構造体（縦書き既定、フォント埋め込み可否、挿絵可否、外字方針）を定義。

## 3. 青空注記パーサ (aozora_parser.rs)

- [x] 3.1 中間テキストを行単位＋ブロック範囲（前書き/後書き/見出し/字下げ等の開始終了ペア）に分解する中間 AST/イベント型を定義。
- [x] 3.2 行内注記（ルビ `｜親《ルビ》`、縦中横 `［＃縦中横］…［＃縦中横終わり］`、傍点/傍線/太字/斜体/取消線）の解析を実装。
- [x] 3.3 外字注記（米印 `※［＃米印、1-2-8］`、二重山括弧、面区点 `［＃…、N-N-N］`）をトークン化。
- [x] 3.4 挿絵 `［＃挿絵（…）入る］`・URL・区切り線・改頁・ページ左右中央を解析。
- [x] 3.5 未知 `［＃…］` 注記をフェイルセーフに本文化する分岐とユニットテスト。

## 4. XHTML レンダラ (xhtml.rs)

- [x] 4.1 イベント列→本文 XHTML 文字列生成（行→`<p>`、空行→`<p><br/></p>`、`［＃区切り線］`→`<hr/>`）と XML エスケープ。
- [x] 4.2 見出し（大/中）・柱・前書き/後書き（`introduction` 等）・字下げ・地付きのブロック要素生成。
- [x] 4.3 ルビ→`<ruby><rt>`、縦中横→`<span class="tcy">`、傍点/傍線/太字/斜体/取消線のインライン要素生成。
- [x] 4.4 章/話単位の本文分割（section 境界で別 XHTML へ）と各話のタイトル抽出（目次用）。
- [x] 4.5 XHTML head（縦書き `class="vrtl"`、CSS link）とタイトルページ（`class="hltr"`）生成。
- [x] 4.6 レンダラのユニットテスト（代表注記が期待 XHTML になることを検証）。

## 5. 外字解決 (gaiji.rs)

- [x] 5.1 面区点→Unicode のマッピング表を実装（米印・二重山括弧など頻出を最優先で網羅）。
- [x] 5.2 未解決外字のフォールバックは代替文字で出力（空欄/文字化け禁止）＋ログ出力。外字画像フォールバックは初版スコープ外。
- [x] 5.3 マッピングのユニットテスト。

## 6. EPUB3 パッケージング (package.rs)

- [x] 6.1 `mimetype`（無圧縮・先頭）＋ `META-INF/container.xml` を生成し `zip` crate で書き出し。
- [x] 6.2 OPF(`version="3.0"`) 生成: metadata（title/creator/language/identifier/modified）、manifest（全 XHTML/CSS/画像/フォント）、spine（`page-progression-direction="rtl"`、`nav` properties）。**OPF パスは `item/standard.opf` 固定、metadata は `</metadata>` で閉じる**（既存 `add_dc_subject_to_epub` 後処理互換のため必須）。identifier は `sha2` 由来の決定論的 `urn:uuid:`。
- [x] 6.3 `nav.xhtml`（`epub:type="toc"` 目次 + landmarks）を生成し各話/タイトルへリンク。目次は `nav.xhtml` のみ（`toc.ncx` は出さない）。
- [x] 6.4 全エントリを ZIP へ統合する `build_epub` 本体を完成。

## 7. CSS・フォント・挿絵資産 (assets.rs)

- [x] 7.1 `preset/vertical_font*.css` を母体に EPUB 同梱用の最小縦書き CSS を整備（行高テンプレ展開、`tcy`/`introduction`/見出しクラス対応）。
- [x] 7.2 フォント埋め込み（`preset/DMincho.ttf`）を設定で制御し manifest 登録。
- [x] 7.3 挿絵画像が存在する場合に manifest 登録＋本文 `<img>` 参照、無効/不在時はスキップ。

## 8. device.rs 統合と経路選択

- [x] 8.1 `device.rs::convert_file` の `Device::Epub`/`Reader`/`Ibooks` 分岐を既定ネイティブ経路に切替（`run_aozora_epub3` は温存）。`Mobi`/`Kobo` は不変。
- [x] 8.2 ローカル設定 `convert.use-aozoraepub3`（bool, 既定 false）を読み取り、true かつ AozoraEpub3 解決可なら従来経路、解決不可ならネイティブへフォールバック。設定追加が既存 setting 読み書きを壊さないこと（`narou setting` 互換）を確認。
- [x] 8.3 検証用の環境変数オーバーライド（経路の強制切替で比較検証）を実装。
- [x] 8.4 出力ファイル名・配置・拡張子・エラー/終了コードが従来経路と一致することを確認。

## 9. 検証

- [x] 9.1 `cargo check` と `cargo test`（epub モジュールのユニットテスト含む）を通す。変換に影響する変更時は `cargo test --test convert_parity` も実行。epub モジュール 29 テスト通過。フルスイートの失敗2件（`normalize_windows_verbatim_path_strips_prefix`、`notepad_path_uses_narou_root_instead_of_current_dir`）はベースラインでも再現する macOS 環境依存で epub 無関係。`convert_parity` のバイト一致ケースは gitignore 済み `sample/` フィクスチャ不在で skip 相当、フィクスチャ不要ケースは通過。
- [x] 9.2 `cargo local-build` 後 `cp target/release/narou_rs ~/run/narou_rs/app/`、既 DL 済み小説（なろう短編・カクヨム）でネイティブ経路 `convert` を実行。
- [x] 9.3 生成 EPUB を `unzip` 検証: mimetype 先頭・無圧縮、container.xml、OPF `version="3.0"`、nav.xhtml 目次、本文 XHTML、縦書き CSS。
- [x] 9.4 `epubcheck` があれば通し、FATAL/ERROR が無いことを確認。
- [x] 9.5 AozoraEpub3 経路の出力と章構成・本文・ルビ・縦書き・目次を比較し差分を確認。
- [x] 9.6 `convert.add-dc-subject-to-epub` 有効時に、ネイティブ生成 EPUB へ `add_dc_subject_to_epub` が `<dc:subject>` を注入できることを確認（OPF 名・`</metadata>`・mimetype 前提の整合）。

## 10. ドキュメント

- [x] 10.1 `COMMANDS.md` の convert 節を更新（ネイティブ EPUB 経路・経路選択設定・Java 非依存を反映）。
- [x] 10.2 `AGENTS.md` の Converter Pipeline / 実装状況節を更新（device.rs の経路分岐と新 epub モジュールを反映）。
- [x] 10.3 Serena メモ／関連メモに最新実装状況を反映（`conversion/native_epub3_2026-06-07.md` 追加、`porting_status.md` の convert 行更新）。
