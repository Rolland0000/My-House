# Graph Report - /Users/hermann/Documents/M@Vie/My-/My-House  (2026-08-23)

## Corpus Check
- 181 files · ~78,956 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1066 nodes · 1853 edges · 104 communities (88 shown, 16 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 87 edges (avg confidence: 0.86)
- Token cost: 81,799 input · 0 output

## Community Hubs (Navigation)
- Frontend App Shell & Routing
- Auth Model & Refresh Token Repository
- App Cache & Refresh Replay Cache
- Listing Detail DTOs & Models
- Database Rules (Agents)
- JWT Crypto & AuthUser Extractor
- AppConfig, Env Parsing & Storage Provider Mod
- Frontend Listings API
- Frontend Runtime Dependencies
- LocalFsStorage Implementation
- Frontend Dev Dependencies (ESLint, Prettier, Tailwind, Types)
- Listings Handler & Pagination
- Frontend TS App Config
- Backend Code Review Skill & Invariants
- CI Workflows & OpenAPI Type Generation
- Storage Key Generation & Typed IDs
- Frontend TS Node Config
- Mailer & SMTP Config
- AppState & Token Decoder
- MCP Server Config
- AppState, API Doc & Router Bootstrap
- Notification Templates Service
- AppError Centralized Type
- Architecture Doc — Core Flows & Modules
- Project CLAUDE.md Instructions
- StorageProvider Trait Core
- Graphify Extraction Spec
- Technical Spec — DB Triggers & Contact Endpoint
- App Server Bootstrap & Main Entrypoint
- Request ID / Logging Middleware
- Auth DTOs, Refresh & Logout Handlers
- Graphify Export Steps (Wiki/Neo4j/FalkorDB/SVG/GraphML)
- No-Proxy & Owner Request Docs Flow
- Health Check Endpoint
- Graphify Query/Path/Explain Flow
- Design Tokens & Listings Endpoints
- Docker Compose Services (Dev)
- MokaStore Implementation
- Docker Rules Skill
- Graphify Video Transcription Step
- OTP Auth Flow & ADR
- MokaStore Generic Methods
- Graphify Add/Watch Ingest
- Graphify Multi-Repo Merge Flow
- Storage ADR & Owner Requests
- README Skill (embeds duplicated React/TS rules)
- Graphify Update/Cluster-Only Subcommands
- Listing Management Wireframes
- Database Connection Pool
- Graphify Manifest & Honesty Rules
- Graphify Build Pipeline Steps
- Graphify Codex Multi-Agent Spawn
- Graphify CLAUDE.md/Hook Integration
- CI Security Workflows (CodeQL/Gitleaks)
- Docker Compose Files (Structural)
- Owner Request Email Templates
- Pre-Tool-Use Hook Script
- TS Project References Config
- README & Git Conventions
- Moka Cache ADR
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
1. `AppError` - 73 edges
2. `AppState` - 32 edges
3. `cn()` - 27 edges
4. `AppCacheProvider` - 19 edges
5. `perform_refresh()` - 19 edges
6. `compilerOptions` - 18 edges
7. `resolve_identity()` - 16 edges
8. `compilerOptions` - 16 edges
9. `ConfigError` - 13 edges
10. `AppConfig` - 13 edges

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

## Communities (104 total, 16 thin omitted)

### Community 0 - "Frontend App Shell & Routing"
Cohesion: 0.05
Nodes (53): App(), RootLayout(), Providers(), ProvidersProps, queryClient, ListingDetail, ListingFeed, router (+45 more)

### Community 1 - "Auth Model & Refresh Token Repository"
Cohesion: 0.09
Nodes (51): RefreshTokenLookup, Uuid, db_err(), find_by_hash(), revoke(), revoke_all_for_user(), rotate(), Error (+43 more)

### Community 2 - "App Cache & Refresh Replay Cache"
Cohesion: 0.09
Nodes (39): AppCache, build_cache_provider(), build_refresh_replay_cache_provider(), Arc, RefreshTokenId, Self, String, Uuid (+31 more)

### Community 3 - "Listing Detail DTOs & Models"
Cohesion: 0.09
Nodes (43): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+35 more)

### Community 4 - "Database Rules (Agents)"
Cohesion: 0.05
Nodes (44): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+36 more)

### Community 5 - "JWT Crypto & AuthUser Extractor"
Cohesion: 0.11
Nodes (30): Algorithm, Claims, encode_with_exp(), expired_token_is_rejected_with_token_expired(), hash_refresh_token(), issue_access_token(), issued_token_expires_exactly_ttl_seconds_after_issuance(), jwt_token_decoder_adapter_delegates_correctly() (+22 more)

### Community 6 - "AppConfig, Env Parsing & Storage Provider Mod"
Cohesion: 0.14
Nodes (29): app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), rejects_invalid_app_env() (+21 more)

### Community 7 - "Frontend Listings API"
Cohesion: 0.13
Nodes (27): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+19 more)

### Community 8 - "Frontend Runtime Dependencies"
Cohesion: 0.06
Nodes (32): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+24 more)

### Community 9 - "LocalFsStorage Implementation"
Cohesion: 0.17
Nodes (22): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes, Duration (+14 more)

### Community 10 - "Frontend Dev Dependencies (ESLint, Prettier, Tailwind, Types)"
Cohesion: 0.06
Nodes (31): eslint, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks (+23 more)

### Community 11 - "Listings Handler & Pagination"
Cohesion: 0.12
Nodes (23): get_by_id(), list(), Json, Path, Result, State, Uuid, PaginatedResponse (+15 more)

### Community 12 - "Frontend TS App Config"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 13 - "Backend Code Review Skill & Invariants"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 14 - "CI Workflows & OpenAPI Type Generation"
Cohesion: 0.14
Nodes (22): ADR-06: utoipa + openapi-typescript over manual TS types, TypeScript type generation pipeline (utoipa → openapi-typescript → types.ts), src/shared/api/types.ts (generated OpenAPI types), CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest) (+14 more)

### Community 15 - "Storage Key Generation & Typed IDs"
Cohesion: 0.14
Nodes (13): assert_key_shape(), avatar_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key(), listing_media_key_has_expected_shape(), owner_request_document_key() (+5 more)

### Community 16 - "Frontend TS Node Config"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 17 - "Mailer & SMTP Config"
Cohesion: 0.16
Nodes (15): Address, AddressError, AsyncSmtpTransport, builds_successfully_with_valid_config(), Mailer, MailerError, rejects_malformed_smtp_from(), Formatter (+7 more)

### Community 18 - "AppState & Token Decoder"
Cohesion: 0.19
Nodes (11): Inner, Arc, PgPool, RefreshTokenId, Self, StorageProvider, String, Uuid (+3 more)

### Community 19 - "MCP Server Config"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 20 - "AppState, API Doc & Router Bootstrap"
Cohesion: 0.29
Nodes (12): ApiDoc, AppState, admin_router(), build_router(), merged_router(), openapi_spec(), owner_router(), public_router() (+4 more)

### Community 21 - "Notification Templates Service"
Cohesion: 0.19
Nodes (14): NotificationTemplate, OtpTemplate, OwnerRequestApprovedTemplate, OwnerRequestReceivedTemplate, OwnerRequestRejectedTemplate, render(), renders_otp_template_with_sample_context(), renders_owner_request_approved_template_with_sample_context() (+6 more)

### Community 22 - "AppError Centralized Type"
Cohesion: 0.24
Nodes (11): ErrorBody, ErrorEnvelope, parse_envelope(), Response, StatusCode, String, test_bad_request_carries_detail(), test_internal_error_is_500() (+3 more)

### Community 23 - "Architecture Doc — Core Flows & Modules"
Cohesion: 0.29
Nodes (14): ARCHITECTURE_v1.2.md — arc42 Software Architecture Document, Core Loop — Listing Publication and Discovery Flow (§6.3), admin module (moderation, owner request validation), auth module (OTP passwordless, JWT access+refresh, rate limiting), contact module (owner phone reveal), listings module (CRUD, status, cover photo), media module (upload, StorageProvider abstraction, magic-byte validation), notifications module (email OTP, email notifications) (+6 more)

### Community 24 - "Project CLAUDE.md Instructions"
Cohesion: 0.17
Nodes (13): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), ADR-02: OTP Passwordless over Password/OAuth, ADR-09: Refresh token rotation on each use (+5 more)

### Community 25 - "StorageProvider Trait Core"
Cohesion: 0.26
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 26 - "Graphify Extraction Spec"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 27 - "Technical Spec — DB Triggers & Contact Endpoint"
Cohesion: 0.26
Nodes (13): R-08: fn_update_listing_search_vector trigger SELECT-per-row cost, TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP, GET /listings/:id/contact endpoint, fn_cascade_owner_name_to_listings() trigger function, fn_set_updated_at() trigger function, fn_update_listing_search_vector() trigger function, listing_media table, listings table (+5 more)

### Community 28 - "App Server Bootstrap & Main Entrypoint"
Cohesion: 0.24
Nodes (7): AppServer, Error, Result, Self, shutdown_signal(), Box, SocketAddr

### Community 29 - "Request ID / Logging Middleware"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), Response, String, two_calls_without_header_produce_distinct_ids(), HeaderMap (+2 more)

### Community 30 - "Auth DTOs, Refresh & Logout Handlers"
Cohesion: 0.29
Nodes (9): RefreshResponse, RefreshTokenDto, String, logout(), refresh(), Json, Result, State (+1 more)

### Community 31 - "Graphify Export Steps (Wiki/Neo4j/FalkorDB/SVG/GraphML)"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 32 - "No-Proxy & Owner Request Docs Flow"
Cohesion: 0.18
Nodes (11): No-proxy principle for public file reads (§7.3), Owner Request and Validation Runtime Flow (§6.2), Owner-requests documents proxy exception (admin-only read), R-09: Single admin account, no operational redundancy, GET /admin/owner-requests/:id/documents/:doc_id endpoint, frontend/nginx.conf (SPA fallback, /media/ static, /api proxy), mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire') (+3 more)

### Community 33 - "Health Check Endpoint"
Cohesion: 0.29
Nodes (7): check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus

### Community 34 - "Graphify Query/Path/Explain Flow"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 35 - "Design Tokens & Listings Endpoints"
Cohesion: 0.20
Nodes (10): GET/POST /listings endpoints, GET /search endpoint, DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-13-feed-detail.html wireframe (+2 more)

### Community 36 - "Docker Compose Services (Dev)"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 37 - "MokaStore Implementation"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 38 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 39 - "Graphify Video Transcription Step"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 40 - "OTP Auth Flow & ADR"
Cohesion: 0.25
Nodes (8): ADR-08: Single OTP endpoint over separate login/register, OTP Login/Signup Runtime Flow (§6.1), POST /auth/otp/request endpoint, POST /auth/otp/verify endpoint, mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 41 - "MokaStore Generic Methods"
Cohesion: 0.52
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 42 - "Graphify Add/Watch Ingest"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 43 - "Graphify Multi-Repo Merge Flow"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 44 - "Storage ADR & Owner Requests"
Cohesion: 0.29
Nodes (7): ADR-05: StorageProvider trait + LocalFsStorage over S3-at-MVP, LocalFsStorage implementation, R-06: Filesystem storage not shared across instances, StorageProvider trait, POST /owner-requests endpoint, Backend Environment Variables (.env.example), StorageProvider trait interface (upload/read/delete/presigned_url)

### Community 45 - "README Skill (embeds duplicated React/TS rules)"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 46 - "Graphify Update/Cluster-Only Subcommands"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 47 - "Listing Management Wireframes"
Cohesion: 0.33
Nodes (6): POST /media/upload endpoint, mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, Identity document dropzone component

### Community 48 - "Database Connection Pool"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 49 - "Graphify Manifest & Honesty Rules"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 50 - "Graphify Build Pipeline Steps"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 51 - "Graphify Codex Multi-Agent Spawn"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 52 - "Graphify CLAUDE.md/Hook Integration"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 53 - "CI Security Workflows (CodeQL/Gitleaks)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 54 - "Docker Compose Files (Structural)"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 55 - "Owner Request Email Templates"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 58 - "README & Git Conventions"
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
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `react-typecrypt.md Rules File`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `Embedded React/TypeScript Rules Content`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `Auth Model & Refresh Token Repository` to `App Cache & Refresh Replay Cache`, `Listing Detail DTOs & Models`, `JWT Crypto & AuthUser Extractor`, `LocalFsStorage Implementation`, `Listings Handler & Pagination`, `Notification Templates Service`, `AppError Centralized Type`, `StorageProvider Trait Core`, `Auth DTOs, Refresh & Logout Handlers`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState, API Doc & Router Bootstrap` to `Health Check Endpoint`, `App Cache & Refresh Replay Cache`, `Listings Handler & Pagination`, `AppState & Token Decoder`, `App Server Bootstrap & Main Entrypoint`, `Auth DTOs, Refresh & Logout Handlers`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `AppConfig, Env Parsing & Storage Provider Mod` to `Mailer & SMTP Config`, `AppState & Token Decoder`, `AppState, API Doc & Router Bootstrap`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _205 weakly-connected nodes found - possible documentation gaps or missing edges._