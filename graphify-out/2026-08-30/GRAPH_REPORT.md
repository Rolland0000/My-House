# Graph Report - .  (2026-08-30)

## Corpus Check
- 50 files · ~88,061 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1275 nodes · 2276 edges · 131 communities (86 shown, 45 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 71 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Listings API & Types
- Auth API & OTP Types
- API Docs & App Server
- Listings DTOs
- App Config
- Hashing Utils
- Auth/Storage Rationale (ADRs)
- Database Rules Skill
- Cache Provider (moka)
- Auth Service
- Mailer
- Local FS Storage
- Users Handler
- App Cache
- TS App Config
- Backend Review Skill
- Design Tokens
- TS Node Config
- Users Service
- ESLint Plugins
- Users DTOs
- MCP Config
- App Layouts
- Auth Repository
- Listings Repository
- Error Envelope
- Auth DTOs
- Storage Key Utils
- Storage Provider Trait
- Extraction Spec Rules
- Frontend Dependencies
- NPM Scripts
- Request Logging
- Graphify Export Steps
- CI Backend Workflow
- Graphify Query Flow
- Docker Compose Services & Email Templates
- MokaStore Implementation
- Users Repository
- CI Frontend Workflow
- Docker Rules Skill
- Video/Audio Transcription Flow
- Package Manifest
- Project CLAUDE.md Instructions
- Graphify Watch/Add Flow
- Multi-Repo Merge Flow
- Auth Model
- React/TS Rules File
- Graphify Update Subcommands
- DB Connection
- Graphify Manifest & Honesty Rules
- Graphify Build & Cluster Steps
- Codex Subagent Dispatch
- OTP Code Generation
- Graphify Hook Install
- Security Scan Workflows
- Docker Compose Configs
- Owner Request Email Templates
- Pre-Tool-Use Hook
- TS Project References
- Project Conventions
- Health Endpoint & Infra
- Layering Architecture
- Search Vector Trigger
- Refresh Token Cookie
- OpenAPI Codegen Pipeline
- ESLint
- Vite SPA Entry
- Globals Package
- Prettier
- Tailwind CSS
- React DOM Types
- TypeScript ESLint
- Pre-Commit Hook
- StorageProvider Trait Node
- AtomicU32 Type
- Duration Type
- IpAddr Type
- HeaderMap Type
- Middleware Next
- Request Type
- AtomicUsize Type
- Send Trait
- Sync Trait
- Vec Type
- StatusCode Type
- AtomicUsize Type (2)
- Option Type
- Send Trait (2)
- Sync Trait (2)
- Graphify Slash Trigger
- Backend Review Checklist Ref
- AppError Type
- Graceful Shutdown
- Modular Monolith Concept
- OpenAPI TS Generation
- Pagination Standard
- Listings Price Index Gap
- Refresh Endpoint
- Env Variables
- Updated-At Trigger Function
- Frontend Folder Structure
- MVP Exclusions
- Test Strategy

## God Nodes (most connected - your core abstractions)
1. `AppError` - 82 edges
2. `AppState` - 38 edges
3. `cn()` - 27 edges
4. `set_valid_env()` - 20 edges
5. `AppCache` - 19 edges
6. `compilerOptions` - 18 edges
7. `ConfigError` - 16 edges
8. `compilerOptions` - 16 edges
9. `register()` - 16 edges
10. `AppConfig` - 14 edges

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

## Communities (131 total, 45 thin omitted)

### Community 0 - "Listings API & Types"
Cohesion: 0.05
Nodes (67): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+59 more)

### Community 1 - "Auth API & OTP Types"
Cohesion: 0.06
Nodes (57): logout(), OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, refreshSession(), registerAccount(), RegisterPayload (+49 more)

### Community 2 - "API Docs & App Server"
Cohesion: 0.06
Nodes (50): ApiDoc, AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), AppState (+42 more)

### Community 3 - "Listings DTOs"
Cohesion: 0.07
Nodes (51): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+43 more)

### Community 4 - "App Config"
Cohesion: 0.09
Nodes (50): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, empty_trusted_proxies_trusts_nothing(), loads_admin_bootstrap_email_when_enabled(), loads_valid_config() (+42 more)

### Community 5 - "Hashing Utils"
Cohesion: 0.08
Nodes (36): Algorithm, hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest(), String, sha256_hex(), Claims, encode_with_exp() (+28 more)

### Community 6 - "Auth/Storage Rationale (ADRs)"
Cohesion: 0.05
Nodes (47): Account Deletion Cascade + Storage Cleanup, Single Admin Account Bootstrap, Atomic Registration via POST /auth/register, AuthUser Extractor, AwsS3Storage (V2), LocalFsStorage, Module: admin, Module: auth (+39 more)

### Community 7 - "Database Rules Skill"
Cohesion: 0.05
Nodes (44): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+36 more)

### Community 8 - "Cache Provider (moka)"
Cohesion: 0.10
Nodes (35): AppCacheProvider, MokaStore<K, V>, K, Option, Send, Sync, V, allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it() (+27 more)

### Community 9 - "Auth Service"
Cohesion: 0.14
Nodes (28): handle_revoked(), logout(), mint(), otp_request(), OtpVerifyConfig, refresh(), register(), RegisterInput (+20 more)

### Community 10 - "Mailer"
Cohesion: 0.09
Nodes (31): Address, AddressError, AsyncSmtpTransport, builds_successfully_with_valid_config(), Mailer, MailerError, rejects_malformed_smtp_from(), Formatter (+23 more)

### Community 11 - "Local FS Storage"
Cohesion: 0.17
Nodes (22): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes, Duration (+14 more)

### Community 12 - "Users Handler"
Cohesion: 0.14
Nodes (22): Json, Result, State, update_me(), AuthState, AuthUser, bearer_token(), resolve_identity() (+14 more)

### Community 13 - "App Cache"
Cohesion: 0.30
Nodes (16): AtomicU32, AppCache, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), AppCacheProvider (+8 more)

### Community 14 - "TS App Config"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 15 - "Backend Review Skill"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 16 - "Design Tokens"
Cohesion: 0.10
Nodes (22): DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page (+14 more)

### Community 17 - "TS Node Config"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 18 - "Users Service"
Cohesion: 0.21
Nodes (16): bootstrap_admin(), AppConfig, PgPool, Result, Uuid, update_me(), AppError, Error (+8 more)

### Community 19 - "ESLint Plugins"
Cohesion: 0.11
Nodes (19): @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, devDependencies, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, openapi-typescript (+11 more)

### Community 20 - "Users DTOs"
Cohesion: 0.18
Nodes (14): From, Option, Role, Self, String, Uuid, UpdateMeDto, UserDto (+6 more)

### Community 21 - "MCP Config"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 22 - "App Layouts"
Cohesion: 0.15
Nodes (10): App(), AuthLayout(), RootLayout(), Providers(), ProvidersProps, queryClient, AuthFlow, ListingDetail (+2 more)

### Community 23 - "Auth Repository"
Cohesion: 0.33
Nodes (15): create_account(), db_err(), email_exists(), find_by_hash(), find_user_by_email(), insert_refresh_token(), revoke(), revoke_all_for_user() (+7 more)

### Community 24 - "Listings Repository"
Cohesion: 0.26
Nodes (15): count_listings(), find_listing_by_id(), find_media_for_listing(), list_listings(), ListingFilters, push_filters(), ListingType, Option (+7 more)

### Community 25 - "Error Envelope"
Cohesion: 0.24
Nodes (12): ErrorBody, ErrorEnvelope, parse_envelope(), Response, String, test_bad_request_carries_detail(), test_internal_error_is_500(), test_listing_not_found_produces_correct_envelope() (+4 more)

### Community 26 - "Auth DTOs"
Cohesion: 0.26
Nodes (13): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+5 more)

### Community 27 - "Storage Key Utils"
Cohesion: 0.25
Nodes (13): assert_key_shape(), avatar_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key(), listing_media_key_has_expected_shape(), owner_request_document_key() (+5 more)

### Community 28 - "Storage Provider Trait"
Cohesion: 0.26
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 29 - "Extraction Spec Rules"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 30 - "Frontend Dependencies"
Cohesion: 0.15
Nodes (13): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+5 more)

### Community 31 - "NPM Scripts"
Cohesion: 0.17
Nodes (12): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+4 more)

### Community 32 - "Request Logging"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), Response, String, two_calls_without_header_produce_distinct_ids(), HeaderMap (+2 more)

### Community 33 - "Graphify Export Steps"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 34 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 35 - "Graphify Query Flow"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 36 - "Docker Compose Services & Email Templates"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 37 - "MokaStore Implementation"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 38 - "Users Repository"
Cohesion: 0.50
Nodes (8): admin_exists(), find_is_active(), Option, PgPool, Result, Uuid, update_profile(), upsert_admin()

### Community 39 - "CI Frontend Workflow"
Cohesion: 0.39
Nodes (9): src/shared/api/types.ts (generated OpenAPI types), CI Frontend Workflow, frontend_build job, frontend_codegen job (OpenAPI TS codegen), frontend_docker job, frontend_install job, frontend_lint job (ESLint), frontend_prettier job (+1 more)

### Community 40 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 41 - "Video/Audio Transcription Flow"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 42 - "Package Manifest"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 43 - "Project CLAUDE.md Instructions"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 44 - "Graphify Watch/Add Flow"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 45 - "Multi-Repo Merge Flow"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 46 - "Auth Model"
Cohesion: 0.47
Nodes (5): NewAccount, RefreshTokenLookup, Option, Role, Uuid

### Community 47 - "React/TS Rules File"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 48 - "Graphify Update Subcommands"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 49 - "DB Connection"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 50 - "Graphify Manifest & Honesty Rules"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 51 - "Graphify Build & Cluster Steps"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 52 - "Codex Subagent Dispatch"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 53 - "OTP Code Generation"
Cohesion: 0.67
Nodes (3): generate_otp_code(), generated_code_is_always_six_digits(), String

### Community 54 - "Graphify Hook Install"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 55 - "Security Scan Workflows"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 56 - "Docker Compose Configs"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 57 - "Owner Request Email Templates"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 60 - "Project Conventions"
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
- **229 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+224 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **45 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `react-typecrypt.md Rules File`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `Embedded React/TypeScript Rules Content`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `Users Service` to `API Docs & App Server`, `Listings DTOs`, `Hashing Utils`, `Users Repository`, `Auth Service`, `Mailer`, `Local FS Storage`, `Users Handler`, `Auth Repository`, `Listings Repository`, `Error Envelope`, `Storage Provider Trait`?**
  _High betweenness centrality (0.155) - this node is a cross-community bridge._
- **Why does `AppState` connect `API Docs & App Server` to `Listings DTOs`, `Users Handler`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _229 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Listings API & Types` be split into smaller, more focused modules?**
  _Cohesion score 0.05372405372405373 - nodes in this community are weakly interconnected._