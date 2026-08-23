# Graph Report - /Users/hermann/Documents/M@Vie/My-/My-House  (2026-08-23)

## Corpus Check
- 189 files · ~82,254 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1127 nodes · 2104 edges · 117 communities (95 shown, 22 thin omitted)
- Extraction: 95% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 92 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- StorageProvider Trait & Auth Repository Core
- Auth Model & Service — OTP Login/Signup
- JWT Crypto & AuthUser Extractor
- Frontend Shared UI Components (Alert/Button/Card)
- Database Rules & Project Instructions (Agents)
- Listing Detail DTOs & Handler
- AppConfig & Env Parsing
- App Cache — OTP, Rate Limit & Refresh Replay
- LocalFsStorage Implementation
- Frontend Listings API
- AppState, Mailer & AppConfig
- Frontend TS App Config
- Backend/Frontend Code Review Skills & Invariants
- CI Workflows & OpenAPI Type Generation
- Frontend TS Node Config
- Auth DTOs & Handler (OTP Request/Verify)
- Pagination DTOs
- Frontend Dev Dependencies (ESLint plugins, Types)
- Notification Templates Service
- MCP Server Config
- AppState, API Doc & Router Bootstrap
- Storage Key Generation
- Architecture Doc — Core Flows & Modules
- Frontend App Shell & Routing
- Project CLAUDE.md Instructions
- AppState & Token Decoder
- Graphify Extraction Spec
- Frontend Runtime Dependencies
- Technical Spec — DB Triggers & Contact Endpoint
- Frontend NPM Scripts
- App Server Bootstrap & Main Entrypoint
- Request ID / Logging Middleware
- Graphify Export Steps (Wiki/Neo4j/FalkorDB/SVG/GraphML)
- No-Proxy & Owner Request Docs Flow
- Health Check Endpoint
- Graphify Query/Path/Explain Flow
- Design Tokens & Listings Endpoints
- Docker Compose Services (Dev)
- MokaStore Implementation
- Frontend Toast Notification Component
- Docker Rules Skill
- Graphify Video Transcription Step
- OTP Auth Flow & ADR
- Frontend package.json Metadata
- Frontend API Client (fetch wrapper)
- MokaStore Generic Methods
- OTP Hashing & Crypto Mod Root
- Graphify Add/Watch Ingest
- Graphify Multi-Repo Merge Flow
- Storage ADR & Owner Requests
- Frontend Pagination Component & Utils
- README Skill (embeds duplicated React/TS rules)
- Graphify Update/Cluster-Only Subcommands
- Listing Management Wireframes
- Database Connection Pool
- Graphify Manifest & Honesty Rules
- Graphify Build Pipeline Steps
- Graphify Codex Multi-Agent Spawn
- OTP Code Generation
- Graphify CLAUDE.md/Hook Integration
- CI Security Workflows (CodeQL/Gitleaks)
- Docker Compose Files (Structural)
- Owner Request Email Templates
- Pre-Tool-Use Hook Script
- TS Project References Config
- README & Git Conventions
- Moka Cache ADR
- ESLint Dependency Pair
- ESLint React Refresh Plugin Pair
- Globals Dependency Pair
- Prettier Dependency Pair
- Tailwind CSS Dependency Pair
- React DOM Types Dependency Pair
- Pre-Commit Hook Script
- Graphify Slash Command Trigger
- Backend Review Checklist Reference
- ADR: Modular Monolith
- ADR: Postgres Full-Text Search
- ADR: Seeker Default Role
- AppError Doc Reference (§8.2)
- Health Check & Graceful Shutdown
- Pagination Standard Spec
- Listings Price Index Gap
- Auth Refresh Endpoint
- Backend Dockerfile
- Frontend Dockerfile

## God Nodes (most connected - your core abstractions)
1. `AppError` - 88 edges
2. `AppState` - 34 edges
3. `cn()` - 27 edges
4. `AppCacheProvider` - 26 edges
5. `Mailer` - 20 edges
6. `perform_verify_otp()` - 20 edges
7. `Role` - 20 edges
8. `perform_refresh()` - 19 edges
9. `compilerOptions` - 18 edges
10. `AppConfig` - 16 edges

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
- **Shared Four-Phase Review Structure** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_code_review_backend_skill_four_phase_review_process, claude_skills_code_review_frontend_skill_four_phase_review_process [INFERRED 0.85]
- **Refresh Token Handling Invariants Across Review Skills** — claude_skills_code_review_backend_skill_refresh_token_rotation_invariant, claude_skills_code_review_frontend_skill_refresh_token_cookie_invariant, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Skills Deferring to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_github_ticket_skill_github_ticket, docs_architecture_doc, docs_technical_spec_mvp_doc [INFERRED 0.85]
- **Extract-Build-Cluster pipeline flow (Steps 3-5)** — claude_skills_graphify_skill_step3_extract_entities, claude_skills_graphify_skill_part_a_structural_extraction, claude_skills_graphify_skill_part_b_semantic_extraction, claude_skills_graphify_skill_part_c_merge_ast_semantic, claude_skills_graphify_skill_step4_build_cluster_analyze, claude_skills_graphify_skill_step5_label_communities [EXTRACTED 1.00]
- **query/path/explain feedback-loop family** — claude_skills_graphify_references_query_traversal, claude_skills_graphify_references_query_graphify_path, claude_skills_graphify_references_query_graphify_explain, claude_skills_graphify_references_query_save_result [EXTRACTED 1.00]
- **Incremental-update integrity guards (stamping, pruning, manifest)** — claude_skills_graphify_skill_stamped_manifest_files_2015, claude_skills_graphify_references_update_replace_on_reextract_1344, claude_skills_graphify_references_update_incremental_update, claude_skills_graphify_skill_step9_manifest_cost_cleanup [INFERRED 0.85]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin, frontend_src_shared_api_types_ts, docs_technical_spec_mvp_v1_2_md_ts_codegen_pipeline, github_workflows_ci_frontend_yml_frontend_codegen [EXTRACTED 1.00]
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]
- **Owner Upgrade Request Flow (users → media → StorageProvider → admin)** — docs_architecture_v1_2_md_module_users, docs_architecture_v1_2_md_module_media, docs_architecture_v1_2_md_storageprovider_trait, docs_architecture_v1_2_md_module_admin [EXTRACTED 1.00]

## Communities (117 total, 22 thin omitted)

### Community 0 - "StorageProvider Trait & Auth Repository Core"
Cohesion: 0.05
Nodes (63): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage (+55 more)

### Community 1 - "Auth Model & Service — OTP Login/Signup"
Cohesion: 0.10
Nodes (48): RefreshTokenLookup, Uuid, correct_code_known_email_mutates_nothing_and_reports_is_new_user_false(), correct_code_new_email_creates_account_and_issues_session_with_is_new_user_true(), expired_token_is_rejected_without_family_revocation(), handle_revoked(), logout(), logout_revokes_only_the_single_current_token() (+40 more)

### Community 2 - "JWT Crypto & AuthUser Extractor"
Cohesion: 0.08
Nodes (50): Algorithm, Claims, encode_with_exp(), expired_token_is_rejected_with_token_expired(), issue_access_token(), issued_token_expires_exactly_ttl_seconds_after_issuance(), jwt_token_decoder_adapter_delegates_correctly(), JwtTokenDecoder (+42 more)

### Community 3 - "Frontend Shared UI Components (Alert/Button/Card)"
Cohesion: 0.10
Nodes (34): Alert(), AlertProps, AlertVariant, variantConfig, Button(), ButtonProps, ButtonSize, ButtonVariant (+26 more)

### Community 4 - "Database Rules & Project Instructions (Agents)"
Cohesion: 0.05
Nodes (44): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+36 more)

### Community 5 - "Listing Detail DTOs & Handler"
Cohesion: 0.11
Nodes (36): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+28 more)

### Community 6 - "AppConfig & Env Parsing"
Cohesion: 0.15
Nodes (29): app_port_defaults_to_3000_when_absent(), AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), parses_allowed_origins_into_a_list(), rejects_allowed_origins_with_a_stray_comma() (+21 more)

### Community 7 - "App Cache — OTP, Rate Limit & Refresh Replay"
Cohesion: 0.14
Nodes (20): AppCache, build_cache_provider(), build_otp_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), Arc, Duration, RefreshTokenId (+12 more)

### Community 8 - "LocalFsStorage Implementation"
Cohesion: 0.17
Nodes (22): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes, Duration (+14 more)

### Community 9 - "Frontend Listings API"
Cohesion: 0.18
Nodes (20): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+12 more)

### Community 10 - "AppState, Mailer & AppConfig"
Cohesion: 0.12
Nodes (19): Address, AddressError, AsyncSmtpTransport, AppConfig, Vec, builds_successfully_with_valid_config(), Mailer, MailerError (+11 more)

### Community 11 - "Frontend TS App Config"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 12 - "Backend/Frontend Code Review Skills & Invariants"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 13 - "CI Workflows & OpenAPI Type Generation"
Cohesion: 0.14
Nodes (22): ADR-06: utoipa + openapi-typescript over manual TS types, TypeScript type generation pipeline (utoipa → openapi-typescript → types.ts), src/shared/api/types.ts (generated OpenAPI types), CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest) (+14 more)

### Community 14 - "Frontend TS Node Config"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 15 - "Auth DTOs & Handler (OTP Request/Verify)"
Cohesion: 0.24
Nodes (17): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+9 more)

### Community 16 - "Pagination DTOs"
Cohesion: 0.18
Nodes (15): PaginatedResponse, PaginatedResponse<T>, PaginationMeta, Option, Self, T, Vec, test_defaults_applied_when_none() (+7 more)

### Community 17 - "Frontend Dev Dependencies (ESLint plugins, Types)"
Cohesion: 0.11
Nodes (19): @eslint/js, eslint-plugin-react-hooks, devDependencies, @eslint/js, eslint-plugin-react-hooks, openapi-typescript, @tailwindcss/vite, @types/react (+11 more)

### Community 18 - "Notification Templates Service"
Cohesion: 0.18
Nodes (16): NotificationTemplate, OtpTemplate, OwnerRequestApprovedTemplate, OwnerRequestReceivedTemplate, OwnerRequestRejectedTemplate, render(), renders_otp_template_with_sample_context(), renders_owner_request_approved_template_with_sample_context() (+8 more)

### Community 19 - "MCP Server Config"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 20 - "AppState, API Doc & Router Bootstrap"
Cohesion: 0.32
Nodes (12): ApiDoc, AppState, admin_router(), build_router(), merged_router(), openapi_spec(), owner_router(), public_router() (+4 more)

### Community 21 - "Storage Key Generation"
Cohesion: 0.25
Nodes (13): assert_key_shape(), avatar_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key(), listing_media_key_has_expected_shape(), owner_request_document_key() (+5 more)

### Community 22 - "Architecture Doc — Core Flows & Modules"
Cohesion: 0.29
Nodes (14): ARCHITECTURE_v1.2.md — arc42 Software Architecture Document, Core Loop — Listing Publication and Discovery Flow (§6.3), admin module (moderation, owner request validation), auth module (OTP passwordless, JWT access+refresh, rate limiting), contact module (owner phone reveal), listings module (CRUD, status, cover photo), media module (upload, StorageProvider abstraction, magic-byte validation), notifications module (email OTP, email notifications) (+6 more)

### Community 23 - "Frontend App Shell & Routing"
Cohesion: 0.19
Nodes (8): App(), RootLayout(), Providers(), ProvidersProps, queryClient, ListingDetail, ListingFeed, router

### Community 24 - "Project CLAUDE.md Instructions"
Cohesion: 0.17
Nodes (13): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), ADR-02: OTP Passwordless over Password/OAuth, ADR-09: Refresh token rotation on each use (+5 more)

### Community 25 - "AppState & Token Decoder"
Cohesion: 0.26
Nodes (8): Inner, Arc, PgPool, Self, StorageProvider, Send, Sync, TokenDecoder

### Community 26 - "Graphify Extraction Spec"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 27 - "Frontend Runtime Dependencies"
Cohesion: 0.15
Nodes (13): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+5 more)

### Community 28 - "Technical Spec — DB Triggers & Contact Endpoint"
Cohesion: 0.26
Nodes (13): R-08: fn_update_listing_search_vector trigger SELECT-per-row cost, TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP, GET /listings/:id/contact endpoint, fn_cascade_owner_name_to_listings() trigger function, fn_set_updated_at() trigger function, fn_update_listing_search_vector() trigger function, listing_media table, listings table (+5 more)

### Community 29 - "Frontend NPM Scripts"
Cohesion: 0.17
Nodes (12): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+4 more)

### Community 30 - "App Server Bootstrap & Main Entrypoint"
Cohesion: 0.24
Nodes (7): AppServer, Error, Result, Self, shutdown_signal(), Box, SocketAddr

### Community 31 - "Request ID / Logging Middleware"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), Response, String, two_calls_without_header_produce_distinct_ids(), HeaderMap (+2 more)

### Community 32 - "Graphify Export Steps (Wiki/Neo4j/FalkorDB/SVG/GraphML)"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 33 - "No-Proxy & Owner Request Docs Flow"
Cohesion: 0.18
Nodes (11): No-proxy principle for public file reads (§7.3), Owner Request and Validation Runtime Flow (§6.2), Owner-requests documents proxy exception (admin-only read), R-09: Single admin account, no operational redundancy, GET /admin/owner-requests/:id/documents/:doc_id endpoint, frontend/nginx.conf (SPA fallback, /media/ static, /api proxy), mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire') (+3 more)

### Community 34 - "Health Check Endpoint"
Cohesion: 0.29
Nodes (7): check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus

### Community 35 - "Graphify Query/Path/Explain Flow"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 36 - "Design Tokens & Listings Endpoints"
Cohesion: 0.20
Nodes (10): GET/POST /listings endpoints, GET /search endpoint, DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-13-feed-detail.html wireframe (+2 more)

### Community 37 - "Docker Compose Services (Dev)"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 38 - "MokaStore Implementation"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 39 - "Frontend Toast Notification Component"
Cohesion: 0.22
Nodes (8): ToastContext, ToastContextValue, ToastItem, ToastOptions, ToastProvider(), ToastVariant, useToast(), variantConfig

### Community 40 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 41 - "Graphify Video Transcription Step"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 42 - "OTP Auth Flow & ADR"
Cohesion: 0.25
Nodes (8): ADR-08: Single OTP endpoint over separate login/register, OTP Login/Signup Runtime Flow (§6.1), POST /auth/otp/request endpoint, POST /auth/otp/verify endpoint, mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 43 - "Frontend package.json Metadata"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 44 - "Frontend API Client (fetch wrapper)"
Cohesion: 0.32
Nodes (6): ApiError, apiGet(), buildQueryString(), ErrorEnvelope, QueryValue, request()

### Community 45 - "MokaStore Generic Methods"
Cohesion: 0.52
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 46 - "OTP Hashing & Crypto Mod Root"
Cohesion: 0.52
Nodes (5): hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest(), String, sha256_hex()

### Community 47 - "Graphify Add/Watch Ingest"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 48 - "Graphify Multi-Repo Merge Flow"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 49 - "Storage ADR & Owner Requests"
Cohesion: 0.29
Nodes (7): ADR-05: StorageProvider trait + LocalFsStorage over S3-at-MVP, LocalFsStorage implementation, R-06: Filesystem storage not shared across instances, StorageProvider trait, POST /owner-requests endpoint, Backend Environment Variables (.env.example), StorageProvider trait interface (upload/read/delete/presigned_url)

### Community 50 - "Frontend Pagination Component & Utils"
Cohesion: 0.38
Nodes (4): Pagination(), PaginationProps, getPageItems(), PageItem

### Community 51 - "README Skill (embeds duplicated React/TS rules)"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 52 - "Graphify Update/Cluster-Only Subcommands"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 53 - "Listing Management Wireframes"
Cohesion: 0.33
Nodes (6): POST /media/upload endpoint, mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, Identity document dropzone component

### Community 54 - "Database Connection Pool"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 55 - "Graphify Manifest & Honesty Rules"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 56 - "Graphify Build Pipeline Steps"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 57 - "Graphify Codex Multi-Agent Spawn"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 58 - "OTP Code Generation"
Cohesion: 0.67
Nodes (3): generate_otp_code(), generated_code_is_always_six_digits(), String

### Community 59 - "Graphify CLAUDE.md/Hook Integration"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 60 - "CI Security Workflows (CodeQL/Gitleaks)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 61 - "Docker Compose Files (Structural)"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 62 - "Owner Request Email Templates"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 65 - "README & Git Conventions"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

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
- **Why does `AppError` connect `StorageProvider Trait & Auth Repository Core` to `Auth Model & Service — OTP Login/Signup`, `JWT Crypto & AuthUser Extractor`, `Listing Detail DTOs & Handler`, `LocalFsStorage Implementation`, `Auth DTOs & Handler (OTP Request/Verify)`, `Notification Templates Service`?**
  _High betweenness centrality (0.151) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState, API Doc & Router Bootstrap` to `Health Check Endpoint`, `JWT Crypto & AuthUser Extractor`, `Listing Detail DTOs & Handler`, `AppState, Mailer & AppConfig`, `Auth DTOs & Handler (OTP Request/Verify)`, `AppState & Token Decoder`, `App Server Bootstrap & Main Entrypoint`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `AppState, Mailer & AppConfig` to `AppState & Token Decoder`, `AppConfig & Env Parsing`, `App Cache — OTP, Rate Limit & Refresh Replay`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _205 weakly-connected nodes found - possible documentation gaps or missing edges._