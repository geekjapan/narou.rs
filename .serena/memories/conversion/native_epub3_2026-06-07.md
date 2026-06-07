# Native EPUB3 conversion (2026-06-07)

## Goal
Generate EPUB3 directly in Rust for `.epub`-producing devices, removing the Java/AozoraEpub3 dependency for the default path. OpenSpec change: `openspec/changes/native-epub3-conversion`.

## Route selection (src/converter/device.rs)
- `epub` / `reader` / `ibooks` devices now default to the native path (`build_epub_output` → `run_native_epub`).
- `mobi` (kindlegen) and `kobo` (AozoraEpub3) routes are unchanged.
- `OutputManager::use_aozora_engine` decides the engine:
  1. env `NAROU_RS_EPUB_ENGINE=native|aozora` forces the route (validation/comparison only).
  2. else local_setting `convert.use-aozoraepub3` (bool, default false). When true AND both `aozora_epub3_path` and a resolvable Java command exist, fall back to the legacy AozoraEpub3 path; otherwise native.
- `run_aozora_epub3` is preserved intact for the fallback.

## Native module (src/converter/epub/)
- `mod.rs` — `build_epub(input_txt, output_dir, output_ext, &EpubOptions)` orchestration; `EpubOptions { embed_font, include_illust, line_height }`.
- `parser.rs` — Aozora intermediate text → IR (Block/Page/Document); chapter/page splitting on page breaks.
- `xhtml.rs` — inline annotations + blocks → XHTML; page/title/nav generation; XML escaping.
- `gaiji.rs` — gaiji (men-ku-ten / kome-jirushi `※` / double angle brackets) → Unicode; unresolved → substitute char + log (no blanks/mojibake). Gaiji image fallback is out of scope for v1.
- `package.rs` — `mimetype` (stored, first entry) + `META-INF/container.xml` + OPF v3.0 (`page-progression-direction="rtl"`, nav properties) + ZIP writeout via `zip` crate. Deterministic `urn:uuid` identifier from `toc_url` (sha2). `dcterms:modified` from input file mtime.
- `assets.rs` — vertical-writing CSS, optional embedded font, media-type detection.

## Compatibility constraints (must hold)
- OPF path is fixed at `item/standard.opf` and metadata closes with `</metadata>` so the existing `add_dc_subject_to_epub` post-processor (which assumes `standard.opf` + `</metadata>` + stored mimetype) still injects `<dc:subject>` into native EPUBs.
- Output filename / placement / extension / exit codes match the legacy route.
- Settings: `convert.use-aozoraepub3` (default false), `convert.epub-embed-font` (default false). Adding these does not break `narou setting` read/write compat.

## Verification
- `cargo check` passes; epub module unit tests: 29 passing.
- Full `cargo test` has 2 pre-existing macOS-env failures unrelated to epub: `normalize_windows_verbatim_path_strips_prefix` (`\\?\` verbatim prefix) and `notepad_path_uses_narou_root_instead_of_current_dir` (`/var` vs `/private/var` symlink). Both reproduce on baseline with converter changes stashed.
- `cargo test --test convert_parity`: the kakuyomu byte-for-byte case needs the gitignored `sample/` fixtures (absent in this checkout); the fixture-free parity case passes.
- Real-device check on `~/run/narou_rs`: 8 works (short / serialized / Kakuyomu) converted; epubcheck 5.3.0 reports 0 errors / 0 warnings.
- unzip-verified: mimetype first+stored, container.xml, OPF v3.0, nav.xhtml TOC, body XHTML, vertical CSS.

## Files changed
- `src/converter/device.rs`, `src/converter/mod.rs` (`pub mod epub;`)
- new `src/converter/epub/{mod,parser,xhtml,gaiji,package,assets}.rs`
- docs: `COMMANDS.md`, `AGENTS.md`
