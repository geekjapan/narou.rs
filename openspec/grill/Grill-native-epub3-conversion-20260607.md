# Grill 残課題 — native-epub3-conversion (20260607)

Phase 1（自己グリル）/ Phase 2（横断グリル: change は本件のみ）を実施。docs(AGENTS.md/COMMANDS.md/sample/narou) とコードベースで解消できた項目は design.md/tasks.md に inline 反映済み。以下はユーザー確認が必要な残課題。

## 自己グリルで inline 解消済み（参考・要確認ではない）
- OPF パスを `item/standard.opf` に固定（既存 `add_dc_subject_to_epub` が `standard.opf`/`</metadata>`/`mimetype` を前提とするため。`convert.rs:649,668,678`）。
- XHTML クラス語彙を参照 EPUB 互換（`vrtl`/`hltr`/`tcy`/`introduction`/`mt3`）に統一し、同梱 CSS を `preset/vertical_font*.css` 母体に集約。
- 章/話分割は `［＃改ページ］` 境界（`render.rs:72`、参照 EPUB と同型）。
- `dc:identifier` は既存依存 `sha2` 由来の決定論的 `urn:uuid:`（新規 crate 不要）。
- 設定は `convert.*` 名前空間（`load_local_setting_bool` 経由）が確立済み。

---

### Q1. 経路選択の設定キー名・既定値と CLI フラグの要否
- **対象**: design.md Decision 3 / spec「経路選択と後方互換」/ tasks 8.1-8.2
- **なぜ重要**: キー名が割れると `narou setting` 互換と将来の設定移行に影響。既定値次第で既存ユーザーの出力挙動が変わりうる。
- **検討した選択肢**: A) `convert.native-epub`(bool, 既定 false)＋自動フォールバック / B) 既定で全面ネイティブ化 / C) CLI フラグのみ（設定恒久化なし）
- **推奨案**: A。`convert.*` 名前空間に既存 bool キーが多数（`convert.no-epub` 等、`cli.rs`/`convert.rs`）あり整合。既定 false＋「Java 不在時のみ自動ネイティブ」で後方互換を担保。CLI フラグは当面不要（必要なら後続）。
- **不足インプット**: キー名の最終承認、既定値（false 維持でよいか）、CLI フラグ追加希望の有無。
- **Status**: Resolved — 既定をネイティブ経路化。AozoraEpub3 退避は `convert.use-aozoraepub3`(bool,既定false)、CLIフラグなし (proposal.md / spec「経路選択（既定ネイティブ）と退避口」/ design Decision 3 / tasks 8.1-8.2)

### Q2. 自動フォールバックの対象デバイス範囲
- **対象**: spec「Java/AozoraEpub3 非依存の EPUB 生成」/ design Goals,Non-Goals / tasks 8.1
- **なぜ重要**: `Device::Reader`/`Ibooks` も `run_aozora_epub3` で `.epub` を生成（`device.rs:586-587`）。Java 不在時、これらも対象にしないとユーザーは依然 EPUB を作れない。Kobo は `.kepub.epub`(kobo span 必要)、Mobi は kindlegen 必要で別物。
- **検討した選択肢**: A) Epub のみ / B) Epub + Reader + Ibooks（同一 `.epub` 出力なので低コスト）/ C) さらに Mobi も（native epub→kindlegen、Java 不要だが kindlegen 依存）
- **推奨案**: B。Reader/Ibooks は出力が実質 Epub と同一なので同経路で吸収。Kobo は後続 change、Mobi は本変更スコープ外（kindlegen 前提のまま）。
- **不足インプット**: Reader/Ibooks を含めてよいか、Mobi の native-epub 化を今回やるか後回しか。
- **Status**: Resolved — 対象は Epub+Reader+Ibooks。Kobo/Mobi 経路は不変 (proposal.md / spec / design Decision 3 / tasks 8.1)

### Q3. `dc:modified`（更新日時メタ）の決定性方針
- **対象**: design Decision 7 / spec「EPUB3 パッケージ構造」
- **なぜ重要**: AozoraEpub3 は変換時刻（wall-clock）を `dcterms:modified` に書く。決定論にすると再変換で出力安定（比較・回帰が容易）だが AozoraEpub3 と挙動が変わる。byte 一致は非目標だが方針を固定したい。
- **検討した選択肢**: A) 決定論（小説の最終更新日時など）/ B) wall-clock（AozoraEpub3 同等）
- **推奨案**: A。テスト容易性と再現性を優先。互換は構造/本文/目次レベルで足り、modified の時刻一致は不要。
- **不足インプット**: A/B どちらを採るか。
- **Status**: Resolved — dc:modified は決定論的ソース (design Decision 7 / spec)

### Q4. フォント埋め込みの既定値
- **対象**: spec「CSS とフォント資産の同梱」/ design Decision 6 / tasks 7.2
- **なぜ重要**: `preset/DMincho.ttf`(約319KB) を埋め込むと表示品質は安定するが EPUB が肥大化。既定値で全出力サイズが変わる。
- **検討した選択肢**: A) 既定 OFF（設定で ON） / B) 既定 ON
- **推奨案**: A。サイズ優先。縦書き表示はリーダ標準フォントで成立し、必要な利用者のみ ON。
- **不足インプット**: 既定 OFF でよいか。
- **Status**: Resolved — フォント埋め込み既定 OFF (spec「CSS とフォント資産の同梱」/ tasks 7.2)

### Q5. 外字(gaiji)の初版対応範囲
- **対象**: spec「外字(gaiji)・特殊文字の扱い」/ design Decision 5 / tasks 5.x
- **なぜ重要**: 「空欄/文字化け禁止」を満たす実装コストに直結。外字画像同梱まで初版でやるかで作業量が大きく変わる。
- **検討した選択肢**: A) 面区点→Unicode マッピング＋未解決は判読可能な代替文字（画像は後続）/ B) 初版から外字画像フォールバックも実装（`preset`/AozoraEpub3 `gaiji/` 参照）
- **推奨案**: A。頻出外字（米印・二重山括弧等）は Unicode 化で実用十分。spec の「代替文字でも可」を満たす。画像は将来拡充。
- **不足インプット**: A で初版可とするか、画像フォールバック必須か。
- **Status**: Resolved — Unicode マッピング＋代替文字、外字画像は後続 (design Decision 5 / tasks 5.2)

### Q6. `toc.ncx` の併置要否
- **対象**: spec「章/話単位の本文分割と目次」/ design Decision 4,7
- **なぜ重要**: EPUB3 は `nav.xhtml` が正で `toc.ncx` は任意。旧リーダ互換のため参照 EPUB は併置している。出さない判断も可。
- **検討した選択肢**: A) `nav.xhtml` + `toc.ncx` 併置（参照と同じ）/ B) `nav.xhtml` のみ
- **推奨案**: A。低コストで旧リーダ互換が広がる。
- **不足インプット**: 併置するか nav のみにするか。
- **Status**: Resolved — nav.xhtml のみ、toc.ncx は出さない (design Decision 4/Open Questions / tasks 6.3)
