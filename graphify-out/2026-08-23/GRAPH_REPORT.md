# Graph Report - My-House  (2026-08-23)

## Corpus Check
- 170 files · ~81,746 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1117 nodes · 2068 edges · 115 communities (93 shown, 22 thin omitted)
- Extraction: 95% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 91 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f154f000`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- components/index.ts
- AppConfig, Env Parsing & Mailer
- AppState
- ListingDetailDto
- AppError
- MyHouse Database Rules (sqlx/PostgreSQL)
- ListingFeed.tsx
- extractors.rs
- dependencies
- local_fs.rs
- devDependencies
- AppCacheProvider
- .new
- compilerOptions
- Code Review — Backend Skill
- CI Backend Workflow
- compilerOptions
- UnimplementedStorage
- notifications/service.rs
- auth/handler.rs
- find_is_active
- TokenDecoder
- .mcp.json
- auth/repository.rs
- errors.rs
- storage_key.rs
- ARCHITECTURE_v1.2.md — arc42 Software Architecture Document
- CLAUDE.md — MyHouse project instructions
- Mailer
- Extraction subagent prompt (full)
- TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP
- resolve_request_id
- scripts
- Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark
- mh-15-owner-request.html wireframe
- save-result feedback loop
- DESIGN_TOKENS.md (MH-17) — Design Token Set
- backend-dev service
- MokaStore
- Docker Rules Skill
- /graphify command
- mh-12-auth-flow.html wireframe
- .run
- health.rs
- /graphify add <url>
- Multiple repos cross-repo graph merge
- StorageProvider trait
- Embedded React/TypeScript Rules Content
- --update (incremental re-extraction)
- mh-14-listing-management.html wireframe
- connect_db
- Step 9 - Save manifest, update cost tracker, clean up, and report
- Step 4 - Build graph, cluster, analyze, generate outputs
- Extraction subagent prompt (compact)
- generate_otp_code
- git commit hook (graphify hook install)
- analyze job (rust + javascript-typescript matrix)
- Root Docker Compose
- Owner Request Approved Email Template
- pre-tool-use.sh
- tsconfig.json
- README.md — Project Overview and Setup
- ADR-04: moka in-memory over Redis
- pre-commit
- package.json
- hashing.rs
- eslint
- eslint-plugin-react-refresh
- globals
- prettier
- tailwindcss
- @types/react-dom
- graphify Slash Command Trigger (.claude/CLAUDE.md)
- Backend Review Checklist Reference
- ADR-01: Modular Monolith over Microservices
- ADR-03: PostgreSQL full-text over Meilisearch/Elasticsearch
- ADR-07: seeker default role over role choice at signup
- AppError centralized error type (§8.2)
- Health Check and Graceful Shutdown (§8.7)
- Pagination standard (§8.3)
- R-07: Absence of index on listings.price
- POST /auth/refresh endpoint
- backend/Dockerfile (multi-stage build)
- frontend/Dockerfile (multi-stage build)

## God Nodes (most connected - your core abstractions)
1. `AppError` - 88 edges
2. `AppState` - 34 edges
3. `cn()` - 27 edges
4. `AppCacheProvider` - 26 edges
5. `Mailer` - 20 edges
6. `perform_refresh()` - 19 edges
7. `perform_verify_otp()` - 19 edges
8. `Role` - 19 edges
9. `compilerOptions` - 18 edges
10. `AppCache` - 16 edges

## Surprising Connections (you probably didn't know these)
- `Step B2 - Dispatch ALL subagents in a single message` --semantically_similar_to--> `Step B2 - Dispatch ALL subagents (Codex spawn_agent)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md
- `Extraction subagent prompt (full)` --semantically_similar_to--> `Extraction subagent prompt (compact)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/extraction-spec.md → .codex/skills/graphify/references/extraction-spec.md
- `--update (incremental re-extraction)` --semantically_similar_to--> `--update (incremental re-extraction) (Codex)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/update.md → .codex/skills/graphify/references/update.md
- `AGENTS.md — graphify trigger instructions` --semantically_similar_to--> `CLAUDE.md — MyHouse project instructions`  [INFERRED] [semantically similar]
  AGENTS.md → CLAUDE.md
- `/graphify command` --semantically_similar_to--> `/graphify command (Codex variant)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Duplicated Rule Sets Between .agents/rules and .claude/rules** — agents_rules_database_rules_myhouse_db_rules, claude_rules_database_myhouse_db_rules, agents_rules_rust_general_rules_myhouse_rust_rules [INFERRED 0.85]
- **Extract-Build-Cluster pipeline flow (Steps 3-5)** — claude_skills_graphify_skill_step3_extract_entities, claude_skills_graphify_skill_part_a_structural_extraction, claude_skills_graphify_skill_part_b_semantic_extraction, claude_skills_graphify_skill_part_c_merge_ast_semantic, claude_skills_graphify_skill_step4_build_cluster_analyze, claude_skills_graphify_skill_step5_label_communities [EXTRACTED 1.00]
- **query/path/explain feedback-loop family** — claude_skills_graphify_references_query_traversal, claude_skills_graphify_references_query_graphify_path, claude_skills_graphify_references_query_graphify_explain, claude_skills_graphify_references_query_save_result [EXTRACTED 1.00]
- **Incremental-update integrity guards (stamping, pruning, manifest)** — claude_skills_graphify_skill_stamped_manifest_files_2015, claude_skills_graphify_references_update_replace_on_reextract_1344, claude_skills_graphify_references_update_incremental_update, claude_skills_graphify_skill_step9_manifest_cost_cleanup [INFERRED 0.85]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin, frontend_src_shared_api_types_ts, docs_technical_spec_mvp_v1_2_md_ts_codegen_pipeline, github_workflows_ci_frontend_yml_frontend_codegen [EXTRACTED 1.00]
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]
- **Owner Upgrade Request Flow (users → media → StorageProvider → admin)** — docs_architecture_v1_2_md_module_users, docs_architecture_v1_2_md_module_media, docs_architecture_v1_2_md_storageprovider_trait, docs_architecture_v1_2_md_module_admin [EXTRACTED 1.00]
- **Shared Four-Phase Review Structure** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_code_review_backend_skill_four_phase_review_process, claude_skills_code_review_frontend_skill_four_phase_review_process [INFERRED 0.85]
- **Skills Deferring to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_github_ticket_skill_github_ticket, docs_architecture_doc, docs_technical_spec_mvp_doc [INFERRED 0.85]
- **Refresh Token Handling Invariants Across Review Skills** — claude_skills_code_review_backend_skill_refresh_token_rotation_invariant, claude_skills_code_review_frontend_skill_refresh_token_cookie_invariant, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]

## Communities (115 total, 22 thin omitted)

### Community 0 - "components/index.ts"
Cohesion: 0.05
Nodes (54): App(), RootLayout(), Providers(), ProvidersProps, queryClient, ListingDetail, ListingFeed, router (+46 more)

### Community 1 - "AppConfig, Env Parsing & Mailer"
Cohesion: 0.16
Nodes (25): app_port_defaults_to_3000_when_absent(), AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), rejects_invalid_app_env(), rejects_invalid_smtp_port() (+17 more)

### Community 2 - "AppState"
Cohesion: 0.32
Nodes (12): ApiDoc, AppState, admin_router(), build_router(), merged_router(), openapi_spec(), owner_router(), public_router() (+4 more)

### Community 3 - "ListingDetailDto"
Cohesion: 0.08
Nodes (51): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+43 more)

### Community 4 - "AppError"
Cohesion: 0.10
Nodes (51): RefreshTokenLookup, Uuid, correct_code_known_email_mutates_nothing_and_reports_is_new_user_false(), correct_code_new_email_creates_account_and_issues_session_with_is_new_user_true(), expired_token_is_rejected_without_family_revocation(), handle_revoked(), logout(), logout_revokes_only_the_single_current_token() (+43 more)

### Community 5 - "MyHouse Database Rules (sqlx/PostgreSQL)"
Cohesion: 0.05
Nodes (44): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+36 more)

### Community 6 - "ListingFeed.tsx"
Cohesion: 0.14
Nodes (26): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+18 more)

### Community 7 - "extractors.rs"
Cohesion: 0.06
Nodes (56): Algorithm, Claims, encode_with_exp(), expired_token_is_rejected_with_token_expired(), issue_access_token(), issued_token_expires_exactly_ttl_seconds_after_issuance(), jwt_token_decoder_adapter_delegates_correctly(), JwtTokenDecoder (+48 more)

### Community 8 - "dependencies"
Cohesion: 0.15
Nodes (13): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+5 more)

### Community 9 - "local_fs.rs"
Cohesion: 0.17
Nodes (22): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes, Duration (+14 more)

### Community 10 - "devDependencies"
Cohesion: 0.11
Nodes (19): @eslint/js, eslint-plugin-react-hooks, devDependencies, @eslint/js, eslint-plugin-react-hooks, openapi-typescript, @tailwindcss/vite, @types/react (+11 more)

### Community 11 - "AppCacheProvider"
Cohesion: 0.11
Nodes (24): AppCache, build_cache_provider(), build_otp_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), Arc, Duration, RefreshTokenId (+16 more)

### Community 12 - ".new"
Cohesion: 0.18
Nodes (15): PaginatedResponse, PaginatedResponse<T>, PaginationMeta, Option, Self, T, Vec, test_defaults_applied_when_none() (+7 more)

### Community 13 - "compilerOptions"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 14 - "Code Review — Backend Skill"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 15 - "CI Backend Workflow"
Cohesion: 0.14
Nodes (22): ADR-06: utoipa + openapi-typescript over manual TS types, TypeScript type generation pipeline (utoipa → openapi-typescript → types.ts), src/shared/api/types.ts (generated OpenAPI types), CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest) (+14 more)

### Community 16 - "compilerOptions"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 17 - "UnimplementedStorage"
Cohesion: 0.26
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 18 - "notifications/service.rs"
Cohesion: 0.18
Nodes (16): NotificationTemplate, OtpTemplate, OwnerRequestApprovedTemplate, OwnerRequestReceivedTemplate, OwnerRequestRejectedTemplate, render(), renders_otp_template_with_sample_context(), renders_owner_request_approved_template_with_sample_context() (+8 more)

### Community 19 - "auth/handler.rs"
Cohesion: 0.24
Nodes (17): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+9 more)

### Community 20 - "find_is_active"
Cohesion: 0.47
Nodes (5): find_is_active(), Option, PgPool, Result, Uuid

### Community 21 - "TokenDecoder"
Cohesion: 0.26
Nodes (8): Inner, Arc, PgPool, Self, StorageProvider, Send, Sync, TokenDecoder

### Community 22 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 23 - "auth/repository.rs"
Cohesion: 0.34
Nodes (14): create_seeker(), db_err(), email_exists(), find_by_hash(), find_user_by_email(), insert_refresh_token(), revoke(), revoke_all_for_user() (+6 more)

### Community 24 - "errors.rs"
Cohesion: 0.24
Nodes (11): ErrorBody, ErrorEnvelope, parse_envelope(), Response, StatusCode, String, test_bad_request_carries_detail(), test_internal_error_is_500() (+3 more)

### Community 25 - "storage_key.rs"
Cohesion: 0.25
Nodes (13): assert_key_shape(), avatar_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key(), listing_media_key_has_expected_shape(), owner_request_document_key() (+5 more)

### Community 26 - "ARCHITECTURE_v1.2.md — arc42 Software Architecture Document"
Cohesion: 0.29
Nodes (14): ARCHITECTURE_v1.2.md — arc42 Software Architecture Document, Core Loop — Listing Publication and Discovery Flow (§6.3), admin module (moderation, owner request validation), auth module (OTP passwordless, JWT access+refresh, rate limiting), contact module (owner phone reveal), listings module (CRUD, status, cover photo), media module (upload, StorageProvider abstraction, magic-byte validation), notifications module (email OTP, email notifications) (+6 more)

### Community 27 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.17
Nodes (13): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), ADR-02: OTP Passwordless over Password/OAuth, ADR-09: Refresh token rotation on each use (+5 more)

### Community 28 - "Mailer"
Cohesion: 0.14
Nodes (16): Address, AddressError, AsyncSmtpTransport, AppConfig, builds_successfully_with_valid_config(), Mailer, MailerError, rejects_malformed_smtp_from() (+8 more)

### Community 29 - "Extraction subagent prompt (full)"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 30 - "TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP"
Cohesion: 0.26
Nodes (13): R-08: fn_update_listing_search_vector trigger SELECT-per-row cost, TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP, GET /listings/:id/contact endpoint, fn_cascade_owner_name_to_listings() trigger function, fn_set_updated_at() trigger function, fn_update_listing_search_vector() trigger function, listing_media table, listings table (+5 more)

### Community 31 - "resolve_request_id"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), Response, String, two_calls_without_header_produce_distinct_ids(), HeaderMap (+2 more)

### Community 32 - "scripts"
Cohesion: 0.17
Nodes (12): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+4 more)

### Community 33 - "Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 34 - "mh-15-owner-request.html wireframe"
Cohesion: 0.18
Nodes (11): No-proxy principle for public file reads (§7.3), Owner Request and Validation Runtime Flow (§6.2), Owner-requests documents proxy exception (admin-only read), R-09: Single admin account, no operational redundancy, GET /admin/owner-requests/:id/documents/:doc_id endpoint, frontend/nginx.conf (SPA fallback, /media/ static, /api proxy), mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire') (+3 more)

### Community 35 - "save-result feedback loop"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 36 - "DESIGN_TOKENS.md (MH-17) — Design Token Set"
Cohesion: 0.20
Nodes (10): GET/POST /listings endpoints, GET /search endpoint, DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-13-feed-detail.html wireframe (+2 more)

### Community 37 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 38 - "MokaStore"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 39 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 40 - "/graphify command"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 41 - "mh-12-auth-flow.html wireframe"
Cohesion: 0.25
Nodes (8): ADR-08: Single OTP endpoint over separate login/register, OTP Login/Signup Runtime Flow (§6.1), POST /auth/otp/request endpoint, POST /auth/otp/verify endpoint, mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 42 - ".run"
Cohesion: 0.24
Nodes (7): AppServer, Error, Result, Self, shutdown_signal(), Box, SocketAddr

### Community 43 - "health.rs"
Cohesion: 0.29
Nodes (7): check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus

### Community 44 - "/graphify add <url>"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 45 - "Multiple repos cross-repo graph merge"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 46 - "StorageProvider trait"
Cohesion: 0.29
Nodes (7): ADR-05: StorageProvider trait + LocalFsStorage over S3-at-MVP, LocalFsStorage implementation, R-06: Filesystem storage not shared across instances, StorageProvider trait, POST /owner-requests endpoint, Backend Environment Variables (.env.example), StorageProvider trait interface (upload/read/delete/presigned_url)

### Community 47 - "Embedded React/TypeScript Rules Content"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 48 - "--update (incremental re-extraction)"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 49 - "mh-14-listing-management.html wireframe"
Cohesion: 0.33
Nodes (6): POST /media/upload endpoint, mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, Identity document dropzone component

### Community 50 - "connect_db"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 51 - "Step 9 - Save manifest, update cost tracker, clean up, and report"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 52 - "Step 4 - Build graph, cluster, analyze, generate outputs"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 53 - "Extraction subagent prompt (compact)"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 54 - "generate_otp_code"
Cohesion: 0.67
Nodes (3): generate_otp_code(), generated_code_is_always_six_digits(), String

### Community 55 - "git commit hook (graphify hook install)"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 56 - "analyze job (rust + javascript-typescript matrix)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 57 - "Root Docker Compose"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 58 - "Owner Request Approved Email Template"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 61 - "README.md — Project Overview and Setup"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

### Community 67 - "package.json"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 68 - "hashing.rs"
Cohesion: 0.52
Nodes (5): hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest(), String, sha256_hex()

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `README Writing Rules Skill` → `react-typecrypt.md Rules File`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with
- `README Writing Rules Skill` → `Embedded React/TypeScript Rules Content`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with

## Knowledge Gaps
- **205 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+200 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `react-typecrypt.md Rules File`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `Embedded React/TypeScript Rules Content`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `AppError` to `ListingDetailDto`, `extractors.rs`, `local_fs.rs`, `UnimplementedStorage`, `notifications/service.rs`, `auth/handler.rs`, `find_is_active`, `auth/repository.rs`, `errors.rs`?**
  _High betweenness centrality (0.157) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState` to `ListingDetailDto`, `extractors.rs`, `.run`, `health.rs`, `auth/handler.rs`, `TokenDecoder`, `Mailer`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `Mailer` to `AppConfig, Env Parsing & Mailer`, `AppCacheProvider`, `TokenDecoder`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _205 weakly-connected nodes found - possible documentation gaps or missing edges._