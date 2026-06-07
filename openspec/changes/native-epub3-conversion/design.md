## Context

`narou convert` の EPUB 生成は現在、外部の `AozoraEpub3.jar` を Java(JRE) で起動する方式に全面依存している。中核となる流れは以下:

- `converter/mod.rs::convert_novel` が変換パイプラインを実行し、`render.rs::render_novel_text` で青空文庫形式の中間テキストを生成、これをファイルへ書き出す。
- `device.rs::OutputManager::convert_file(input_txt, output_dir, base_name, include_illust)` がデバイス別に分岐し、`Device::Epub`/`Kobo`/`Reader`/`Ibooks`/`Mobi` で `run_aozora_epub3(input_txt, output_dir, ext)` を呼ぶ（`device.rs:581-588`）。
- `run_aozora_epub3` は `~/.narousetting/global_setting.yaml` の `aozoraepub3dir` から jar を探し、`java -jar AozoraEpub3.jar` を起動して EPUB を得る。

参照実装の出力構造（`~/run/narou_rs/.../*.epub` を `unzip` で確認済み）:

```
mimetype                       (stored/無圧縮, 先頭)
META-INF/container.xml         (rootfile → item/standard.opf)
item/standard.opf              (<package version="3.0">, spine page-progression-direction="rtl")
item/nav.xhtml                 (epub:type="toc" の目次)
item/toc.ncx                   (後方互換 NCX)
item/xhtml/title.xhtml         (html class="hltr" 横組みの表紙)
item/xhtml/0001.xhtml..        (html class="vrtl" 縦組みの本文。話単位で分割)
item/style/*.css               (book-style.css 他)
```

注記→XHTML の主要マッピング（参照出力で確認）:
- 本文1行 → `<p>…</p>`、空行 → `<p><br/></p>`
- `［＃区切り線］` → `<hr/>`
- 中見出し → `<div class="mt3"><h2 class="font-1em30">…</h2></div>`
- 前書き → `<div class="introduction">…</div><div class="clear"></div>`
- 縦中横 → `<span class="tcy">…</span>`、ルビ → `<ruby>…<rt>…</rt></ruby>`
- N字下げ → 行頭の全角空白による表現

narou.rs が生成しうる青空注記の語彙は別途網羅調査済み（改頁/区切り線/各種見出し・柱/字下げ・地付き/前書き後書き/太字斜体取消線傍点/縦中横/ルビ/外字（米印・二重山括弧・面区点）/挿絵/URL、および内部 stash マーカー）。

制約: AGENTS.md の外部互換性要件（出力ファイル名・配置・CLI 挙動の不変、AozoraEpub3 出力との構造・本文互換重視、サイト固有ロジックのハードコード禁止、`Cargo.toml` 直接編集禁止）。

## Goals / Non-Goals

**Goals:**
- Java/AozoraEpub3 不在でも `Device::Epub` の EPUB3 を Rust 単体で生成する。
- 入力は既存の青空文庫中間テキスト（AozoraEpub3 に渡しているものと同一）を再利用し、`converter_base/` の変換ロジックには手を入れない。
- AozoraEpub3 出力と章構成・本文・ルビ・縦書き・目次レベルで互換な EPUB3 を生成（バイト一致は非目標）。
- 既存 AozoraEpub3 経路を温存し、後方互換な経路選択を提供する。
- 注記→XHTML マッパは網羅した注記語彙をカバーし、未知注記でフェイルセーフ。

**Non-Goals:**
- mobi/kindle のネイティブ生成（kindlegen 依存が残るため本変更ではスコープ外。`Device::Mobi` は従来経路のまま）。
- AozoraEpub3 とのバイト単位一致や、AozoraEpub3 独自 CSS（style-advance.css 等）の完全再現。
- 縦書き以外（横組み）レイアウトの新規オプション追加。
- 挿絵画像のダウンロード／生成（既存の取得済み画像を埋め込むのみ）。
- Web UI 側の新規 UI（経路選択はまず CLI/設定で提供。Web 連携は後続）。

## Decisions

### Decision 1: 入力は青空中間テキストを再利用（変換器は不改変）
ネイティブ経路は `convert_file` に渡る `input_txt`（青空文庫中間テキスト）を入力とし、そこから EPUB を構築する。`converter_base/`・`render.rs` は変更しない。

- 理由: 中間テキストは「なろう・カクヨムでバイト一致」まで作り込まれた安定資産。これを唯一の真実として再利用すれば、AozoraEpub3 経路とネイティブ経路で本文が一致し、二重メンテを避けられる。サイト固有処理にも触れないため AGENTS.md のハードコード禁止に抵触しない。
- 代替案: `ConvertedSection`（構造化データ）から直接 EPUB を組む案。本文整形（ルビ・外字・縦中横等）が中間テキスト生成段で確定するため、構造体経由だと整形ロジックを再実装する必要があり却下。ただし `render.rs` が持つ章/話メタ（タイトル・著者・section 境界）は、分割と目次生成のために補助的に参照できるよう検討する。

### Decision 2: 新モジュール `src/converter/epub/` を新設
責務を分離した小モジュール群を置く:
- `epub/mod.rs` — エントリ `build_epub(input_txt, meta, opts, output_path)`。`device.rs` から呼ぶ公開 API。
- `epub/aozora_parser.rs` — 青空中間テキストを行/ブロックのイベント列（中間 AST）へ解析。注記語彙を集中管理。
- `epub/xhtml.rs` — イベント列 → 本文 XHTML 文字列。話単位分割、エスケープ、ルビ/縦中横/見出し/前書き等のタグ生成。
- `epub/gaiji.rs` — 面区点・米印・二重山括弧 → Unicode/外字画像/代替のマッピング表。
- `epub/package.rs` — OPF/nav.xhtml/container.xml/title.xhtml 生成と、`zip` crate による mimetype 先頭・無圧縮の EPUB ZIP 書き出し。
- `epub/assets.rs` — `preset/` 由来の CSS・フォント・外字画像の同梱制御。

- 理由: `device.rs`（1438 行）にこれ以上ロジックを積まない。注記解析・XHTML 化・パッケージングは関心が異なるためファイル分割し、ユニットテストを各層に置く。
- 既存層との整合: `converter/`(変換層)内に閉じる。`db`/`downloader`/`web`/`queue` には影響しない。`device.rs` は `convert_file` の `Device::Epub` 分岐から `epub::build_epub` を呼ぶだけにする。

### Decision 3: 経路選択は「設定項目 + 自動フォールバック」
- 既定: AozoraEpub3 が利用可能（`aozoraepub3dir` 有効かつ Java 解決可）なら従来経路。不可なら自動的にネイティブ経路へフォールバックして EPUB を生成する。
- 明示選択: ローカル設定（`.narou/local_setting.yaml`）に真偽値の設定項目（例 `converter.native_epub` を想定。最終キー名は実装時に既存設定命名規約へ合わせて確定）を追加し、有効時は AozoraEpub3 が使えてもネイティブ経路を使う。
- 環境変数によるテスト用オーバーライドも検討（検証で AozoraEpub3 を持つ環境でもネイティブを強制するため）。

- 理由: 「Java 無しでも動く」ことが主目的なので、未設定時はまず後方互換（既存挙動）を守りつつ、AozoraEpub3 不在時だけ自動でネイティブに切り替えると利用者体験が最良。明示設定で先進利用者がネイティブを常用できる。
- 代替案: 既定でネイティブに切替 → 既存ユーザーの出力が変わり後方互換を損なうため却下。CLI フラグのみ → 設定で恒久化できず不便なため、設定項目を主、必要なら補助フラグ。
- 互換性: 設定項目は追加のみ。未知キーを既存 setting が破壊しないことを確認する。narou.rb 側に同名設定は無いが、`narou setting` は任意キーの読み書きを許容する方針のため整合する。

### Decision 4: EPUB レイアウトは参照構造に倣いつつ簡素化
参照（AozoraEpub3）の `mimetype`/`META-INF/container.xml`/`item/standard.opf`/`item/nav.xhtml`/`item/xhtml/*`/`item/style/*` の骨格を踏襲する。CSS は AozoraEpub3 の多層 CSS をそのまま使わず、`preset/vertical_font*.css` を母体にした自前の最小縦書き CSS セットへ集約する。`toc.ncx` は EPUB3 単独では必須でないが、リーダ互換のため任意で併置を検討。

- 理由: リーダ互換に効く骨格（縦書き・右綴じ・目次）は踏襲しつつ、AozoraEpub3 固有の巨大 CSS を持ち込まない。`preset/` は既にプロジェクト同梱資産で、行高 `<%= line_height %>` 等のテンプレ展開実績がある。
- マッピング基準: 本文行→`<p>`、空行→`<p><br/></p>`、`［＃区切り線］`→`<hr/>`、中見出し→見出し要素、前書き→`introduction` クラス、縦中横→`tcy` クラス、ルビ→`<ruby>` を採用（参照出力に一致）。

### Decision 5: 外字(gaiji)はマッピング表で解決、未解決は代替へ
`epub/gaiji.rs` に面区点(N-N-N)→Unicode の対応表を持ち、米印・二重山括弧など頻出外字を最優先で網羅する。Unicode 化できないものは (a) `preset`/AozoraEpub3 同梱の外字画像、(b) 判読可能な代替文字、のいずれかで出力し、空欄・文字化けを禁止する。

- 理由: spec 要件「文字化け・欠落の禁止」を満たす。多くの面区点は Unicode に対応文字があるため、まず Unicode マッピングで実用十分。
- 代替案: 全外字を画像化 → EPUB 肥大化と実装コスト大。Unicode 優先＋限定画像フォールバックが妥当。

### Decision 6: ZIP 生成は既存 `zip` crate を使用
`device.rs` で既に `zip`(`ZipWriter`, `SimpleFileOptions`, `CompressionMethod`)を利用中。EPUB の `mimetype` は `CompressionMethod::Stored` で先頭に、その他は Deflate で格納する。

- 理由: 新規依存を増やさない（`Cargo.toml` 直接編集回避方針とも整合）。XHTML/OPF は文字列テンプレートで生成し、重量級 XML ライブラリは導入しない（エスケープ専用の小ヘルパは自前 or 既存依存）。

## Risks / Trade-offs

- [リーダ互換の差異] 自前 CSS が AozoraEpub3 ほど多様なリーダで検証されていない → 主要リーダ（iBooks/Kobo/一般 EPUB3 リーダ）と epubcheck で確認し、`preset/vertical_font*.css` 由来の確立された指定を踏襲して差異を抑える。
- [注記網羅漏れ] 想定外の注記でレイアウト崩れ → パーサは未知注記をフェイルセーフに本文化し、注記語彙調査結果を `aozora_parser.rs` のテスト表に固定。AozoraEpub3 出力との差分比較で漏れを検出。
- [外字の取りこぼし] 面区点表の不足で代替出力が増える → 頻出外字を優先網羅、未解決はログに残し段階的に拡充。空欄化は禁止。
- [出力非互換の混入] ネイティブ既定化で既存ユーザー出力が変わる懸念 → 既定は後方互換（AozoraEpub3 優先）に固定し、ネイティブは不在時フォールバック＋明示設定のみ。
- [挿絵パス解決] 挿絵相対パスと実ファイル配置の不一致 → 既存の挿絵保存場所（stash/rebuild が前提とするパス）を実測し、存在時のみ埋め込む。無ければ画像を含めずスキップ（中断しない）。
- [二重メンテ] AozoraEpub3 経路とネイティブ経路の本文差異 → 入力を共通の中間テキストに限定することで本文ロジックの二重化を回避。

## Migration Plan

1. `epub/` モジュールを追加（既存コードへの影響なし、ビルドのみ）。
2. `device.rs::convert_file` の `Device::Epub` 分岐に経路選択を追加（既定は従来挙動を維持）。
3. 設定項目・自動フォールバックを有効化。
4. 検証（unzip 構造・epubcheck・AozoraEpub3 出力比較・`cargo test`）後、`COMMANDS.md`/`AGENTS.md` を更新。
5. ロールバック: 設定既定が後方互換のため、問題時はネイティブ経路の呼び出し分岐を無効化するだけで従来挙動に戻る。Kobo/Ibooks/Reader への拡張は本変更後の別 change で。

## Open Questions

- 経路選択の最終的な設定キー名（`converter.native_epub` 等）と、補助 CLI フラグの要否。既存 `NovelSettings`/`local_setting.yaml` の命名規約に合わせて実装時に確定。
- `toc.ncx` を併置するか（EPUB3 では nav が正だが、古いリーダ互換のため）。
- フォント埋め込みの既定値（ON/OFF）。EPUB サイズと表示品質のトレードオフ。
- 外字画像の調達元（`preset/` か AozoraEpub3 同梱 `gaiji/` を参照するか、Unicode 代替のみで足りるか）。
- Kobo/Ibooks/Reader をネイティブ経路に含める範囲（本変更は Epub 限定。.kepub.epub の kobo span 付与は後続 change を想定）。
