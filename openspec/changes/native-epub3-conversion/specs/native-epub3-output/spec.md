## ADDED Requirements

### Requirement: Java/AozoraEpub3 非依存の EPUB 生成

`narou convert` は、Java(JRE) と `AozoraEpub3.jar` がいずれもインストールされていない環境でも、`.epub` を出力するデバイス（Epub / Reader / Ibooks）に対して有効な EPUB3 ファイルを生成 SHALL する。生成には外部プロセス（`java` 等）を起動 MUST しない。

#### Scenario: Java も AozoraEpub3 も無い環境で EPUB を生成
- **WHEN** `aozoraepub3dir` が未設定、かつ `java` が PATH に存在しない環境で、変換済み小説に対し `narou convert <target> --device epub` を実行する
- **THEN** 小説フォルダに narou.rb 互換のファイル名・配置で `.epub` ファイルが生成され、終了コードは 0 になる
- **AND** 変換処理中に `java` などの外部プロセスは起動されない

#### Scenario: ネイティブ生成 EPUB は ZIP として展開可能
- **WHEN** ネイティブ経路で生成した `.epub` を `unzip` で展開する
- **THEN** 破損なく全エントリが展開でき、`mimetype` の内容が `application/epub+zip` である

#### Scenario: Reader / Ibooks でも EPUB を生成
- **WHEN** Java/AozoraEpub3 不在の環境で `--device reader` または `--device ibooks` を指定して変換する
- **THEN** Epub と同一経路でネイティブ EPUB3 が生成され、終了コードは 0 になる

### Requirement: 経路選択（既定ネイティブ）と退避口

`.epub` を出力するデバイス（Epub / Reader / Ibooks）では、ネイティブ EPUB3 生成経路を既定 SHALL とする。利用者は設定項目（`convert.use-aozoraepub3` 相当）により従来の AozoraEpub3 経路へ明示的に切り替え SHALL できる。経路選択は決定的でなければならない。`Device::Mobi` / `Device::Kobo` の経路は本変更で変更 MUST しない。

#### Scenario: 既定でネイティブ経路を使用
- **WHEN** 経路を明示指定しない状態で `narou convert <target> --device epub` を実行する
- **THEN** AozoraEpub3 が利用可能であってもネイティブ経路で EPUB が生成される

#### Scenario: 設定で AozoraEpub3 経路へ退避
- **WHEN** AozoraEpub3 を選択する設定項目を有効にし、有効な `aozoraepub3dir` がある状態で `narou convert` を実行する
- **THEN** 従来どおり AozoraEpub3 経路で EPUB が生成される

#### Scenario: AozoraEpub3 選択時に AozoraEpub3 が使えない場合
- **WHEN** AozoraEpub3 を選択する設定が有効だが `aozoraepub3dir`/Java が解決できない状態で `narou convert <target> --device epub` を実行する
- **THEN** ネイティブ経路へフォールバックして EPUB を生成し、終了コードは 0 になる

#### Scenario: mobi/kobo の経路は不変
- **WHEN** `--device mobi` または `--device kobo` で `narou convert` を実行する
- **THEN** 本変更導入前と同一の経路（AozoraEpub3 / kindlegen）で処理され、挙動は変わらない

### Requirement: 出力ファイル名・配置・CLI 互換

ネイティブ経路で生成する EPUB の出力ファイル名・拡張子・配置先ディレクトリ、および `convert` の CLI 引数・終了コード・主要なエラーメッセージは、既存実装（AozoraEpub3 経路）および narou.rb と互換 SHALL である。

#### Scenario: 出力ファイル名と配置が AozoraEpub3 経路と一致
- **WHEN** 同一小説を AozoraEpub3 経路とネイティブ経路でそれぞれ EPUB 変換する
- **THEN** 生成される `.epub` のファイル名・拡張子・出力先ディレクトリが一致する

#### Scenario: 変換対象が存在しない場合のエラー互換
- **WHEN** 存在しない target を指定して `narou convert` を実行する
- **THEN** 既存実装と同一のエラーメッセージと終了コードを返す

### Requirement: EPUB3 パッケージ構造

ネイティブ経路が生成する EPUB は EPUB3 仕様に準拠 SHALL する。先頭エントリは無圧縮(stored)の `mimetype`（内容 `application/epub+zip`）であり、`META-INF/container.xml` から OPF(rootfile)を解決でき、OPF は `version="3.0"` を宣言 MUST する。OPF は全 XHTML・CSS・画像・フォント資産を `manifest` に列挙し、本文を `spine` に順序付けし、`nav` プロパティを持つ目次 XHTML（`nav.xhtml`）を含 MUST む。

#### Scenario: mimetype が先頭・無圧縮
- **WHEN** 生成 EPUB の ZIP セントラルディレクトリ／ローカルヘッダを検査する
- **THEN** 最初のエントリが `mimetype` で、圧縮方式は stored(無圧縮) である

#### Scenario: container から OPF を解決
- **WHEN** `META-INF/container.xml` の `rootfile` を辿る
- **THEN** 参照先の OPF ファイルが存在し、`<package ... version="3.0">` を宣言している

#### Scenario: manifest と spine の整合
- **WHEN** OPF の `manifest` と `spine` を検査する
- **THEN** `spine` の各 `itemref` は `manifest` の item を参照し、`properties="nav"` を持つ目次 item が存在する

#### Scenario: epubcheck 検証（利用可能な場合）
- **WHEN** 生成 EPUB を `epubcheck` に通す
- **THEN** EPUB3 として致命的(FATAL/ERROR)な検証エラーが報告されない

### Requirement: 縦書きレイアウト

ネイティブ経路は既定で縦書き(縦組み右綴じ)の EPUB を生成 SHALL する。spine は右から左へ進行（`page-progression-direction="rtl"`）し、本文 XHTML には縦書き(`writing-mode: vertical-rl` 相当)を指定する CSS が適用 MUST される。

#### Scenario: 右綴じ・縦書き指定
- **WHEN** 生成 EPUB の OPF と本文 XHTML/CSS を検査する
- **THEN** spine に `page-progression-direction="rtl"` があり、本文に縦書きを指定する CSS（`writing-mode: vertical-rl`）が適用されている

### Requirement: 章/話単位の本文分割と目次

ネイティブ経路は、小説を章/話（section）単位で複数の本文 XHTML に分割 SHALL し、`nav.xhtml` の目次から各話へリンク MUST する。表紙(タイトル)ページを含 MUST む。目次の項目構成は AozoraEpub3 経路と互換（各話・章見出しが目次に現れる）であることを目指す。

#### Scenario: 話ごとに XHTML が分割される
- **WHEN** 複数話の小説をネイティブ変換する
- **THEN** 本文が話単位で別々の XHTML エントリに分割され、それぞれが spine に順序付けられている

#### Scenario: 目次から本文へ遷移できる
- **WHEN** `nav.xhtml` の目次リンクを辿る
- **THEN** 各リンクが実在する本文 XHTML（およびタイトルページ）を指している

### Requirement: 青空文庫注記の XHTML への変換

ネイティブ経路は、`render.rs::render_novel_text` が出力する青空文庫形式中間テキストを入力 SHALL とし、以下の注記を意味を保って XHTML へ変換 MUST する: 改頁（`［＃改ページ］`）、区切り線（`［＃区切り線］`→`<hr/>`）、大/中見出しと柱（running head）、N字下げ・地付き・地寄せ、前書き/後書きブロック、傍点・傍線・太字・斜体・取消線、縦中横（`［＃縦中横］`）、ルビ（`｜親《ルビ》`→`<ruby>`）、空行（→空段落）。未知・未対応の注記に遭遇しても処理を中断せず、安全に本文テキストとして扱う（フェイルセーフ）MUST。

#### Scenario: 行・空行・区切り線の対応
- **WHEN** 本文の各行・空行・`［＃区切り線］` を含む中間テキストを変換する
- **THEN** 各本文行が段落要素に、空行が空段落に、区切り線が `<hr/>` に対応した XHTML が生成される

#### Scenario: ルビが ruby 要素に変換される
- **WHEN** `｜親文字《るび》` を含む中間テキストを変換する
- **THEN** 親文字とルビ文字が `<ruby>`/`<rt>` 構造で表現される

#### Scenario: 縦中横・見出し・前書きの変換
- **WHEN** 縦中横・中見出し・前書きブロックを含む中間テキストを変換する
- **THEN** 縦中横は縦中横用のインライン要素、中見出しは見出し要素、前書きは前書き用ブロック要素に変換される

#### Scenario: 未知注記でも中断しない
- **WHEN** 未対応の `［＃…］` 注記を含む中間テキストを変換する
- **THEN** 変換は失敗せず、当該注記は本文として安全に出力され、その他の本文は正しく変換される

### Requirement: 外字(gaiji)・特殊文字の扱い

ネイティブ経路は外字注記（面区点 `［＃…、N-N-N］`、米印 `※［＃米印、1-2-8］`、二重山括弧 `※［＃始め二重山括弧］`/`※［＃終わり二重山括弧］`）を、表示可能な文字へ解決 SHALL する。Unicode に対応する文字があればその文字へマッピングし、解決できない外字は判読可能な代替（同梱フォント文字・外字画像・代替文字のいずれか）として出力 MUST する。文字化けや空欄での欠落を起こしてはならない。

#### Scenario: 米印・二重山括弧の解決
- **WHEN** `※［＃米印、1-2-8］`・`※［＃始め二重山括弧］`・`※［＃終わり二重山括弧］` を含む中間テキストを変換する
- **THEN** それぞれ米印・始め二重山括弧・終わり二重山括弧として判読可能な文字が出力され、注記記法そのものは本文に露出しない

#### Scenario: 未解決外字のフォールバック
- **WHEN** Unicode に対応文字が存在しない面区点外字を含む中間テキストを変換する
- **THEN** 判読可能な代替（外字画像または代替文字）として出力され、空欄・文字化けにならない

### Requirement: 挿絵・画像と URL 注記の扱い

ネイティブ経路は挿絵注記（`［＃挿絵（相対パス）入る］`）を、対象画像が存在する場合 EPUB の `manifest` へ登録し本文 XHTML から参照 SHALL する。URL 注記から復元された URL 文字列は本文中にリンクまたはテキストとして保持 MUST する。挿絵が無効化（`enable_illust=false` 相当）されている場合は画像を含めない。

#### Scenario: 挿絵が EPUB に埋め込まれる
- **WHEN** 挿絵注記と対応する画像ファイルを持つ小説をネイティブ変換する（挿絵有効）
- **THEN** 画像が EPUB の manifest に登録され、本文 XHTML から `<img>` で参照され、ZIP に画像エントリが含まれる

#### Scenario: URL がリンクとして保持される
- **WHEN** URL を含む本文（掲載ページ等）を変換する
- **THEN** URL が本文 XHTML 内にリンクまたはテキストとして保持され、全角化などで破壊されない

### Requirement: CSS とフォント資産の同梱

ネイティブ経路は縦書き表示に必要な CSS を EPUB に同梱 SHALL し、`preset/` の縦書き CSS（`vertical_font*.css`）を母体とする。フォント埋め込み（`preset/DMincho.ttf` 等）は設定で制御 SHALL し、埋め込み時は OPF manifest に登録して本文から参照する。

#### Scenario: 縦書き CSS が同梱される
- **WHEN** 生成 EPUB を検査する
- **THEN** 縦書きを規定する CSS が ZIP に含まれ、OPF manifest に登録され、本文 XHTML から参照されている

#### Scenario: フォント埋め込みの切替
- **WHEN** フォント埋め込みを無効化して変換する
- **THEN** 生成 EPUB にフォントファイルが含まれず、有効化時は manifest 登録の上で含まれる
