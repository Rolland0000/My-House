# Graph Report - .  (2026-08-22)

## Corpus Check
- 71 files · ~75,839 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 949 nodes · 1513 edges · 109 communities (86 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 74 edges (avg confidence: 0.87)
- Token cost: 55,475 input · 0 output

## Community Hubs (Navigation)
- Frontend App Shell & Routing
- Backend App Bootstrap & Mail
- AppCache & AuthUser Extractor
- StorageProvider Trait Core
- Listing Detail DTOs
- Frontend Listings API
- AppConfig & Env Parsing
- LocalFsStorage Implementation
- Frontend TS App Config
- OpenAPI Type Generation Pipeline
- Storage Key Generation
- Frontend TS Node Config
- Pagination DTOs
- ESLint Config (Frontend)
- MCP Server Config
- React/TS Coding Rules
- Notification Templates Service
- Architecture Doc — Core Flows & Modules
- Project CLAUDE.md Instructions
- Graphify Extraction Spec
- Frontend Runtime Dependencies
- Technical Spec — DB Triggers & Contact Endpoint
- Frontend NPM Scripts
- Request ID / Logging Middleware
- Graphify Export Steps (Wiki/Neo4j/FalkorDB/SVG/GraphML)
- No-Proxy & Owner Request Docs Flow
- Rust Concurrency & Error Handling Rules
- Graphify Query/Path/Explain Flow
- Design Tokens & Listings Endpoints
- Docker Compose Services (Dev)
- MokaStore Implementation
- Database Rules (Agents)
- General Coding Guidelines (Agents)
- Database Rules (Claude)
- Graphify Video Transcription Step
- OTP Auth Flow & ADR
- Frontend package.json
- MokaStore Generic Methods
- Graphify Add/Watch Ingest
- Graphify Multi-Repo Merge Flow
- Storage ADR & Owner Requests
- Backend Code Review Skill
- Storage Provider Bootstrap
- Graphify Update/Cluster-Only Subcommands
- Listing Management Wireframes
- Database Connection Pool
- Graphify Manifest & Honesty Rules
- Graphify Build Pipeline Steps
- Graphify Codex Multi-Agent Spawn
- Ticket Conventions (MH-XXX)
- Graphify CLAUDE.md/Hook Integration
- CI Security Workflows (CodeQL/Gitleaks)
- Docker Compose Files (Structural)
- Owner Request Email Templates
- Pre-Tool-Use Hook Script
- TS Project References Config
- README & Git Conventions
- Moka Cache ADR
- ESLint React Refresh Plugin
- Globals Dependency
- Prettier Dependency
- Tailwind CSS Dependency
- React DOM Types Dependency
- TypeScript ESLint Dependency
- Pre-Commit Hook Script
- Graphify Slash Command Trigger
- ADR: Modular Monolith
- ADR: Postgres Full-Text Search
- ADR: Seeker Default Role
- AppError Centralized Type
- Health Check & Graceful Shutdown
- Pagination Standard Spec
- Listings Price Index Gap
- Auth Refresh Endpoint
- Backend Dockerfile
- Frontend Dockerfile
- Send Trait Bound
- Sync Trait Bound

## God Nodes (most connected - your core abstractions)
1. `AppError` - 44 edges
2. `AppState` - 29 edges
3. `cn()` - 19 edges
4. `compilerOptions` - 18 edges
5. `compilerOptions` - 16 edges
6. `resolve_identity()` - 16 edges
7. `ConfigError` - 13 edges
8. `AppConfig` - 13 edges
9. `AppCacheProvider` - 13 edges
10. `scripts` - 12 edges

## Surprising Connections (you probably didn't know these)
- `Step B2 - Dispatch ALL subagents in a single message` --semantically_similar_to--> `Step B2 - Dispatch ALL subagents (Codex spawn_agent)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md
- `Extraction subagent prompt (full)` --semantically_similar_to--> `Extraction subagent prompt (compact)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/extraction-spec.md → .codex/skills/graphify/references/extraction-spec.md
- `--update (incremental re-extraction)` --semantically_similar_to--> `--update (incremental re-extraction) (Codex)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/update.md → .codex/skills/graphify/references/update.md
- `AGENTS.md — graphify trigger instructions` --semantically_similar_to--> `CLAUDE.md — MyHouse project instructions`  [INFERRED] [semantically similar]
  AGENTS.md → CLAUDE.md
- `README Writing Rules (MyHouse)` --conceptually_related_to--> `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md → .agents/rules/react-typecrypt.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Duplicated Rule Sets Between .agents/rules and .claude/rules** — agents_rules_database_rules_myhouse_db_rules, claude_rules_database_myhouse_db_rules, agents_rules_rust_general_rules_myhouse_rust_rules [INFERRED 0.85]
- **Backend and Frontend Code Review Skills Share Four-Phase Pattern and Severity Model** — claude_skills_code_review_backend_skill_backend_review_skill, claude_skills_code_review_frontend_skill_frontend_review_skill, claude_skills_code_review_backend_skill_severity_labels [INFERRED 0.85]
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Extract-Build-Cluster pipeline flow (Steps 3-5)** — claude_skills_graphify_skill_step3_extract_entities, claude_skills_graphify_skill_part_a_structural_extraction, claude_skills_graphify_skill_part_b_semantic_extraction, claude_skills_graphify_skill_part_c_merge_ast_semantic, claude_skills_graphify_skill_step4_build_cluster_analyze, claude_skills_graphify_skill_step5_label_communities [EXTRACTED 1.00]
- **Incremental-update integrity guards (stamping, pruning, manifest)** — claude_skills_graphify_skill_stamped_manifest_files_2015, claude_skills_graphify_references_update_replace_on_reextract_1344, claude_skills_graphify_references_update_incremental_update, claude_skills_graphify_skill_step9_manifest_cost_cleanup [INFERRED 0.85]
- **query/path/explain feedback-loop family** — claude_skills_graphify_references_query_traversal, claude_skills_graphify_references_query_graphify_path, claude_skills_graphify_references_query_graphify_explain, claude_skills_graphify_references_query_save_result [EXTRACTED 1.00]
- **Owner Upgrade Request Flow (users → media → StorageProvider → admin)** — docs_architecture_v1_2_md_module_users, docs_architecture_v1_2_md_module_media, docs_architecture_v1_2_md_storageprovider_trait, docs_architecture_v1_2_md_module_admin [EXTRACTED 1.00]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin, frontend_src_shared_api_types_ts, docs_technical_spec_mvp_v1_2_md_ts_codegen_pipeline, github_workflows_ci_frontend_yml_frontend_codegen [EXTRACTED 1.00]

## Communities (109 total, 23 thin omitted)

### Community 0 - "Frontend App Shell & Routing"
Cohesion: 0.05
Nodes (54): App(), RootLayout(), Providers(), ProvidersProps, queryClient, ListingDetail, ListingFeed, router (+46 more)

### Community 1 - "Backend App Bootstrap & Mail"
Cohesion: 0.05
Nodes (52): Address, AddressError, AsyncSmtpTransport, ApiDoc, AppServer, Error, Result, Self (+44 more)

### Community 2 - "AppCache & AuthUser Extractor"
Cohesion: 0.08
Nodes (41): AtomicUsize, AppCache, build_cache_provider(), Arc, Self, Uuid, AppCacheProvider, Send (+33 more)

### Community 3 - "StorageProvider Trait Core"
Cohesion: 0.07
Nodes (45): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage (+37 more)

### Community 4 - "Listing Detail DTOs"
Cohesion: 0.11
Nodes (36): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+28 more)

### Community 5 - "Frontend Listings API"
Cohesion: 0.14
Nodes (26): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+18 more)

### Community 6 - "AppConfig & Env Parsing"
Cohesion: 0.16
Nodes (26): app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), rejects_invalid_app_env() (+18 more)

### Community 7 - "LocalFsStorage Implementation"
Cohesion: 0.17
Nodes (19): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_returns_previously_uploaded_bytes(), Bytes, Duration, Path (+11 more)

### Community 8 - "Frontend TS App Config"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 9 - "OpenAPI Type Generation Pipeline"
Cohesion: 0.14
Nodes (22): ADR-06: utoipa + openapi-typescript over manual TS types, TypeScript type generation pipeline (utoipa → openapi-typescript → types.ts), src/shared/api/types.ts (generated OpenAPI types), CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest) (+14 more)

### Community 10 - "Storage Key Generation"
Cohesion: 0.14
Nodes (13): assert_key_shape(), avatar_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key(), listing_media_key_has_expected_shape(), owner_request_document_key() (+5 more)

### Community 11 - "Frontend TS Node Config"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 12 - "Pagination DTOs"
Cohesion: 0.18
Nodes (15): PaginatedResponse, PaginatedResponse<T>, PaginationMeta, Option, Self, T, Vec, test_defaults_applied_when_none() (+7 more)

### Community 13 - "ESLint Config (Frontend)"
Cohesion: 0.11
Nodes (19): eslint, @eslint/js, eslint-plugin-react-hooks, devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks, openapi-typescript (+11 more)

### Community 14 - "MCP Server Config"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 15 - "React/TS Coding Rules"
Cohesion: 0.15
Nodes (15): Functional Component Conventions (no React.FC), api.ts Single Fetch Entry Point Rule, Locked Frontend Folder Structure (app/features/shared), React/TypeScript Rules (Agents), Functional Component Conventions (Claude rules), api.ts Single Fetch Entry Point Rule (Claude rules), Locked Frontend Folder Structure (Claude rules), React/TypeScript Rules (Claude rules) (+7 more)

### Community 16 - "Notification Templates Service"
Cohesion: 0.19
Nodes (14): NotificationTemplate, OtpTemplate, OwnerRequestApprovedTemplate, OwnerRequestReceivedTemplate, OwnerRequestRejectedTemplate, render(), renders_otp_template_with_sample_context(), renders_owner_request_approved_template_with_sample_context() (+6 more)

### Community 17 - "Architecture Doc — Core Flows & Modules"
Cohesion: 0.29
Nodes (14): ARCHITECTURE_v1.2.md — arc42 Software Architecture Document, Core Loop — Listing Publication and Discovery Flow (§6.3), admin module (moderation, owner request validation), auth module (OTP passwordless, JWT access+refresh, rate limiting), contact module (owner phone reveal), listings module (CRUD, status, cover photo), media module (upload, StorageProvider abstraction, magic-byte validation), notifications module (email OTP, email notifications) (+6 more)

### Community 18 - "Project CLAUDE.md Instructions"
Cohesion: 0.17
Nodes (13): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), ADR-02: OTP Passwordless over Password/OAuth, ADR-09: Refresh token rotation on each use (+5 more)

### Community 19 - "Graphify Extraction Spec"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 20 - "Frontend Runtime Dependencies"
Cohesion: 0.15
Nodes (13): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+5 more)

### Community 21 - "Technical Spec — DB Triggers & Contact Endpoint"
Cohesion: 0.26
Nodes (13): R-08: fn_update_listing_search_vector trigger SELECT-per-row cost, TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP, GET /listings/:id/contact endpoint, fn_cascade_owner_name_to_listings() trigger function, fn_set_updated_at() trigger function, fn_update_listing_search_vector() trigger function, listing_media table, listings table (+5 more)

### Community 22 - "Frontend NPM Scripts"
Cohesion: 0.17
Nodes (12): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+4 more)

### Community 23 - "Request ID / Logging Middleware"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), Response, String, two_calls_without_header_produce_distinct_ids(), HeaderMap (+2 more)

### Community 24 - "Graphify Export Steps (Wiki/Neo4j/FalkorDB/SVG/GraphML)"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 25 - "No-Proxy & Owner Request Docs Flow"
Cohesion: 0.18
Nodes (11): No-proxy principle for public file reads (§7.3), Owner Request and Validation Runtime Flow (§6.2), Owner-requests documents proxy exception (admin-only read), R-09: Single admin account, no operational redundancy, GET /admin/owner-requests/:id/documents/:doc_id endpoint, frontend/nginx.conf (SPA fallback, /media/ static, /api proxy), mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire') (+3 more)

### Community 26 - "Rust Concurrency & Error Handling Rules"
Cohesion: 0.20
Nodes (10): Concurrency and Async Rules (no blocking locks across await), Error Handling (thiserror/anyhow), Rust General Rules (Agents), Ownership and Types Conventions, Testing and Quality Gates (cargo check/fmt/clippy), Concurrency and Async Rules (Claude rules), Error Handling (thiserror/anyhow) (Claude rules), MCP Usage Policy (GitHub/PostgreSQL/Git/Context7/Filesystem) (+2 more)

### Community 27 - "Graphify Query/Path/Explain Flow"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 28 - "Design Tokens & Listings Endpoints"
Cohesion: 0.20
Nodes (10): GET/POST /listings endpoints, GET /search endpoint, DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-13-feed-detail.html wireframe (+2 more)

### Community 29 - "Docker Compose Services (Dev)"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 30 - "MokaStore Implementation"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 31 - "Database Rules (Agents)"
Cohesion: 0.29
Nodes (8): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern

### Community 32 - "General Coding Guidelines (Agents)"
Cohesion: 0.25
Nodes (8): General Coding Guidelines (Agents), Architecture Invariants (modular monolith, layering, AppError), APP_ENV Environment Scoping Rule, MyHouse Project Instructions (Agents), Locked Tech Stack (Rust/Axum/React/Postgres/moka/Docker), General Coding Guidelines (Claude rules), Docker Rules Skill (MyHouse), Dockerfile Security Rules (non-root, pinned versions, no secrets)

### Community 33 - "Database Rules (Claude)"
Cohesion: 0.25
Nodes (8): Cascade and Filesystem Cleanup Ordering (Claude rules), Migration Conventions (Claude rules), Database Rules (Claude, sqlx/PostgreSQL), Listings/Search Index Performance Rules (Claude rules), sqlx Query Conventions (Claude rules), Schema Conventions (Claude rules), SQL Injection Prevention / Sensitive Column Exclusion (Claude rules), Repository Test Transaction Rollback Pattern (Claude rules)

### Community 34 - "Graphify Video Transcription Step"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 35 - "OTP Auth Flow & ADR"
Cohesion: 0.25
Nodes (8): ADR-08: Single OTP endpoint over separate login/register, OTP Login/Signup Runtime Flow (§6.1), POST /auth/otp/request endpoint, POST /auth/otp/verify endpoint, mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 36 - "Frontend package.json"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 37 - "MokaStore Generic Methods"
Cohesion: 0.52
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 38 - "Graphify Add/Watch Ingest"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 39 - "Graphify Multi-Repo Merge Flow"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 40 - "Storage ADR & Owner Requests"
Cohesion: 0.29
Nodes (7): ADR-05: StorageProvider trait + LocalFsStorage over S3-at-MVP, LocalFsStorage implementation, R-06: Filesystem storage not shared across instances, StorageProvider trait, POST /owner-requests endpoint, Backend Environment Variables (.env.example), StorageProvider trait interface (upload/read/delete/presigned_url)

### Community 41 - "Backend Code Review Skill"
Cohesion: 0.33
Nodes (6): Key Locked Decisions (OTP auth, roles, refresh tokens, upload security), Code Review Backend Skill (Rust/Axum/MyHouse), Four-Phase Backend Review Process, MyHouse Backend Invariants (auth, refresh tokens, storage keys), Backend Review Severity Labels (blocking/important/nit/suggestion), Backend Review Checklist Reference

### Community 42 - "Storage Provider Bootstrap"
Cohesion: 0.47
Nodes (4): build_storage_provider(), Arc, StorageProvider, main()

### Community 43 - "Graphify Update/Cluster-Only Subcommands"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 44 - "Listing Management Wireframes"
Cohesion: 0.33
Nodes (6): POST /media/upload endpoint, mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, Identity document dropzone component

### Community 45 - "Database Connection Pool"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 46 - "Graphify Manifest & Honesty Rules"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 47 - "Graphify Build Pipeline Steps"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 48 - "Graphify Codex Multi-Agent Spawn"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 49 - "Ticket Conventions (MH-XXX)"
Cohesion: 0.50
Nodes (4): MH-XXX Ticket Conventions, MH-XX Ticket Format Template, GitHub Ticket Generation Skill (MyHouse), Vertical Slicing Rule (BE/FE sub-tickets)

### Community 50 - "Graphify CLAUDE.md/Hook Integration"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 51 - "CI Security Workflows (CodeQL/Gitleaks)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 52 - "Docker Compose Files (Structural)"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 53 - "Owner Request Email Templates"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 56 - "README & Git Conventions"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `React/TypeScript Rules (Agents)` → `README Writing Rules (MyHouse)`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: conceptually_related_to

## Knowledge Gaps
- **190 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+185 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `React/TypeScript Rules (Agents)` and `README Writing Rules (MyHouse)`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **Why does `AppError` connect `StorageProvider Trait Core` to `Backend App Bootstrap & Mail`, `AppCache & AuthUser Extractor`, `Listing Detail DTOs`, `LocalFsStorage Implementation`, `Notification Templates Service`?**
  _High betweenness centrality (0.076) - this node is a cross-community bridge._
- **Why does `AppState` connect `Backend App Bootstrap & Mail` to `AppCache & AuthUser Extractor`, `Listing Detail DTOs`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `AppConfig & Env Parsing` to `Backend App Bootstrap & Mail`, `Storage Provider Bootstrap`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _190 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Frontend App Shell & Routing` be split into smaller, more focused modules?**
  _Cohesion score 0.05052631578947368 - nodes in this community are weakly interconnected._