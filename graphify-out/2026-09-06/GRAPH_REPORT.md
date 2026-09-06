# Graph Report - My-House  (2026-09-01)

## Corpus Check
- 202 files · ~92,215 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1342 nodes · 2559 edges · 123 communities (94 shown, 29 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 80 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f5c2395b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- components/index.ts
- auth/index.ts
- ListingFeed.tsx
- .new
- config/mod.rs
- jwt.rs
- Module: users
- MyHouse Database Rules (sqlx/PostgreSQL)
- rate_limit.rs
- AppCacheProvider
- Mailer
- local_fs.rs
- extractors.rs
- UserRow
- compilerOptions
- Code Review — Backend Skill
- DESIGN_TOKENS.md (MH-17) — Design Token Set
- compilerOptions
- OtpVerifyForm.tsx
- devDependencies
- storage_key.rs
- .mcp.json
- client.ts
- notifications/service.rs
- AppError
- route.rs
- ProfileForm.tsx
- Pagination.tsx
- .run
- Extraction subagent prompt (full)
- dependencies
- scripts
- resolve_request_id
- Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark
- CI Backend Workflow
- Graphify Query Flow
- backend-dev service
- MokaStore
- health.rs
- CI Frontend Workflow
- Docker Rules Skill
- Video/Audio Transcription Flow
- package.json
- CLAUDE.md — MyHouse project instructions
- /graphify add <url>
- Multiple repos cross-repo graph merge
- MokaStore<K, V>
- Embedded React/TypeScript Rules Content
- --update (incremental re-extraction)
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
- GET /health Endpoint
- Handler-Service-Repository Layering
- R-08: search_vector trigger N+1 query
- Refresh Token httpOnly Cookie
- API Contract /api/v1
- auth/handler.rs
- frontend/index.html — Vite SPA entry point
- globals
- prettier
- tailwindcss
- @types/react-dom
- typescript-eslint
- pre-commit
- AppState
- eslint-plugin-react-refresh
- Toast.tsx
- users/service.rs
- validation.rs
- router.tsx
- RecordingStorage
- AuthFlow.tsx
- file_validation.rs
- AvatarUpload.tsx
- graphify Slash Command Trigger (.claude/CLAUDE.md)
- Backend Review Checklist Reference
- AppError Centralized Error Type
- Graceful Shutdown (SIGTERM)
- Modular Monolith
- OpenAPI-driven TypeScript Type Generation
- Pagination Standard
- R-07: No index on listings.price
- POST /auth/refresh
- Environment Variables (.env.example)
- fn_set_updated_at() Trigger Function
- Frontend Folder Structure
- MVP Exclusions (V2 scope)
- Test Strategy per Layer

## God Nodes (most connected - your core abstractions)
1. `AppError` - 96 edges
2. `AppState` - 41 edges
3. `AppCacheProvider` - 29 edges
4. `cn()` - 27 edges
5. `AppConfig` - 20 edges
6. `set_valid_env()` - 20 edges
7. `AppCache` - 19 edges
8. `Mailer` - 18 edges
9. `Role` - 18 edges
10. `compilerOptions` - 18 edges

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
- **MyHouse Domain Modules (Modular Monolith)** — docs_architecture_v1_2_module_auth, docs_architecture_v1_2_module_users, docs_architecture_v1_2_module_listings, docs_architecture_v1_2_module_search, docs_architecture_v1_2_module_media, docs_architecture_v1_2_module_contact, docs_architecture_v1_2_module_notifications, docs_architecture_v1_2_module_admin [EXTRACTED 1.00]
- **OTP Login/Registration Flow** — docs_architecture_v1_2_otp_passwordless_auth, docs_architecture_v1_2_registration_ticket, docs_architecture_v1_2_atomic_registration, docs_technical_spec_mvp_v1_2_endpoint_auth_otp_request, docs_technical_spec_mvp_v1_2_endpoint_auth_otp_verify, docs_technical_spec_mvp_v1_2_endpoint_auth_register [EXTRACTED 1.00]
- **Owner Upgrade Request/Validation Workflow** — docs_architecture_v1_2_owner_request_flow, docs_architecture_v1_2_admin_bootstrap, docs_technical_spec_mvp_v1_2_owner_requests_table, docs_technical_spec_mvp_v1_2_endpoint_owner_requests_post, docs_technical_spec_mvp_v1_2_endpoint_admin_owner_requests_doc [EXTRACTED 1.00]
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Duplicated Rule Sets Between .agents/rules and .claude/rules** — agents_rules_database_rules_myhouse_db_rules, claude_rules_database_myhouse_db_rules, agents_rules_rust_general_rules_myhouse_rust_rules [INFERRED 0.85]
- **Shared Four-Phase Review Structure** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_code_review_backend_skill_four_phase_review_process, claude_skills_code_review_frontend_skill_four_phase_review_process [INFERRED 0.85]
- **Refresh Token Handling Invariants Across Review Skills** — claude_skills_code_review_backend_skill_refresh_token_rotation_invariant, claude_skills_code_review_frontend_skill_refresh_token_cookie_invariant, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Skills Deferring to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_github_ticket_skill_github_ticket, docs_architecture_doc, docs_technical_spec_mvp_doc [INFERRED 0.85]
- **Extract-Build-Cluster pipeline flow (Steps 3-5)** — claude_skills_graphify_skill_step3_extract_entities, claude_skills_graphify_skill_part_a_structural_extraction, claude_skills_graphify_skill_part_b_semantic_extraction, claude_skills_graphify_skill_part_c_merge_ast_semantic, claude_skills_graphify_skill_step4_build_cluster_analyze, claude_skills_graphify_skill_step5_label_communities [EXTRACTED 1.00]
- **query/path/explain feedback-loop family** — claude_skills_graphify_references_query_traversal, claude_skills_graphify_references_query_graphify_path, claude_skills_graphify_references_query_graphify_explain, claude_skills_graphify_references_query_save_result [EXTRACTED 1.00]
- **Incremental-update integrity guards (stamping, pruning, manifest)** — claude_skills_graphify_skill_stamped_manifest_files_2015, claude_skills_graphify_references_update_replace_on_reextract_1344, claude_skills_graphify_references_update_incremental_update, claude_skills_graphify_skill_step9_manifest_cost_cleanup [INFERRED 0.85]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin, frontend_src_shared_api_types_ts, github_workflows_ci_frontend_yml_frontend_codegen [EXTRACTED 1.00]
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]

## Communities (123 total, 29 thin omitted)

### Community 0 - "components/index.ts"
Cohesion: 0.10
Nodes (32): AlertProps, AlertVariant, variantConfig, Button(), ButtonProps, ButtonSize, ButtonVariant, sizeClasses (+24 more)

### Community 1 - "auth/index.ts"
Cohesion: 0.17
Nodes (22): logout(), OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, refreshSession(), registerAccount(), RegisterPayload (+14 more)

### Community 2 - "ListingFeed.tsx"
Cohesion: 0.20
Nodes (20): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+12 more)

### Community 3 - ".new"
Cohesion: 0.07
Nodes (51): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+43 more)

### Community 4 - "config/mod.rs"
Cohesion: 0.09
Nodes (50): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, empty_trusted_proxies_trusts_nothing(), loads_admin_bootstrap_email_when_enabled(), loads_valid_config() (+42 more)

### Community 5 - "jwt.rs"
Cohesion: 0.08
Nodes (37): Algorithm, NewAccount, RefreshTokenLookup, Option, Uuid, hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest() (+29 more)

### Community 6 - "Module: users"
Cohesion: 0.05
Nodes (47): Account Deletion Cascade + Storage Cleanup, Single Admin Account Bootstrap, Atomic Registration via POST /auth/register, AuthUser Extractor, AwsS3Storage (V2), LocalFsStorage, Module: admin, Module: auth (+39 more)

### Community 7 - "MyHouse Database Rules (sqlx/PostgreSQL)"
Cohesion: 0.05
Nodes (44): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+36 more)

### Community 8 - "rate_limit.rs"
Cohesion: 0.14
Nodes (28): allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it(), distinct_clients_get_distinct_counters(), falls_back_to_peer_ip_when_trusted_header_is_missing(), headers_with_xff(), ignores_x_forwarded_for_from_an_untrusted_peer(), middleware_passes_then_rejects_with_429_and_retry_after(), peer(), rate_limit() (+20 more)

### Community 9 - "AppCacheProvider"
Cohesion: 0.10
Nodes (43): AppCache, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), Arc, AtomicU32 (+35 more)

### Community 10 - "Mailer"
Cohesion: 0.16
Nodes (15): Address, AddressError, AsyncSmtpTransport, builds_successfully_with_valid_config(), Mailer, MailerError, rejects_malformed_smtp_from(), Formatter (+7 more)

### Community 11 - "local_fs.rs"
Cohesion: 0.17
Nodes (22): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes, Duration (+14 more)

### Community 12 - "extractors.rs"
Cohesion: 0.10
Nodes (34): get_me(), multipart_error(), read_file_field(), Bytes, Json, Result, State, update_me() (+26 more)

### Community 13 - "UserRow"
Cohesion: 0.13
Nodes (16): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Self, String (+8 more)

### Community 14 - "compilerOptions"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 15 - "Code Review — Backend Skill"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 16 - "DESIGN_TOKENS.md (MH-17) — Design Token Set"
Cohesion: 0.10
Nodes (22): DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page (+14 more)

### Community 17 - "compilerOptions"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 18 - "OtpVerifyForm.tsx"
Cohesion: 0.18
Nodes (14): requestOtp(), OtpCodeInput(), OtpCodeInputProps, OtpRequestForm(), OtpRequestFormProps, emptyCode(), OtpVerifyForm(), OtpVerifyFormProps (+6 more)

### Community 19 - "devDependencies"
Cohesion: 0.11
Nodes (19): eslint, @eslint/js, eslint-plugin-react-hooks, devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks, openapi-typescript (+11 more)

### Community 20 - "storage_key.rs"
Cohesion: 0.17
Nodes (16): assert_key_shape(), avatar_key(), avatar_key_from_url(), avatar_key_from_url_round_trips_a_generated_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key() (+8 more)

### Community 21 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 22 - "client.ts"
Cohesion: 0.25
Nodes (10): AccessTokenGetter, buildQueryString(), ErrorEnvelope, isAuthPath(), QueryValue, readAccessToken(), readRetryAfter(), request() (+2 more)

### Community 23 - "notifications/service.rs"
Cohesion: 0.18
Nodes (16): NotificationTemplate, OtpTemplate, OwnerRequestApprovedTemplate, OwnerRequestReceivedTemplate, OwnerRequestRejectedTemplate, render(), renders_otp_template_with_sample_context(), renders_owner_request_approved_template_with_sample_context() (+8 more)

### Community 24 - "AppError"
Cohesion: 0.06
Nodes (64): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage (+56 more)

### Community 25 - "route.rs"
Cohesion: 0.32
Nodes (12): ApiDoc, admin_router(), avatar_router(), build_router(), merged_router(), openapi_spec(), owner_router(), public_router() (+4 more)

### Community 26 - "ProfileForm.tsx"
Cohesion: 0.17
Nodes (21): getMe(), Profile, profileQueryKey, updateMe(), UpdateProfilePayload, uploadAvatar(), UserResponse, FieldErrors (+13 more)

### Community 27 - "Pagination.tsx"
Cohesion: 0.38
Nodes (4): Pagination(), PaginationProps, getPageItems(), PageItem

### Community 28 - ".run"
Cohesion: 0.27
Nodes (7): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box

### Community 29 - "Extraction subagent prompt (full)"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 30 - "dependencies"
Cohesion: 0.15
Nodes (13): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+5 more)

### Community 31 - "scripts"
Cohesion: 0.17
Nodes (12): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+4 more)

### Community 32 - "resolve_request_id"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), HeaderMap, Next, Request, Response (+2 more)

### Community 33 - "Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 34 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 35 - "Graphify Query Flow"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 36 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 37 - "MokaStore"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 38 - "health.rs"
Cohesion: 0.29
Nodes (7): check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus

### Community 39 - "CI Frontend Workflow"
Cohesion: 0.39
Nodes (9): src/shared/api/types.ts (generated OpenAPI types), CI Frontend Workflow, frontend_build job, frontend_codegen job (OpenAPI TS codegen), frontend_docker job, frontend_install job, frontend_lint job (ESLint), frontend_prettier job (+1 more)

### Community 40 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 41 - "Video/Audio Transcription Flow"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 42 - "package.json"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 43 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 44 - "/graphify add <url>"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 45 - "Multiple repos cross-repo graph merge"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 46 - "MokaStore<K, V>"
Cohesion: 0.52
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 47 - "Embedded React/TypeScript Rules Content"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 48 - "--update (incremental re-extraction)"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 49 - "connect_db"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 50 - "Step 9 - Save manifest, update cost tracker, clean up, and report"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 51 - "Step 4 - Build graph, cluster, analyze, generate outputs"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 52 - "Extraction subagent prompt (compact)"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 53 - "generate_otp_code"
Cohesion: 0.67
Nodes (3): generate_otp_code(), generated_code_is_always_six_digits(), String

### Community 54 - "git commit hook (graphify hook install)"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 55 - "analyze job (rust + javascript-typescript matrix)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 56 - "Root Docker Compose"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 57 - "Owner Request Approved Email Template"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 60 - "README.md — Project Overview and Setup"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

### Community 67 - "auth/handler.rs"
Cohesion: 0.14
Nodes (29): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+21 more)

### Community 76 - "AppState"
Cohesion: 0.27
Nodes (6): AppState, Inner, Arc, PgPool, Self, StorageProvider

### Community 78 - "Toast.tsx"
Cohesion: 0.15
Nodes (12): App(), Providers(), ProvidersProps, queryClient, router, ToastContext, ToastContextValue, ToastItem (+4 more)

### Community 79 - "users/service.rs"
Cohesion: 0.30
Nodes (13): bootstrap_admin(), delete_previous_avatar(), deletes_the_key_behind_the_previous_avatar_url(), get_me(), replace_avatar(), Option, PgPool, Self (+5 more)

### Community 82 - "validation.rs"
Cohesion: 0.20
Nodes (10): update_me(), optional_name(), repeat(), required_name(), required_name_trims_and_accepts_the_upper_bound(), required_phone(), required_phone_trims_and_accepts_the_upper_bound(), Option (+2 more)

### Community 83 - "router.tsx"
Cohesion: 0.18
Nodes (9): AuthLayout(), RootLayout(), RequireAuth(), RequireAuthProps, AuthFlow, ListingDetail, ListingFeed, ProfileForm (+1 more)

### Community 84 - "RecordingStorage"
Cohesion: 0.29
Nodes (7): RecordingStorage, Bytes, Duration, Result, String, Vec, Mutex

### Community 88 - "AuthFlow.tsx"
Cohesion: 0.31
Nodes (7): AuthFlow(), markInterrupted(), readInterrupted(), Screen, FieldErrors, RegistrationForm(), RegistrationFormProps

### Community 89 - "file_validation.rs"
Cohesion: 0.32
Nodes (4): accepts_supported_image_formats(), Result, validate_image(), ValidatedFile

### Community 90 - "AvatarUpload.tsx"
Cohesion: 0.39
Nodes (5): AvatarUpload(), AvatarUploadProps, preCheck(), ACCEPTED_AVATAR_TYPES, useToast()

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `README Writing Rules Skill` → `react-typecrypt.md Rules File`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with
- `README Writing Rules Skill` → `Embedded React/TypeScript Rules Content`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with

## Knowledge Gaps
- **234 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+229 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **29 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `react-typecrypt.md Rules File`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `Embedded React/TypeScript Rules Content`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `AppError` to `.new`, `auth/handler.rs`, `jwt.rs`, `AppCacheProvider`, `local_fs.rs`, `extractors.rs`, `users/service.rs`, `validation.rs`, `RecordingStorage`, `notifications/service.rs`, `file_validation.rs`?**
  _High betweenness centrality (0.169) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `config/mod.rs` to `Mailer`, `AppState`, `users/service.rs`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `AppCacheProvider` connect `AppCacheProvider` to `rate_limit.rs`, `extractors.rs`, `MokaStore<K, V>`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _234 weakly-connected nodes found - possible documentation gaps or missing edges._