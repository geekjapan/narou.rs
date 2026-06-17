## ADDED Requirements

### Requirement: OS 非依存のリスキー文字ファイル名退避

AozoraEpub3 経路（`Device::Kobo`、`Device::Mobi` の中間 EPUB、`convert.use-aozoraepub3=true`、`NAROU_RS_EPUB_ENGINE=aozora`）に渡す入力テキストパスまたは出力ディレクトリパスが、AozoraEpub3 がファイル名正規化で取りこぼす Unicode 文字（`～`(U+FF5E)、`〜`(U+301C)、`−`(U+2212)、`‼`(U+203C)、`⁇`(U+2047)、`⁈`(U+2048)、`⁉`(U+2049)、異体字セレクタ U+FE0E/U+FE0F 等）を含む場合、実行プラットフォーム（Windows / Linux / macOS）に関わらず、安全な一時ファイル名のワークスペースで AozoraEpub3 を実行 SHALL する。変換後、生成物は本来の Unicode ファイル名・出力先へ復元 MUST する。

#### Scenario: Linux で ～ を含むタイトルを AozoraEpub3 経路で変換

- **WHEN** Debian 等の非 Windows 環境で、タイトルに `～`(U+FF5E) または `〜`(U+301C) を含む小説を、AozoraEpub3 を通るデバイス（`--device kobo`）または `convert.use-aozoraepub3=true` で `narou convert` する
- **THEN** 安全名ワークスペースで AozoraEpub3 が実行され、有効な EPUB ファイル（kobo は `.kepub.epub`）が生成され、終了コードは 0 になる
- **AND** `AozoraEpub3 did not create expected output` エラーは発生しない

#### Scenario: 最終ファイル名が本来の Unicode 名に復元される

- **WHEN** リスキー文字を含むタイトルを安全名ワークスペース経由で（OS を問わず）変換する
- **THEN** 生成された EPUB のファイル名・拡張子・出力先ディレクトリは、リスキー文字を含む本来のタイトル由来名と一致し、一時名（`input.epub` 等）のまま残らない

#### Scenario: リスキー文字を含まないタイトルは退避しない

- **WHEN** リスキー文字を一切含まないタイトル（例「普通のタイトル」）を AozoraEpub3 経路で変換する
- **THEN** 一時ワークスペースは使われず、従来どおり出力先ディレクトリへ直接生成される

### Requirement: デバイス別経路選択の不変

本変更はファイル名退避のトリガ条件のみを拡張 SHALL し、デバイス別の出力経路選択（Epub / Reader / Ibooks は既定でネイティブ EPUB3、Kobo / Mobi は AozoraEpub3、`convert.use-aozoraepub3` および `NAROU_RS_EPUB_ENGINE` による切替）を変更 MUST しない。

#### Scenario: epub 既定はネイティブ経路のまま

- **WHEN** device 未指定または `--device epub` で、かつ `convert.use-aozoraepub3` が無効の状態で `narou convert` する
- **THEN** 従来どおりネイティブ EPUB3 経路で生成され、`java` / AozoraEpub3 は起動されず、安全名ワークスペース判定も経由しない

#### Scenario: Windows の既存挙動は不変

- **WHEN** Windows 上でリスキー文字を含むタイトルを AozoraEpub3 経路で変換する
- **THEN** 本変更前と同じく安全名ワークスペースが使われ、CP932 未定義文字（`𠮷`/`♠` 等）も従来どおり退避対象となり、出力結果は変わらない

### Requirement: Windows-31J エンコード判定の Windows 限定維持

Windows-31J(CP932) でエンコード不能な文字に基づく退避トリガ（`windows_31j_encode_has_errors` 相当）は、Windows のパス／コンソール encoding 固有問題への対処であり、引き続き Windows 限定 SHALL とする。非 Windows 環境では、この CP932 エンコード判定単独では退避を強制 MUST しない（退避は Unicode リスキー文字リストに基づいて行う）。非 Windows での CP932 未定義文字の扱いは実機検証で別途確定する。

#### Scenario: 非 Windows で CP932 未定義のみのタイトルの退避判定

- **WHEN** 非 Windows 環境で、Unicode リスキー文字リストには含まれないが Windows-31J では未定義の文字（例 `♠`）のみを含むパスについて退避要否を判定する
- **THEN** CP932 エンコード判定による退避は強制されず（Windows 限定のまま）、退避要否は Unicode リスキー文字リストの有無のみで決まる
