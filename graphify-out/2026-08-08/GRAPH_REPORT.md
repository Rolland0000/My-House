# Graph Report - My-House  (2026-08-08)

## Corpus Check
- 140 files · ~71,765 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 781 nodes · 1214 edges · 86 communities (73 shown, 13 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 68 edges (avg confidence: 0.88)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `78b2d74c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- AppState
- config/mod.rs
- scripts
- CI Backend Workflow
- devDependencies
- components/index.ts
- mh-15-owner-request.html wireframe
- ARCHITECTURE_v1.2.md — arc42 Software Architecture Document
- TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP
- compilerOptions
- AppError
- compilerOptions
- .new
- .mcp.json
- React/TypeScript Rules (Agents)
- Extraction subagent prompt (full)
- Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark
- Rust General Rules (Agents)
- save-result feedback loop
- MyHouse Database Rules (sqlx/PostgreSQL)
- MyHouse Project Instructions (Agents)
- Database Rules (Claude, sqlx/PostgreSQL)
- /graphify command
- /graphify add <url>
- Multiple repos cross-repo graph merge
- Code Review Backend Skill (Rust/Axum/MyHouse)
- --update (incremental re-extraction)
- connect_db
- Step 9 - Save manifest, update cost tracker, clean up, and report
- Step 4 - Build graph, cluster, analyze, generate outputs
- Extraction subagent prompt (compact)
- MH-XXX Ticket Conventions
- git commit hook (graphify hook install)
- analyze job (rust + javascript-typescript matrix)
- pre-tool-use.sh
- tsconfig.json
- ADR-04: moka in-memory over Redis
- ListingDetailDto
- graphify Slash Command Trigger (.claude/CLAUDE.md)
- ADR-01: Modular Monolith over Microservices
- ADR-03: PostgreSQL full-text over Meilisearch/Elasticsearch
- ADR-07: seeker default role over role choice at signup
- AppError centralized error type (§8.2)
- Health Check and Graceful Shutdown (§8.7)
- Pagination standard (§8.3)
- R-07: Absence of index on listings.price
- POST /auth/refresh endpoint
- ListingFeed.tsx
- CLAUDE.md — MyHouse project instructions
- DESIGN_TOKENS.md (MH-17) — Design Token Set
- mh-12-auth-flow.html wireframe
- StorageProvider trait
- mh-14-listing-management.html wireframe
- pre-commit

## God Nodes (most connected - your core abstractions)
1. `AppError` - 29 edges
2. `cn()` - 27 edges
3. `AppState` - 21 edges
4. `compilerOptions` - 18 edges
5. `compilerOptions` - 16 edges
6. `ConfigError` - 13 edges
7. `ARCHITECTURE_v1.2.md — arc42 Software Architecture Document` - 13 edges
8. `LocalFsStorage` - 12 edges
9. `ListingSummaryDto` - 12 edges
10. `ListingDetailDto` - 12 edges

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
- **Duplicated Rule Sets Between .agents/rules and .claude/rules** — agents_rules_database_rules_myhouse_db_rules, claude_rules_database_myhouse_db_rules, agents_rules_rust_general_rules_myhouse_rust_rules [INFERRED 0.85]
- **Backend and Frontend Code Review Skills Share Four-Phase Pattern and Severity Model** — claude_skills_code_review_backend_skill_backend_review_skill, claude_skills_code_review_frontend_skill_frontend_review_skill, claude_skills_code_review_backend_skill_severity_labels [INFERRED 0.85]
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Extract-Build-Cluster pipeline flow (Steps 3-5)** — claude_skills_graphify_skill_step3_extract_entities, claude_skills_graphify_skill_part_a_structural_extraction, claude_skills_graphify_skill_part_b_semantic_extraction, claude_skills_graphify_skill_part_c_merge_ast_semantic, claude_skills_graphify_skill_step4_build_cluster_analyze, claude_skills_graphify_skill_step5_label_communities [EXTRACTED 1.00]
- **Incremental-update integrity guards (stamping, pruning, manifest)** — claude_skills_graphify_skill_stamped_manifest_files_2015, claude_skills_graphify_references_update_replace_on_reextract_1344, claude_skills_graphify_references_update_incremental_update, claude_skills_graphify_skill_step9_manifest_cost_cleanup [INFERRED 0.85]
- **query/path/explain feedback-loop family** — claude_skills_graphify_references_query_traversal, claude_skills_graphify_references_query_graphify_path, claude_skills_graphify_references_query_graphify_explain, claude_skills_graphify_references_query_save_result [EXTRACTED 1.00]
- **Owner Upgrade Request Flow (users → media → StorageProvider → admin)** — docs_architecture_v1_2_md_module_users, docs_architecture_v1_2_md_module_media, docs_architecture_v1_2_md_storageprovider_trait, docs_architecture_v1_2_md_module_admin [EXTRACTED 1.00]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin, frontend_src_shared_api_types_ts, docs_technical_spec_mvp_v1_2_md_ts_codegen_pipeline, github_workflows_ci_frontend_yml_frontend_codegen [EXTRACTED 1.00]

## Communities (86 total, 13 thin omitted)

### Community 0 - "AppState"
Cohesion: 0.09
Nodes (28): ApiDoc, AppServer, Error, Result, Self, shutdown_signal(), AppState, Inner (+20 more)

### Community 1 - "config/mod.rs"
Cohesion: 0.14
Nodes (29): app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), rejects_invalid_app_env() (+21 more)

### Community 2 - "scripts"
Cohesion: 0.06
Nodes (32): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+24 more)

### Community 3 - "CI Backend Workflow"
Cohesion: 0.09
Nodes (33): docker-compose.yml (root), backend-dev service (dev profile, cargo-watch), backend-prod service (prod profile, release binary), frontend-dev service (dev profile, vite dev server), frontend-prod service (prod profile, nginx static), postgres service, ADR-06: utoipa + openapi-typescript over manual TS types, backend/Dockerfile (multi-stage build) (+25 more)

### Community 4 - "devDependencies"
Cohesion: 0.06
Nodes (31): eslint, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks (+23 more)

### Community 5 - "components/index.ts"
Cohesion: 0.05
Nodes (53): App(), RootLayout(), Providers(), ProvidersProps, queryClient, ListingDetail, ListingFeed, router (+45 more)

### Community 6 - "mh-15-owner-request.html wireframe"
Cohesion: 0.18
Nodes (11): No-proxy principle for public file reads (§7.3), Owner Request and Validation Runtime Flow (§6.2), Owner-requests documents proxy exception (admin-only read), R-09: Single admin account, no operational redundancy, GET /admin/owner-requests/:id/documents/:doc_id endpoint, frontend/nginx.conf (SPA fallback, /media/ static, /api proxy), mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire') (+3 more)

### Community 7 - "ARCHITECTURE_v1.2.md — arc42 Software Architecture Document"
Cohesion: 0.29
Nodes (14): ARCHITECTURE_v1.2.md — arc42 Software Architecture Document, Core Loop — Listing Publication and Discovery Flow (§6.3), admin module (moderation, owner request validation), auth module (OTP passwordless, JWT access+refresh, rate limiting), contact module (owner phone reveal), listings module (CRUD, status, cover photo), media module (upload, StorageProvider abstraction, magic-byte validation), notifications module (email OTP, email notifications) (+6 more)

### Community 8 - "TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP"
Cohesion: 0.26
Nodes (13): R-08: fn_update_listing_search_vector trigger SELECT-per-row cost, TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP, GET /listings/:id/contact endpoint, fn_cascade_owner_name_to_listings() trigger function, fn_set_updated_at() trigger function, fn_update_listing_search_vector() trigger function, listing_media table, listings table (+5 more)

### Community 9 - "compilerOptions"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 10 - "AppError"
Cohesion: 0.07
Nodes (43): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_returns_previously_uploaded_bytes(), Bytes, Duration, Path (+35 more)

### Community 11 - "compilerOptions"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 12 - ".new"
Cohesion: 0.12
Nodes (23): get_by_id(), list(), Json, Path, Result, State, Uuid, PaginatedResponse (+15 more)

### Community 13 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 14 - "React/TypeScript Rules (Agents)"
Cohesion: 0.15
Nodes (15): Functional Component Conventions (no React.FC), api.ts Single Fetch Entry Point Rule, Locked Frontend Folder Structure (app/features/shared), React/TypeScript Rules (Agents), Functional Component Conventions (Claude rules), api.ts Single Fetch Entry Point Rule (Claude rules), Locked Frontend Folder Structure (Claude rules), React/TypeScript Rules (Claude rules) (+7 more)

### Community 15 - "Extraction subagent prompt (full)"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 17 - "Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 18 - "Rust General Rules (Agents)"
Cohesion: 0.20
Nodes (10): Concurrency and Async Rules (no blocking locks across await), Error Handling (thiserror/anyhow), Rust General Rules (Agents), Ownership and Types Conventions, Testing and Quality Gates (cargo check/fmt/clippy), Concurrency and Async Rules (Claude rules), Error Handling (thiserror/anyhow) (Claude rules), MCP Usage Policy (GitHub/PostgreSQL/Git/Context7/Filesystem) (+2 more)

### Community 19 - "save-result feedback loop"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 20 - "MyHouse Database Rules (sqlx/PostgreSQL)"
Cohesion: 0.29
Nodes (8): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern

### Community 21 - "MyHouse Project Instructions (Agents)"
Cohesion: 0.25
Nodes (8): General Coding Guidelines (Agents), Architecture Invariants (modular monolith, layering, AppError), APP_ENV Environment Scoping Rule, MyHouse Project Instructions (Agents), Locked Tech Stack (Rust/Axum/React/Postgres/moka/Docker), General Coding Guidelines (Claude rules), Docker Rules Skill (MyHouse), Dockerfile Security Rules (non-root, pinned versions, no secrets)

### Community 22 - "Database Rules (Claude, sqlx/PostgreSQL)"
Cohesion: 0.25
Nodes (8): Cascade and Filesystem Cleanup Ordering (Claude rules), Migration Conventions (Claude rules), Database Rules (Claude, sqlx/PostgreSQL), Listings/Search Index Performance Rules (Claude rules), sqlx Query Conventions (Claude rules), Schema Conventions (Claude rules), SQL Injection Prevention / Sensitive Column Exclusion (Claude rules), Repository Test Transaction Rollback Pattern (Claude rules)

### Community 23 - "/graphify command"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 25 - "/graphify add <url>"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 26 - "Multiple repos cross-repo graph merge"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 27 - "Code Review Backend Skill (Rust/Axum/MyHouse)"
Cohesion: 0.33
Nodes (6): Key Locked Decisions (OTP auth, roles, refresh tokens, upload security), Code Review Backend Skill (Rust/Axum/MyHouse), Four-Phase Backend Review Process, MyHouse Backend Invariants (auth, refresh tokens, storage keys), Backend Review Severity Labels (blocking/important/nit/suggestion), Backend Review Checklist Reference

### Community 28 - "--update (incremental re-extraction)"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 29 - "connect_db"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 30 - "Step 9 - Save manifest, update cost tracker, clean up, and report"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 31 - "Step 4 - Build graph, cluster, analyze, generate outputs"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 32 - "Extraction subagent prompt (compact)"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 33 - "MH-XXX Ticket Conventions"
Cohesion: 0.50
Nodes (4): MH-XXX Ticket Conventions, MH-XX Ticket Format Template, GitHub Ticket Generation Skill (MyHouse), Vertical Slicing Rule (BE/FE sub-tickets)

### Community 34 - "git commit hook (graphify hook install)"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 35 - "analyze job (rust + javascript-typescript matrix)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 43 - "ListingDetailDto"
Cohesion: 0.10
Nodes (43): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+35 more)

### Community 73 - "ListingFeed.tsx"
Cohesion: 0.13
Nodes (27): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+19 more)

### Community 74 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.17
Nodes (13): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), ADR-02: OTP Passwordless over Password/OAuth, ADR-09: Refresh token rotation on each use (+5 more)

### Community 75 - "DESIGN_TOKENS.md (MH-17) — Design Token Set"
Cohesion: 0.20
Nodes (10): GET/POST /listings endpoints, GET /search endpoint, DESIGN_TOKENS.md (MH-17) — Design Token Set, Border radius tokens (radius-sm/md/full), Color tokens (color-bg, color-primary, color-success, etc.), Spacing scale (Tailwind default 4px-based), Typography tokens (font-sans, font-mono, text-sm/base/lg), mh-13-feed-detail.html wireframe (+2 more)

### Community 81 - "mh-12-auth-flow.html wireframe"
Cohesion: 0.25
Nodes (8): ADR-08: Single OTP endpoint over separate login/register, OTP Login/Signup Runtime Flow (§6.1), POST /auth/otp/request endpoint, POST /auth/otp/verify endpoint, mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 82 - "StorageProvider trait"
Cohesion: 0.29
Nodes (7): ADR-05: StorageProvider trait + LocalFsStorage over S3-at-MVP, LocalFsStorage implementation, R-06: Filesystem storage not shared across instances, StorageProvider trait, POST /owner-requests endpoint, Backend Environment Variables (.env.example), StorageProvider trait interface (upload/read/delete/presigned_url)

### Community 83 - "mh-14-listing-management.html wireframe"
Cohesion: 0.33
Nodes (6): POST /media/upload endpoint, mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, Identity document dropzone component

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `React/TypeScript Rules (Agents)` → `README Writing Rules (MyHouse)`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: conceptually_related_to

## Knowledge Gaps
- **177 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+172 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **13 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `React/TypeScript Rules (Agents)` and `README Writing Rules (MyHouse)`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **Why does `AppError` connect `AppError` to `ListingDetailDto`, `.new`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState` to `.new`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `config/mod.rs` to `AppState`?**
  _High betweenness centrality (0.023) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _177 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.09024390243902439 - nodes in this community are weakly interconnected._