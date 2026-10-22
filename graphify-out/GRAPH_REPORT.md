# Graph Report - narou.rs  (2026-10-23)

## Corpus Check
- 133 semantic source files; AST status: extracted. Exclusions and reuse evidence: build-audit.json.

## Summary
- 4312 nodes · 10325 edges · 317 communities (161 shown, 156 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 274 edges (avg confidence: 0.85)
- Token usage: per-project usage unavailable (native Codex semantic extraction).

## Graph Freshness
- Built from commit: `a07e7d00`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- jobs.rs
- illustration_store.rs
- downloader/mod.rs
- converter/mod.rs
- path
- compat.rs
- render.js
- actions.js
- worker.rs
- SiteSetting
- diff.rs
- novels.rs
- convert.rs
- main.js
- .convert_for_all_data
- web/mod.rs
- progress.rs
- send.rs
- help.rs
- character_conversion.rs
- .new
- device.rs
- logger.rs
- dakuten_font.rs
- sort_state.rs
- NovelSettings
- inventory.rs
- inspector.rs
- commands/update.rs
- OutputManager
- push.rs
- queue.rs
- fetch.rs
- toc.rs
- PushServer
- render.rs
- text_normalization.rs
- xhtml.rs
- src/mail.rs
- download.rs
- scheduler.rs
- .download_novel_with_force
- tag_colors.rs
- frontend.rs
- PersistentQueue
- manage.rs
- misc.rs
- NovelRecord
- setting.rs
- paths.rs
- settings.js
- updater.rs
- process_novel_for_update
- feature_tour.rs
- global_settings.rs
- src/version.rs
- database.rs
- web.rs
- clean.rs
- info_extraction.rs
- web/update.rs
- Commands
- init.rs
- setting_core.rs
- ruby_time.rs
- hashmap
- cli.rs
- main.rs
- mail_e2e.rs
- Stmt
- fs
- rate_limit.rs
- HttpFetcher
- cargo-local-build.rs
- log.rs
- setting_info.rs
- illust.rs
- IndexStore
- section.rs
- QueueJob
- state.js
- preprocess/parser.rs
- PreprocessResult
- csv.rs
- epub/mod.rs
- Inventory
- .try_register_client
- state.rs
- interpreter.rs
- request_guard_middleware
- updater_promote.rs
- SubtitleInfo
- novel_info.rs
- ast.rs
- context_menu.js
- start_queue_worker_for_lane
- security.rs
- cmd_tag
- output.rs
- .push_internal
- QueueLane
- ApiResponse
- web_tray.rs
- Value
- strip_mobi_sources
- NovelInfoCache
- RateLimiter
- tags.rs
- narou_rs third-party license inventory
- commands/mail.rs
- load_mail_setting
- run_scheduled_auto_update
- Value
- cmd_list_inner
- types.rs
- Expr
- DigestChoice
- package.rs
- apply_general_lastup_check_result
- is_web_mode
- Block
- narou_api.rs
- fetchJson
- write
- preprocess_args
- inspect.rs
- commands/version.rs
- epub/parser.rs
- DivKind
- Method
- StreamingLogger
- duration
- super
- send_target_with_setting_interruptible
- title.rs
- start_auto_update_scheduler
- bind_or_shutdown_and_retry
- update_general_lastup
- Value
- run_web_server
- .new
- pathbuf
- convert_parity.rs
- gaiji.rs
- VarType
- host_allowed
- JobOutcome
- narou_rs Web novel CLI and Web UI
- smtp_tls
- setting_scope
- paint
- resolve_web_address
- MailSetting
- narou.rs project runtime kernel
- IllustSubcommand
- reroute_web_line_to_console
- i18n.js
- format_queue_job_target
- Persistent queue with separate network and local lanes
- de
- package-release.ps1
- build_domain_groups
- HeadingLevel
- output_macros.rs
- convert generates ebook formats and conversion inspection logs
- Hameln site definition
- AGENTS.md authoritative instructions
- Per-domain parallel download with serial requests inside each domain
- 2026-06-07
- Native EPUB3
- EPUB3 packaging
- EPUB validation
- EPUB compatibility decisions
- Aozora annotation mapping
- SMTP TLS
- Version API
- Author comments API
- Bookmarklet registration
- Download queue
- Context menu and shared notepad
- SIL Open Font License 1.1
- Apache License 2.0
- Novel management help
- Novel management dashboard
- Optimistic notepad synchronization
- Per-novel settings
- Reboot return navigation
- Global settings
- Kakuyomu Apollo preprocessing
- Narou site extraction
- Narou R18 extraction
- Paginated TOC and introduction/body/postscript patterns
- Category-aware story URL, title cleanup and article extraction
- OpenSpec propose
- OpenSpec spec sync
- OpenSpec verification
- OpenSpec propose
- OpenSpec spec sync
- OpenSpec verification
- OpenSpec propose
- OpenSpec spec sync
- OpenSpec verification
- alias maps validated names to targets
- backup produces ZIP archives
- browser and folder openers
- clean finds orphan raw and section files
- External observable CLI, YAML, data and output compatibility
- csv export/import routes imported URLs through download
- diff compares section versions
- download resolves targets, downloads sections and chains conversion
- freeze stores frozen novels in freeze.yaml
- help and version
- illust orphan/migrate/fix-ext/rebuild defaults to dry-run
- init creates novel directory structure and resolves AozoraEpub3
- inspect displays conversion diagnostics
- list filters and outputs IDs for pipelines
- log and trace
- mail supports SMTP with TLS required by default
- remove deletes index by default or files with --with-file
- send transfers ebooks and bookmarks to devices
- setting manages local/global/default/force configuration
- tag manages validated names and tag_colors.yaml
- Domain documentation
- GitHub issue tracking
- Triage labels
- Narou Rust architecture
- Development status
- Related narou fork
- Narou.rs release pipeline
- Cross-platform Aozora filename design
- Spec-driven change
- Cross-platform Aozora filename proposal
- Aozora safe filename specification
- Cross-platform Aozora filename tasks
- Native EPUB3 design
- OpenSpec project configuration
- narou_rs
- Narou init compatibility
- YAML-driven preprocessing
- Kakuyomu conversion parity
- Kakuyomu parity fixture
- Native EPUB3 conversion
- Relative CSS units
- Arcadia encoding
- Hameln fetch fallback
- Branching policy
- CP932-unsafe Aozora paths
- Interrobang Aozora paths
- Cross-platform Aozora filenames
- Convert stdin targets
- Narou porting status
- Illustration dedup and Web state
- Illustration hash store
- Release 0.2.12
- Release 0.2.8
- Site timezone handling
- Title date decoration
- Update compatibility status
- Last update timestamp preservation
- Version command parity
- Web auto-update queue
- Web settings parity
- Tag search parity
- Narou project architecture
- Module split record
- Narou style conventions
- Narou command reference
- Narou task completion
- Serena project configuration
- Narou.rs Web UI parity

## God Nodes (most connected - your core abstractions)
1. `AppState` - 122 edges
2. `NovelRecord` - 84 edges
3. `SiteSetting` - 84 edges
4. `PersistentQueue` - 82 edges
5. `bindActions()` - 68 edges
6. `PushServer` - 68 edges
7. `NovelSettings` - 57 edges
8. `Device` - 55 edges
9. `NovelConverter` - 50 edges
10. `ApiResponse` - 46 edges

## Surprising Connections (you probably didn't know these)
- `send_mail_rejects_plain_smtp_without_explicit_opt_in()` --calls--> `send_mail()`  [INFERRED]
  tests/mail_e2e.rs → src/mail.rs
- `convert_sample_to_string()` --references--> `NovelConverter`  [EXTRACTED]
  tests/convert_parity.rs → src/converter/mod.rs
- `plain_smtp_setting()` --references--> `MailSetting`  [EXTRACTED]
  tests/mail_e2e.rs → src/mail.rs
- `runtime_description()` --calls--> `configure_hidden_console_command()`  [INFERRED]
  src/version.rs → src/compat.rs
- `send_mail_applies_attachment_filename_regex_replacement()` --calls--> `send_mail()`  [INFERRED]
  tests/mail_e2e.rs → src/mail.rs

## Import Cycles
- 1-file cycle: `src/main.rs -> src/main.rs`
- 1-file cycle: `src/commands/manage.rs -> src/commands/manage.rs`
- 1-file cycle: `src/commands/send.rs -> src/commands/send.rs`
- 1-file cycle: `src/compat.rs -> src/compat.rs`
- 1-file cycle: `src/converter/converter_base/stash_rebuild.rs -> src/converter/converter_base/stash_rebuild.rs`
- 1-file cycle: `src/converter/device.rs -> src/converter/device.rs`
- 1-file cycle: `src/commands/update.rs -> src/commands/update.rs`
- 1-file cycle: `src/converter/render.rs -> src/converter/render.rs`
- 1-file cycle: `src/converter/settings.rs -> src/converter/settings.rs`
- 1-file cycle: `src/converter/inspector.rs -> src/converter/inspector.rs`
- 1-file cycle: `src/mail.rs -> src/mail.rs`
- 1-file cycle: `src/commands/convert.rs -> src/commands/convert.rs`
- 1-file cycle: `src/converter/mod.rs -> src/converter/mod.rs`
- 1-file cycle: `src/converter/output.rs -> src/converter/output.rs`
- 1-file cycle: `src/db/database.rs -> src/db/database.rs`
- 1-file cycle: `src/downloader/mod.rs -> src/downloader/mod.rs`
- 1-file cycle: `src/title.rs -> src/title.rs`
- 1-file cycle: `src/commands/alias.rs -> src/commands/alias.rs`
- 1-file cycle: `src/db/inventory.rs -> src/db/inventory.rs`
- 1-file cycle: `src/commands/mod.rs -> src/commands/mod.rs`

## Communities (317 total, 156 thin omitted)

### Community 0 - "jobs.rs"
Cohesion: 0.06
Nodes (82): api_backup(), api_backup_bookmark(), api_cancel(), api_convert(), api_csv_download(), api_csv_import(), api_diff(), api_diff_clean() (+74 more)

### Community 1 - "illustration_store.rs"
Cohesion: 0.06
Nodes (55): fetch_illustration_bytes(), find_saved_section_illustration_filename(), apply_extension_fixes(), apply_extension_fixes_renames_files(), apply_legacy_illustration_migrations(), apply_legacy_migrations_renames_files_and_updates_cache(), CACHE_FILE_NAME, cache_path() (+47 more)

### Community 2 - "downloader/mod.rs"
Cohesion: 0.04
Nodes (50): create_cache_dir(), DatabaseGuard, date_string_is_newer(), date_string_is_newer_with_timezone(), date_string_to_ymd(), date_string_to_ymd_with_timezone(), DEFAULT_SITE_TIMEZONE, default_site_timezone() (+42 more)

### Community 3 - "converter/mod.rs"
Cohesion: 0.07
Nodes (40): CacheEntry, CacheSettingsSignature, clear_section_convert_cache(), clear_section_convert_cache_removes_per_novel_cache_and_lock(), convert_novel_keeps_localized_illustration_annotation(), convert_text_file_records_enchant_midashi_recommendation(), html_sections_keep_explicit_ruby(), html_sections_keep_parenthesized_kana_literal() (+32 more)

### Community 4 - "path"
Cohesion: 0.06
Nodes (38): cast_ini_value(), IniData, IniValue, Boolean, Float, Integer, Null, String (+30 more)

### Community 5 - "compat.rs"
Cohesion: 0.05
Nodes (57): add_directory_to_zip(), apply_auto_convert_device_settings(), collect_backup_files(), configure_hidden_console_command(), configure_process_group_command(), configure_process_group_command_child_survives_parent_session(), configure_process_group_command_creates_new_session(), configure_web_subprocess_command() (+49 more)

### Community 6 - "render.js"
Cohesion: 0.06
Nodes (67): persistListState(), showTagColorMenu(), applyFilterFromClick(), assertQueueActionSuccess(), bindRowSelection(), buildStructuredFilter(), clearSelection(), createRow() (+59 more)

### Community 7 - "actions.js"
Cohesion: 0.07
Nodes (68): addTagFromInput(), applyBulkTagEdit(), applyColumnVisibility(), applyNotepadSnapshot(), assertApiSuccess(), assertHistoryPayload(), batchAction(), bindActions() (+60 more)

### Community 8 - "worker.rs"
Cohesion: 0.05
Nodes (41): QueueExecutionSpec, WEBUI_MESSAGE_TEXT_META_KEY, WEBUI_MESSAGE_TYPE_META_KEY, WEBUI_UPDATE_START_MESSAGE_TYPE, append_update_args(), append_update_by_tag_args(), console_target_for_job(), convert_targets_and_device() (+33 more)

### Community 9 - "SiteSetting"
Cohesion: 0.05
Nodes (22): over18_access_decision(), Over18AccessDecision, Allow, Deny, Prompt, requires_over18_confirmation(), SiteSetting, load_all_from_dirs() (+14 more)

### Community 10 - "diff.rs"
Cohesion: 0.07
Nodes (52): bold_cyan(), bold_green(), bold_red(), bold_yellow(), build_diff_texts(), cache_root_dir(), clean_all_diff(), clean_diff() (+44 more)

### Community 11 - "novels.rs"
Cohesion: 0.09
Nodes (39): with_database(), with_database_mut(), read_sorted_cache_dirs(), read_sorted_section_files(), render_diff_list_html_for_target(), resolve_existing_id_for_target(), ANNOTATION_COLOR_TIME_LIMIT_SECS, api_list() (+31 more)

### Community 12 - "convert.rs"
Cohesion: 0.07
Nodes (37): add_dc_subject_to_epub(), apply_dc_subjects_if_needed(), apply_device_related_settings(), build_output_filename(), cmd_convert(), convert_text_target(), copy_to_result_skips_when_device_is_none(), create_ibunko_epub_output() (+29 more)

### Community 13 - "main.js"
Cohesion: 0.10
Nodes (52): initElements(), appendConsole(), appendConsoleRubyBaseText(), appendSanitizedConsoleHtml(), applyPerformanceMode(), applyWebConfig(), bindConsoleAutoScroll(), clearAllProgressBars() (+44 more)

### Community 14 - ".convert_for_all_data"
Cohesion: 0.06
Nodes (14): ConverterBase, ConverterBase, ConverterBase, ConverterBase, decode_stash_index(), encode_stash_token(), ILLUST_STASH_MARKER, replace_illust_tag_removes_annotation_when_disabled() (+6 more)

### Community 15 - "web/mod.rs"
Cohesion: 0.05
Nodes (28): create_router(), host_allowed_ignores_unsafe_wildcard_patterns(), host_allowed_supports_safe_wildcard_patterns(), INTERNAL_CONTROL_HEADER, is_symlink_like(), MAX_WEB_CSV_IMPORT_BYTES, MAX_WEB_LOG_COUNT, MAX_WEB_PAGE_LENGTH (+20 more)

### Community 16 - "progress.rs"
Cohesion: 0.06
Nodes (10): CliProgress, current_web_progress_scope(), emit_novel_refresh(), emit_web_event(), NoProgress, safe_println(), STDOUT_LOCK, WEB_PROGRESS_SCOPE_ENV (+2 more)

### Community 17 - "send.rs"
Cohesion: 0.08
Nodes (31): backup_bookmark_files(), bookmark_backup_copies_sdr_structure(), bookmark_storage_path(), cmd_send(), cmd_send_inner(), collect_all_targets(), collect_files_matching(), collect_files_matching_inner() (+23 more)

### Community 18 - "help.rs"
Cohesion: 0.08
Nodes (42): ALIAS_HELP, ALL_COMMAND_HELP, BACKUP_HELP, BROWSER_HELP, CLEAN_HELP, cmd_help(), CmdHelp, CmdInfo (+34 more)

### Community 19 - "character_conversion.rs"
Cohesion: 0.07
Nodes (28): ascii_letters_to_fullwidth(), calc_kanji_num_with_unit(), calc_sum_unit(), digits_to_kanji(), has_ascii_alpha(), has_trailing_kanji_zeros(), is_halfwidth_katakana(), is_kanji_digit() (+20 more)

### Community 20 - ".new"
Cohesion: 0.11
Nodes (35): clear_non_running_preserves_active_running_tasks(), clear_preserves_non_cancelled_history(), clear_saves_without_relocking_deadlock(), complete_can_drain_deferred_running_entries(), complete_preserves_jobs_enqueued_by_external_queue_instance(), completed_and_failed_history_survive_reload(), completed_backup_sentinel_skips_restore_rerun(), defer_restorable_tasks_requeues_running_on_disk() (+27 more)

### Community 21 - "device.rs"
Cohesion: 0.07
Nodes (27): aozora_output_summary_includes_stdout_and_stderr(), aozora_output_summary_is_empty_when_nothing_captured(), aozora_output_summary_omits_empty_streams(), aozora_temp_workspace_copies_assets_for_risky_paths(), aozora_temp_workspace_is_used_for_windows_31j_unencodable_paths(), AozoraInvocation, build_aozora_output_summary(), copy_aozora_companion_files() (+19 more)

### Community 22 - "logger.rs"
Cohesion: 0.08
Nodes (29): apply_log_postfix(), convert_commands_use_convert_log_postfix_when_concurrency_is_enabled(), DEFAULT_LOG_FORMAT_FILENAME, DEFAULT_LOG_FORMAT_TIMESTAMP, embed_timestamp(), emit(), emit_stderr(), emit_stdout() (+21 more)

### Community 23 - "dakuten_font.rs"
Cohesion: 0.09
Nodes (35): load_global_setting_value(), activate(), activate_writes_dakuten_css_and_font(), capture_warnings(), cleanup_errors(), combine_cleanup_errors(), current_line_height(), DAKUTEN_CSS_NAME (+27 more)

### Community 24 - "sort_state.rs"
Cohesion: 0.09
Nodes (34): current_sort_display_string(), current_web_update_sort_key_for_cli(), current_web_update_sort_state(), requested_sort_display_string(), sample_record(), sort_records_for_web_update(), sort_records_for_web_update_matches_general_lastup_descending(), sorted_update_all_ids() (+26 more)

### Community 25 - "NovelSettings"
Cohesion: 0.08
Nodes (16): ConverterBase, TextType, Body, Chapter, Introduction, Postscript, Story, Subtitle (+8 more)

### Community 26 - "inventory.rs"
Cohesion: 0.08
Nodes (27): atomic_write_locked(), CACHE_MAX_SIZE, CACHE_TARGET_SIZE, CacheEntry, dirs_home(), env_lock(), eviction_keeps_protected_entries(), find_narou_root() (+19 more)

### Community 27 - "inspector.rs"
Cohesion: 0.08
Nodes (18): AUTO_INDENT_THRESHOLD_RATIO, BRACKETS_RETURN_COUNT_THRESHOLD, countup_return_in_brackets_reports_counts(), END_TOUTEN_COUNT_THRESHOLD, inspect_end_touten_conditions_reports_recommendation_when_many(), Inspector, is_ignore_indent_char(), LINE_LENGTH_THRESHOLD (+10 more)

### Community 28 - "commands/update.rs"
Cohesion: 0.07
Nodes (28): alias_to_target(), classify_narou_api_chunk(), classify_narou_api_chunk_accepts_gl_entries_without_title(), copy_to_hotentry_output(), expand_tag_targets(), extract_title_from_file_title(), FORCE_WAIT_SECS, HOTENTRY_DIR_NAME (+20 more)

### Community 29 - "OutputManager"
Cohesion: 0.09
Nodes (9): canonicalize_aozoraepub3_jar_dir(), canonicalize_existing_path(), resolve_java_command_path(), absolutize_path(), aozora_output_looks_generated(), file_contains_dakuten_chuki(), move_aozora_output(), normalize_windows_verbatim_path() (+1 more)

### Community 30 - "push.rs"
Cohesion: 0.08
Nodes (19): ClientFilter, create_push_router(), handle_socket(), MAX_WS_CLIENTS, MAX_WS_FRAME_SIZE, MAX_WS_HISTORY_BYTES, MAX_WS_HISTORY_LINES, MAX_WS_MESSAGE_SIZE (+11 more)

### Community 31 - "queue.rs"
Cohesion: 0.14
Nodes (28): build_legacy_task(), build_stored_job(), configured_max_retries(), DEFAULT_MAX_RETRIES, flatten_value_into(), flatten_values(), job_needs_backup_completion_sentinel(), legacy_cmd_to_job_type_and_target() (+20 more)

### Community 32 - "fetch.rs"
Cohesion: 0.08
Nodes (14): build_reqwest_client(), curl_404_is_not_found(), curl_redirect_is_not_returned_as_success(), default_request_headers(), DEFAULT_RETRY_DELAYS, domain_of(), FAIL_THRESHOLD, fetch_text_does_not_retry_not_found() (+6 more)

### Community 33 - "toc.rs"
Cohesion: 0.10
Nodes (17): delete_ruby_tag(), slim_subtitle(), akatsuki_multipage_toc_fetches_all_pages(), akatsuki_toc_pattern_extracts_sections_with_nbsp_indent(), create_short_story_subtitles(), hameln_episode_list_parses_ruby_titles(), MockProgress, multipage_toc_uses_existing_progress_for_long_series() (+9 more)

### Community 34 - "PushServer"
Cohesion: 0.09
Nodes (10): BroadcastChannel, ConsoleHistoryEntry, ConsoleHistoryState, default_history_target_console(), history_body(), history_entry_from_value(), history_type_replayable(), message_scope() (+2 more)

### Community 35 - "render.rs"
Cohesion: 0.10
Nodes (25): add_date_to_title(), append_illustration_annotations(), calc_reverse_short_time(), create_cover_chuki(), decorate_title(), decorate_title_applies_left_align_end_marker_and_ruby_escape(), decorate_title_replaces_all_narou_rb_extended_format_symbols(), extract_illustration_annotations() (+17 more)

### Community 36 - "text_normalization.rs"
Cohesion: 0.09
Nodes (23): append_word_token(), insert_char_separator(), insert_word_separator(), is_blank_line(), is_border_symbol(), is_opening_no_separator(), is_separator_symbol(), is_word_connector() (+15 more)

### Community 37 - "xhtml.rs"
Cohesion: 0.10
Nodes (21): escape_text(), find_char(), find_seq(), find_sub(), honors_range_indent_amount(), inline_open_tag(), nav_binds_epub_namespace(), NavEntry (+13 more)

### Community 38 - "src/mail.rs"
Cohesion: 0.09
Nodes (16): attachment_name_applies_regex_replacement(), attachment_name_falls_back_to_id_when_path_has_no_name(), attachment_name_keeps_original_filename(), attachment_name_reports_invalid_regex(), build_message(), build_message_applies_reply_to_cc_and_bcc_headers(), guess_attachment_content_type(), MAIL_ATTACHMENT_PATTERN_KEYS (+8 more)

### Community 39 - "download.rs"
Cohesion: 0.12
Nodes (27): after_process(), auto_convert(), auto_convert_existing_download(), auto_convert_via_web_subprocess(), cmd_download(), cmd_download_inner(), confirm(), download_auto_convert_uses_shared_auto_convert_device_resolution() (+19 more)

### Community 40 - "scheduler.rs"
Cohesion: 0.10
Nodes (18): sort_keys(), sort_keys_lists_every_supported_key(), auto_update_echo(), AUTO_UPDATE_LAST_RUN_KEY, auto_update_sort_key_from_value(), auto_update_target_split_skips_frozen_records(), build_auto_update_sort_args(), collect_auto_update_target_ids() (+10 more)

### Community 41 - ".download_novel_with_force"
Cohesion: 0.13
Nodes (7): Downloader, load_local_setting_bool(), ncode_target_url(), section_hash_cache_store_and_clear_roundtrip(), compute_section_hash(), load_section_file(), is_safe_public_url()

### Community 42 - "tag_colors.rs"
Cohesion: 0.11
Nodes (20): configured_new_tag_color(), ensure_tag_colors(), ensure_tag_colors_with_default_color(), ensure_tag_colors_with_default_color_inside_with_database_does_not_deadlock(), is_valid_new_tag_color_value(), is_valid_tag_color(), load_tag_colors(), load_tag_colors_preserves_unknown_colors() (+12 more)

### Community 43 - "frontend.rs"
Cohesion: 0.14
Nodes (23): about_page(), all_versioned_assets_are_served(), apply_asset_versions(), apply_asset_versions_appends_query_hashes_to_known_assets(), apply_js_module_versions(), asset(), author_comments_page(), bookmarklet_page() (+15 more)

### Community 44 - "PersistentQueue"
Cohesion: 0.11
Nodes (5): atomic_write(), has_deferred_jobs(), normalize_deferred_pending_flag(), PersistentQueue, startup_tasks_stay_deferred_until_explicit_restore()

### Community 45 - "manage.rs"
Cohesion: 0.10
Nodes (23): ANNOTATION_COLOR_TIME_LIMIT, build_remove_confirm_message(), cmd_freeze(), cmd_remove(), collect_all_short_story_ids(), decorate_date(), DecoratedNovel, FILTER_TYPE_HELP (+15 more)

### Community 46 - "misc.rs"
Cohesion: 0.19
Nodes (25): all_novel_ids(), clear_history(), console_history(), get_sort_state(), HistoryParams, html_escape(), normalize_version(), notepad_object_id() (+17 more)

### Community 47 - "NovelRecord"
Cohesion: 0.10
Nodes (5): Database, deserialize_nilable_bool(), NovelRecord, .RAW_TITLE_KEY, raw_title_uses_compatible_flattened_field()

### Community 48 - "setting.rs"
Cohesion: 0.13
Nodes (20): tagname_to_ids(), burn_default_settings(), cast_value(), cmd_setting(), cmd_setting_inner(), display_help(), display_variable_list(), format_yaml_value() (+12 more)

### Community 49 - "paths.rs"
Cohesion: 0.12
Nodes (20): sort_key_valid(), DATABASE, absolute_normalized_path(), canonical_existing_path(), create_subdirectory_name(), ensure_within_archive_root(), ensure_within_archive_root_rejects_lexical_escape(), ensure_within_archive_root_returns_canonical_absolute_path() (+12 more)

### Community 50 - "settings.js"
Cohesion: 0.16
Nodes (29): applyLoadedTheme(), applyThemeValue(), bindEvents(), collectFormData(), escapeAttr(), escapeHtml(), fetchSettingsData(), findSetting() (+21 more)

### Community 51 - "updater.rs"
Cohesion: 0.18
Nodes (21): apply_update(), apply_update_and_remove_download(), Args, backup_path_for(), BackupEntry, cleanup_backups(), copy_tree_overwrite(), copy_tree_overwrite_replaces_existing_files() (+13 more)

### Community 52 - "process_novel_for_update"
Cohesion: 0.11
Nodes (26): abort_if_interrupted(), clear_convert_failure(), cmd_update(), collect_record_domains(), get_novel_title(), load_setting_float(), merge_cli_and_stdin_targets(), ParallelUpdateOutcome (+18 more)

### Community 53 - "feature_tour.rs"
Cohesion: 0.15
Nodes (23): all(), compare_versions(), configure(), current_entries(), DISABLED_KEY, FEATURE_TOURS, FeatureTourEntry, is_known_tour_version() (+15 more)

### Community 54 - "global_settings.rs"
Cohesion: 0.12
Nodes (15): resolve_sort_key(), tab_for_setting(), webui_help_override(), auto_schedule_snapshot(), build_setting_entry(), coerce_float_setting_from_string(), get_global_settings(), is_live_webui_config_name() (+7 more)

### Community 55 - "src/version.rs"
Cohesion: 0.11
Nodes (22): aozoraepub3_jar_from_global_setting(), aozoraepub3_jar_from_narou_root(), aozoraepub3_jar_next_to_exe(), aozoraepub3_jar_path(), commit_version_exists(), create_version_string(), DISABLE_SELF_UPDATE_ENV, global_setting_path() (+14 more)

### Community 56 - "database.rs"
Cohesion: 0.17
Nodes (17): ARCHIVE_ROOT_DIR, compare_optional(), compare_records_by_key(), DATABASE_NAME, database_parity_create_new_id_starts_at_zero(), database_parity_save_preserves_unknown_fields_and_zero_id(), record_status_key(), record_tags_key() (+9 more)

### Community 57 - "web.rs"
Cohesion: 0.10
Nodes (8): default_ws_accepted_domains(), load_http_allowed_request_hosts(), load_ws_accepted_domains(), parse_extra_allowed_hosts(), parse_extra_allowed_hosts_drops_unsafe_wildcards(), parse_extra_allowed_hosts_preserves_safe_wildcard(), parse_extra_allowed_hosts_splits_and_trims(), wildcard_bind_defaults_do_not_accept_arbitrary_ws_domains()

### Community 58 - "clean.rs"
Cohesion: 0.15
Nodes (19): build_vote_target_url(), build_vote_url(), cmd_browser(), cmd_browser_inner(), resolve_target_urls(), clean_all(), clean_detects_orphan_raw_and_section_files(), clean_novel_dir() (+11 more)

### Community 59 - "info_extraction.rs"
Cohesion: 0.14
Nodes (15): build_standard_regex(), capture_name_matches_key(), fancy_regex_allowed(), multi_match_decodes_html_entities_in_story_capture(), SiteSetting, test_multiline_tag_capture_collects_all_matches(), test_ncode_tags_regex_match(), try_regex_captures() (+7 more)

### Community 60 - "web/update.rs"
Cohesion: 0.13
Nodes (15): api_update_start(), append_update_session_marker(), build_restart_args(), build_restart_args_adds_no_browser(), current_asset_name(), current_asset_name_known_combinations(), download_to_file(), _ensure_response_impl() (+7 more)

### Community 61 - "Commands"
Cohesion: 0.08
Nodes (25): Commands, Alias, Backup, Browser, Clean, Convert, Csv, Diff (+17 more)

### Community 62 - "init.rs"
Cohesion: 0.16
Nodes (21): ask_aozoraepub3_path(), ask_line_height(), bundled_webnovel_dir(), cmd_init(), copy_bundled_webnovel_files(), ensure_dot_narou_files(), find_existing_narou_root(), format_line_height() (+13 more)

### Community 63 - "setting_core.rs"
Cohesion: 0.18
Nodes (18): apply_device_related_settings(), cast_setting_value(), cast_value_for_type(), coerce_bool_value(), coerce_directory_value(), coerce_float_value(), coerce_integer_value(), coerce_json_setting_value() (+10 more)

### Community 64 - "ruby_time.rs"
Cohesion: 0.14
Nodes (7): deserialize(), format_ruby_time(), jst_offset(), OptionalTime, parse_ruby_time(), RequiredTime, serialize()

### Community 65 - "hashmap"
Cohesion: 0.11
Nodes (11): BANNED_ALIAS_NAME, cmd_alias(), cmd_alias_inner(), display_aliases(), is_valid_alias_name(), process_alias_arg(), resolve_alias_title(), resolve_alias_target() (+3 more)

### Community 66 - "cli.rs"
Cohesion: 0.16
Nodes (21): apply_multiple(), COMMAND_NAMES, convert_make_zip_is_injected_from_local_setting(), convert_no_epub_is_injected_from_local_setting(), convert_no_mobi_is_injected_from_local_setting(), convert_no_open_is_injected_from_local_setting(), convert_no_strip_is_injected_from_local_setting(), convert_no_zip_is_injected_from_local_setting() (+13 more)

### Community 67 - "main.rs"
Cohesion: 0.15
Nodes (13): Cli, install_panic_hook(), last_panic_detail(), main(), panic_detail_message(), panic_payload_message(), parse_stdin_targets(), prepare_windows_console() (+5 more)

### Community 68 - "mail_e2e.rs"
Cohesion: 0.20
Nodes (13): send_mail(), find_attachment(), header_text(), percent_encode_utf8(), plain_smtp_setting(), run_smtp_session(), send_mail_applies_attachment_filename_regex_replacement(), send_mail_delivers_message_via_plain_smtp() (+5 more)

### Community 69 - "Stmt"
Cohesion: 0.13
Nodes (17): Stmt, Emit, For, Guard, If, InsertAtMatch, Let, Set (+9 more)

### Community 70 - "fs"
Cohesion: 0.17
Nodes (13): build_command(), build_log(), LOG_FORMAT_TIMESTAMP, LOG_NAME, log_path(), log_path_falls_back_to_cwd_without_root(), log_path_uses_root_dir_when_initialized(), panic_message() (+5 more)

### Community 71 - "rate_limit.rs"
Cohesion: 0.13
Nodes (10): DEFAULT_INTERVAL_SECS, GLOBAL_HOST_KEY, HostRateLimitState, load_interval_secs(), load_wait_steps(), normalize_wait_steps(), RateLimitState, STATE (+2 more)

### Community 72 - "HttpFetcher"
Cohesion: 0.23
Nodes (7): decode_with_encoding(), ensure_success_status(), HttpFetcher, io_error(), read_response_bytes(), same_site_hosts(), is_safe_header_value()

### Community 73 - "cargo-local-build.rs"
Cohesion: 0.20
Nodes (14): build_app_binary(), build_release_binary(), cargo_build_command(), copy_dir_recursive(), copy_file(), create_local_package(), file_name(), help_text() (+6 more)

### Community 74 - "log.rs"
Cohesion: 0.17
Nodes (10): cmd_log(), cmd_log_inner(), expand_tilde(), home_dir(), latest_log_path_ignores_hidden_files(), resolve_path(), tail_offset(), tail_offset_returns_last_two_lines() (+2 more)

### Community 75 - "setting_info.rs"
Cohesion: 0.17
Nodes (12): DEFAULT_ARG_COMMAND_NAMES, default_local_setting_value(), mail_attachment_filename_settings_are_visible_on_detail_tab(), mail_attachment_filename_webui_help_includes_usage_notes(), original_setting_var_infos(), queue_retry_settings_have_detail_tab_and_defaults(), server_add_accepted_hosts_has_global_tab(), setting_variables() (+4 more)

### Community 76 - "illust.rs"
Cohesion: 0.17
Nodes (18): cmd_illust(), cmd_illust_inner(), collect_all_novel_dirs(), find_orphan_illustrations_returns_only_unreachable_files(), IllustSubcommand, FixExt, Migrate, Orphan (+10 more)

### Community 77 - "IndexStore"
Cohesion: 0.19
Nodes (7): compute_fingerprint(), IndexStore, IndexStoreData, INVENTORY_NAME, MetaEntry, normalize_title(), normalize_url()

### Community 78 - "section.rs"
Cohesion: 0.18
Nodes (13): illustration_sources(), syosetu_org_illustration_pattern_uses_new_raw_html_without_crossing_links(), download_section(), hameln_section_patterns_extract_ruby_body_and_author_notes(), parse_section_html(), section_cache_key(), section_cache_key_uses_resolved_section_url_not_index(), SectionCache (+5 more)

### Community 79 - "QueueJob"
Cohesion: 0.11
Nodes (17): cancel_pending_in_lane_moves_matching_jobs_to_cancelled_history(), JobType, AutoUpdate, Backup, Convert, Download, Mail, Send (+9 more)

### Community 80 - "state.js"
Cohesion: 0.15
Nodes (18): El, ELEMENT_IDS, loadListState(), lsBool(), lsGet(), lsInt(), lsRemove(), lsSet() (+10 more)

### Community 81 - "preprocess/parser.rs"
Cohesion: 0.36
Nodes (16): build_access_part(), build_accessor(), build_block(), build_bracket_key(), build_expr(), build_lvalue(), build_method(), build_regex() (+8 more)

### Community 82 - "PreprocessResult"
Cohesion: 0.40
Nodes (9): cached_extract_json_regex(), Ctx, eval_expr(), eval_extract_json(), eval_method(), eval_stmt(), validate_array_len(), validate_string_size() (+1 more)

### Community 83 - "csv.rs"
Cohesion: 0.17
Nodes (13): cmd_csv(), cmd_csv_inner(), generate_csv(), HR_TEXT, import_csv(), load_import_csv_content(), load_import_csv_content_reads_regular_file(), output_csv() (+5 more)

### Community 84 - "epub/mod.rs"
Cohesion: 0.26
Nodes (15): build_epub(), builds_minimal_epub(), collect_illustrations(), embeds_dakuten_font_when_requested(), embeds_existing_illustration(), EpubOptions, excludes_illustration_when_disabled(), horizontal_mode_emits_ltr_and_hltr() (+7 more)

### Community 85 - "Inventory"
Cohesion: 0.26
Nodes (7): CacheKey, file_mtime(), Inventory, InventoryCache, InventoryScope, Global, Local

### Community 86 - ".try_register_client"
Cohesion: 0.20
Nodes (8): ClientHandle, ClientMessage, closed_clients_are_removed_on_next_dispatch(), concurrent_publish_preserves_history_id_order(), history_snapshot_ids_allow_live_dedup_without_dropping_control_events(), history_snapshot_preserves_message_types_and_structured_progress(), register_client_respects_max_clients_limit(), slow_clients_are_dropped_after_consecutive_queue_overflows()

### Community 87 - "state.rs"
Cohesion: 0.15
Nodes (14): api_taginfo(), html_escape(), tag_color_class(), ConfirmRunningTasksBody, ConvertBody, CsvImportBody, DiffBody, DiffCleanBody (+6 more)

### Community 88 - "interpreter.rs"
Cohesion: 0.19
Nodes (16): and_operator_short_circuits_on_falsy(), basic_string_interpolation_and_chain_access(), bracket_access_with_interpolated_key(), extract_json_not_found_returns_null_gracefully(), flat_map_flattens_single_level(), for_loop_iterates_and_emits_each_item(), guard_allows_processing_when_magic_word_absent(), guard_stops_reprocessing_when_magic_word_present() (+8 more)

### Community 89 - "request_guard_middleware"
Cohesion: 0.20
Nodes (13): authority_matches_state(), basic_auth_matches(), basic_auth_middleware(), constant_time_str_eq(), internal_control_token_matches(), origin_allowed(), origin_allowed_for_ports(), origin_matches_forwarded_host() (+5 more)

### Community 90 - "updater_promote.rs"
Cohesion: 0.20
Nodes (14): ACTIVE_UPDATER_FILENAME, EMBEDDED_UPDATER_SHA256, empty_expected_hash_skips(), hex_eq_ci(), PENDING_UPDATER_FILENAME, pending_with_correct_hash_replaces_active(), pending_with_wrong_hash_is_left_untouched(), pending_without_existing_active_is_promoted() (+6 more)

### Community 91 - "SubtitleInfo"
Cohesion: 0.25
Nodes (16): move_to_cache_dir(), section_relative_path(), ensure_default_files(), find_section_file_by_index(), infer_archive_root(), move_file_to_dir(), raw_filename(), resolve_section_file_path() (+8 more)

### Community 92 - "novel_info.rs"
Cohesion: 0.25
Nodes (8): fill_missing_from_does_not_overwrite_existing_values(), NovelInfo, parse_narou_date(), parse_narou_date_accepts_japanese_datetime_with_weekday(), parse_narou_date_accepts_kakuyomu_rfc3339(), parse_narou_date_with_timezone(), partial_detail_parse_still_backfills_missing_author_from_toc(), syosetu_org_title_falls_back_to_toc_when_detail_page_blocked()

### Community 93 - "ast.rs"
Cohesion: 0.17
Nodes (16): Accessor, AccessPart, Bracket, Dot, BracketKey, Accessor, Str, is_truthy() (+8 more)

### Community 94 - "context_menu.js"
Cohesion: 0.17
Nodes (17): openMenuStyleModal(), actionHandlers, changeTagColor(), createDefaultMenuText(), DEFAULT_COMMANDS, getStorageValue(), getStoredMenuStyle(), getStoredMenuText() (+9 more)

### Community 95 - "start_queue_worker_for_lane"
Cohesion: 0.16
Nodes (16): clear_progress_for_job(), clear_progress_for_job_targets_both_consoles_with_scope(), compute_retry_backoff_secs(), load_retry_backoff_schedule(), parse_backoff_spec(), pop_next_job(), pop_next_job_blocks_secondary_lane_convert_for_novel_running_on_default(), register_running_job() (+8 more)

### Community 96 - "security.rs"
Cohesion: 0.14
Nodes (10): CONNECT_TIMEOUT_SECS, is_safe_public_ip(), is_safe_public_ipv4(), is_safe_public_ipv6(), MAX_REDIRECTS, MAX_REGEX_INPUT_LEN, MAX_RESPONSE_BYTES, MAX_YAML_REGEX_PATTERN_LEN (+2 more)

### Community 97 - "cmd_tag"
Cohesion: 0.15
Nodes (16): build_tag_mode(), build_tag_mode_rejects_banned_word(), cmd_tag(), display_tag_list(), get_tag_list(), normalize_color(), TagMode, Add (+8 more)

### Community 98 - "output.rs"
Cohesion: 0.24
Nodes (14): convert_filename_to_ncode(), create_output_text_filename(), create_output_text_filename_for_textfile(), create_output_text_path(), create_output_text_path_for_textfile(), default_output_basename(), ensure_txt_extension(), extract_domain() (+6 more)

### Community 99 - ".push_internal"
Cohesion: 0.24
Nodes (10): ensure_yaml_size_limit(), ensure_queue_capacity(), find_active_job_id(), generate_job_id(), load_queue_state(), PersistentQueueState, state_contains_job(), total_pending_len() (+2 more)

### Community 100 - "QueueLane"
Cohesion: 0.20
Nodes (7): cancel_pending_jobs_for_lane(), extract_novel_ids(), QueueLane, Default, Secondary, running_novel_ids_collects_from_active_and_deferred_running(), job_conflicts_with_running()

### Community 101 - "ApiResponse"
Cohesion: 0.47
Nodes (12): batch_freeze(), batch_freeze_toggle(), batch_remove(), batch_remove_with_file(), batch_tag(), batch_unfreeze(), batch_untag(), run_batch_cli() (+4 more)

### Community 102 - "web_tray.rs"
Cohesion: 0.16
Nodes (4): build_tray_icon(), run_message_loop(), run_web_tray(), spawn_web_tray()

### Community 103 - "Value"
Cohesion: 0.24
Nodes (13): build_transport(), build_transport_accepts_domain_and_login_authentication(), parse_symbolic_yaml_map(), smtp_accept_invalid_certs(), smtp_allow_insecure(), smtp_auth_mechanisms(), smtp_tls_mode(), yaml_bool() (+5 more)

### Community 104 - "strip_mobi_sources"
Cohesion: 0.24
Nodes (11): patch_range(), read_be_u16(), read_be_u32(), slice_from(), slice_range(), strip_mobi_file(), strip_mobi_sources(), strip_mobi_sources_rejects_non_mobi_input() (+3 more)

### Community 105 - "NovelInfoCache"
Cohesion: 0.22
Nodes (4): CachedNovelInfo, find_narou_root(), MAX_CACHE_ENTRIES, NovelInfoCache

### Community 106 - "RateLimiter"
Cohesion: 0.25
Nodes (8): default_interval_uses_ruby_compatible_value(), different_hosts_do_not_block_each_other(), host_key_from_url(), interval_lower_than_zero_is_clamped(), parallel_workers_on_different_domains_do_not_block(), RateLimiter, reset_state(), same_host_requests_keep_their_reserved_order()

### Community 107 - "tags.rs"
Cohesion: 0.40
Nodes (11): EditTagBody, TagBody, TagsBody, add_tag(), add_tags(), edit_tag(), remove_tag(), remove_tags() (+3 more)

### Community 108 - "narou_rs third-party license inventory"
Cohesion: 0.12
Nodes (16): Apache-2.0, 0BSD, BSD-2-Clause, BSD-3-Clause, bzip2-1.0.6, cargo-about regeneration, CC0-1.0, CDLA-Permissive-2.0 (+8 more)

### Community 109 - "commands/mail.rs"
Cohesion: 0.22
Nodes (8): cmd_mail(), cmd_mail_inner(), collect_all_targets(), expand_targets(), mail_interrupt_flag(), MAIL_INTERRUPT_FLAG, MailOptions, MAIL_INTERRUPTED_MESSAGE

### Community 110 - "load_mail_setting"
Cohesion: 0.19
Nodes (12): mail_hotentry_if_enabled(), ensure_mail_setting_file(), load_mail_setting(), mail_setting_path(), MailSettingLoadError, Incomplete, Io, NotFound (+4 more)

### Community 111 - "run_scheduled_auto_update"
Cohesion: 0.23
Nodes (12): calculate_next_run_time(), calculate_next_run_time_after(), calculate_next_run_time_after_uses_rounded_dst_gap(), calculate_next_run_time_returns_future_time(), load_last_auto_update_run(), local_datetime_in_timezone(), local_datetime_in_timezone_rounds_dst_gap_forward(), missed_run_time() (+4 more)

### Community 113 - "cmd_list_inner"
Cohesion: 0.27
Nodes (8): cmd_list(), cmd_list_inner(), decorate_line(), decorate_title(), ListOptions, output_list(), split_words(), valid_tags()

### Community 114 - "types.rs"
Cohesion: 0.15
Nodes (13): parse_narou_api_entries(), ARCHIVE_ROOT_DIR, CACHE_SAVE_DIR, MAX_SECTION_CACHE, NarouApiEntry, NarouApiResult, RAW_DATA_DIR, SECTION_SAVE_DIR (+5 more)

### Community 115 - "Expr"
Cohesion: 0.14
Nodes (14): Expr, Access, And, Array, Chain, Eq, ExtractJson, Ne (+6 more)

### Community 116 - "DigestChoice"
Cohesion: 0.18
Nodes (13): choose_digest_action(), choose_digest_action_inner(), choose_digest_action_with_auto_choices(), DigestChoice, Backup, Cancel, CancelAndFreeze, Convert (+5 more)

### Community 117 - "package.rs"
Cohesion: 0.24
Nodes (8): build_opf(), deterministic_urn_uuid(), ManifestItem, opf_has_version3_metadata_close_and_rtl(), OPF_ZIP_PATH, OpfMeta, uuid_is_deterministic_and_well_formed(), write_epub()

### Community 118 - "apply_general_lastup_check_result"
Cohesion: 0.27
Nodes (10): apply_general_lastup_check_result(), general_lastup_check_clears_modified_when_novelupdated_at_is_not_newer(), general_lastup_check_marks_modified_when_latest_published_episode_date_changes(), general_lastup_check_respects_existing_last_check_date(), general_lastup_check_uses_last_update_fallback_and_preserves_it(), GeneralLastupNarouOutcome, narou_api_datetime_is_interpreted_as_jst(), parse_api_datetime() (+2 more)

### Community 119 - "is_web_mode"
Cohesion: 0.23
Nodes (8): auto_convert(), auto_convert_via_web_subprocess(), relay_web_convert_stream(), relay_web_stream_to_console(), is_web_mode(), bold_colored(), color_css(), colored()

### Community 120 - "Block"
Cohesion: 0.24
Nodes (12): Block, Blank, CloseDiv, Heading, Hr, Line, OpenDiv, PageCenter (+4 more)

### Community 121 - "narou_api.rs"
Cohesion: 0.27
Nodes (8): narou_api_batch_update(), NAROU_API_DEFAULT_INTERVAL_SECS, NAROU_API_DEFAULT_USER_AGENT, narou_api_interval_secs(), narou_api_rate_limiter(), narou_api_user_agent(), parse_api_datetime(), parse_api_entries()

### Community 122 - "fetchJson"
Cohesion: 0.36
Nodes (10): fetchJson(), postJson(), request(), closeFeatureTour(), initFeatureTour(), maybeShowPendingFeatureTour(), openFeatureTour(), renderEntry() (+2 more)

### Community 123 - "write"
Cohesion: 0.25
Nodes (5): collect_assets(), configure_windows_uac_manifest(), is_versioned_asset(), main(), sha3_256_base64_url()

### Community 124 - "preprocess_args"
Cohesion: 0.18
Nodes (11): args_before_double_dash(), build_shortcuts(), diff_short_number_option_is_expanded(), download_targets_after_double_dash_are_not_treated_as_help(), expand_diff_short_number_options(), GlobalFlags, load_global_no_color(), preprocess_args() (+3 more)

### Community 125 - "inspect.rs"
Cohesion: 0.29
Nodes (8): cmd_inspect(), cmd_inspect_inner(), display_log(), HR_TEXT, InspectEntry, read_inspect_log(), resolve_inspect_entry(), INSPECT_LOG_NAME

### Community 126 - "commands/version.rs"
Cohesion: 0.40
Nodes (8): cmd_version(), java_command(), print_process_output(), print_stream(), run_aozora_command(), run_java_command(), version_more(), sanitize_java_command()

### Community 127 - "epub/parser.rs"
Cohesion: 0.33
Nodes (10): derive_nav_label(), Document, Page, PAGE_BREAK, parse_document(), parses_blank_and_hr(), parses_headings_with_indent_prefix(), parses_inline_jizuke_line() (+2 more)

### Community 128 - "DivKind"
Cohesion: 0.22
Nodes (11): between(), DivKind, Afterword, Indent, Introduction, Jizuke, Small, match_close_div() (+3 more)

### Community 129 - "Method"
Cohesion: 0.18
Nodes (11): Method, Compact, Empty, FlatMap, Flatten, Gsub, IsArray, Join (+3 more)

### Community 132 - "super"
Cohesion: 0.27
Nodes (6): book_style_css(), css_contains_vertical_and_line_height(), css_embeds_dmincho_for_dakuten_without_body_embed(), css_includes_font_face_when_embedding(), DMINCHO_TTF, FONT_FILE

### Community 133 - "send_target_with_setting_interruptible"
Cohesion: 0.29
Nodes (9): alias_to_target(), current_device_ext(), newest_hotentry_file_path(), resolve_record(), send_hotentry(), send_mail_with_progress(), send_mail_with_progress_returns_interrupt_message(), send_target_with_setting_interruptible() (+1 more)

### Community 134 - "title.rs"
Cohesion: 0.33
Nodes (6): DatabaseGuard, project_title(), rename_projected_outputs(), strip_title_prefix(), sync_projection_updates_display_data_and_preserves_raw_title(), sync_title_projection()

### Community 135 - "start_auto_update_scheduler"
Cohesion: 0.31
Nodes (8): parse_schedule_times(), queue_auto_update_job_if_needed(), queue_auto_update_job_reuses_pending_job(), queue_auto_update_job_reuses_running_job(), restart_auto_update_scheduler(), start_auto_update_scheduler(), start_or_restart_auto_update_scheduler(), stop_auto_update_scheduler()

### Community 136 - "bind_or_shutdown_and_retry"
Cohesion: 0.33
Nodes (7): bind_or_shutdown_and_retry(), control_request_host(), create_reusable_listener(), read_pid_file(), send_control_request(), try_kill_via_pid_file(), try_shutdown_via_http()

### Community 137 - "update_general_lastup"
Cohesion: 0.22
Nodes (9): count_general_lastup_narou_targets(), general_lastup_progress(), initial_general_lastup_work_units(), is_novel_frozen(), load_frozen_ids(), partition_novels_by_api_support(), sync_end_tag(), update_general_lastup() (+1 more)

### Community 138 - "Value"
Cohesion: 0.39
Nodes (8): basic_auth_header_from_settings(), encode_base64(), load_web_security_settings(), require_basic_auth_for_external_bind_from_settings(), reverse_proxy_mode_from_settings(), WebSecuritySettings, yaml_bool(), yaml_string()

### Community 139 - "run_web_server"
Cohesion: 0.25
Nodes (9): confirm_first_web_boot(), control_token_generation_is_non_empty(), fill_general_all_no_in_database(), generate_control_token(), pid_file_path(), remove_pid_file(), requires_basic_auth_for_bind(), run_web_server() (+1 more)

### Community 140 - ".new"
Cohesion: 0.33
Nodes (5): accepts_origin_allows_ip_literals_only_when_enabled(), accepts_origin_respects_wildcards(), ws_request_accepts_same_origin_proxy_mode(), ws_request_accepts_same_origin_ws_port(), ws_request_requires_basic_auth_and_valid_origin()

### Community 141 - "pathbuf"
Cohesion: 0.39
Nodes (3): CurrentDirGuard, cwd_lock(), set_current_dir_for_test()

### Community 142 - "convert_parity.rs"
Cohesion: 0.32
Nodes (3): convert_sample_to_string(), find_reference_output(), kakuyomu_sample_matches_narou_rb_reference_byte_for_byte()

### Community 143 - "gaiji.rs"
Cohesion: 0.32
Nodes (4): extract_menku(), FALLBACK_CHAR, resolve_menku(), resolve_named()

### Community 144 - "VarType"
Cohesion: 0.25
Nodes (8): VarType, Boolean, Directory, Float, Integer, Multiple, Select, String

### Community 145 - "host_allowed"
Cohesion: 0.29
Nodes (8): current_hostname(), default_allowed_request_hosts(), host_allowed(), is_exact_subdomain_wildcard_match(), is_safe_wildcard_pattern(), normalize_host_name(), wildcard_host_match(), wildcard_host_match_bytes()

### Community 146 - "JobOutcome"
Cohesion: 0.29
Nodes (6): classify_job_outcome(), JobOutcome, Cancelled, Completed, Failed, Partial

### Community 147 - "narou_rs Web novel CLI and Web UI"
Cohesion: 0.29
Nodes (7): COMMANDS.md detailed compatibility status, Per-directory .narou data and global settings, Release bundle keeps binaries, webnovel and preset together, narou_rs Web novel CLI and Web UI, narou.rb compatibility, External Web exposure requires authentication; reverse proxy opt-in, Narou, Hameln, Arcadia, Akatsuki and Kakuyomu site definitions

### Community 148 - "smtp_tls"
Cohesion: 0.43
Nodes (6): smtp_tls(), SmtpTlsMode, None, Opportunistic, Required, Wrapper

### Community 149 - "setting_scope"
Cohesion: 0.33
Nodes (5): get_scope_of_variable_name(), setting_scope(), SettingScope, Global, Local

### Community 150 - "paint"
Cohesion: 0.33
Nodes (6): decorate_annotation(), decorate_id(), decorate_tags(), paint(), render_tag_count(), render_tags()

### Community 151 - "resolve_web_address"
Cohesion: 0.33
Nodes (6): can_bind(), find_available_web_port(), normalize_bind_host(), resolve_web_address(), WebAddress, yaml_u16()

### Community 152 - "MailSetting"
Cohesion: 0.60
Nodes (6): attachment_name(), mail_attachment_pattern(), mail_attachment_replacement(), mail_extra_string(), MailSetting, original_attachment_name()

### Community 153 - "narou.rs project runtime kernel"
Cohesion: 0.40
Nodes (5): docs/ARCHITECTURE.md, COMMANDS.md implementation status, Externally observable narou.rb compatibility, narou.rs project runtime kernel, OpenSpec for nontrivial changes

### Community 154 - "IllustSubcommand"
Cohesion: 0.40
Nodes (5): IllustSubcommand, FixExt, Migrate, Orphan, Rebuild

### Community 155 - "reroute_web_line_to_console"
Cohesion: 0.40
Nodes (5): reroute_web_convert_line_retargets_structured_events_to_stdout2(), reroute_web_convert_line_wraps_plain_text_for_stdout2(), reroute_web_line_to_console(), reroute_web_line_to_console_retargets_structured_messages(), reroute_web_line_to_console_wraps_plain_text_for_requested_console()

### Community 156 - "i18n.js"
Cohesion: 0.50
Nodes (3): applyI18n(), DICT, toggleLanguage()

### Community 157 - "format_queue_job_target"
Cohesion: 0.50
Nodes (5): describe_update_targets(), format_general_lastup_queue_target(), format_queue_job_target(), format_update_queue_target(), queue_target_fallback_text()

### Community 158 - "Persistent queue with separate network and local lanes"
Cohesion: 0.50
Nodes (4): External bind Basic auth guard and optional reverse-proxy mode, Persistent queue with separate network and local lanes, Retry transient jobs using available_at and configured backoff, Web UI server and API

### Community 161 - "build_domain_groups"
Cohesion: 0.50
Nodes (3): build_domain_groups(), build_domain_groups_collapses_single_domain(), build_domain_groups_partitions_by_domain()

### Community 162 - "HeadingLevel"
Cohesion: 0.50
Nodes (4): HeadingLevel, Large, Medium, Sub

### Community 163 - "output_macros.rs"
Cohesion: 0.50
Nodes (3): eprintln!, print!, println!

### Community 164 - "convert generates ebook formats and conversion inspection logs"
Cohesion: 0.67
Nodes (3): convert generates ebook formats and conversion inspection logs, Native vertical EPUB3 generation, Hash-based illustration cache with mitemin-compatible IDs

### Community 165 - "Hameln site definition"
Cohesion: 0.67
Nodes (3): syosetu.org and h.syosetu.org captured-domain URL matching, Layered TOC, body, metadata, tags and revision extraction, Hameln site definition

## Knowledge Gaps
- **352 isolated node(s):** `narou_rs`, `Alias`, `Backup`, `Browser`, `Clean` (+347 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1063 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **156 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `NovelRecord` connect `NovelRecord` to `jobs.rs`, `downloader/mod.rs`, `compat.rs`, `send_target_with_setting_interruptible`, `title.rs`, `diff.rs`, `novels.rs`, `web/mod.rs`, `send.rs`, `sort_state.rs`, `commands/update.rs`, `fetch.rs`, `render.rs`, `src/mail.rs`, `download.rs`, `scheduler.rs`, `manage.rs`, `paths.rs`, `database.rs`, `clean.rs`, `hashmap`, `IndexStore`, `cmd_list_inner`, `apply_general_lastup_check_result`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Are the 11 inferred relationships involving `bindActions()` (e.g. with `addTagFromInput()` and `hideRegisterModal()`) actually correct?**
  _`bindActions()` has 11 INFERRED edges - model-reasoned connections that need verification._
- **What connects `narou_rs`, `Alias`, `Backup` to the rest of the system?**
  _352 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `jobs.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05589194224499301 - nodes in this community are weakly interconnected._
- **Why does `NovelSettings` connect `NovelSettings` to `output.rs`, `converter/mod.rs`, `text_normalization.rs`, `compat.rs`, `render.rs`, `path`, `src/mail.rs`, `convert.rs`, `.convert_for_all_data`, `convert_parity.rs`, `setting.rs`, `character_conversion.rs`, `inspector.rs`, `commands/update.rs`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Should `illustration_store.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06101231190150479 - nodes in this community are weakly interconnected._
- **Why does `Downloader` connect `.download_novel_with_force` to `toc.rs`, `downloader/mod.rs`, `download.rs`, `HttpFetcher`, `SiteSetting`, `section.rs`, `NovelRecord`, `process_novel_for_update`, `commands/update.rs`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._