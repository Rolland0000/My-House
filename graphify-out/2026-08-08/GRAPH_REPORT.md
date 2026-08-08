# Graph Report - .  (2026-08-04)

## Corpus Check
- 129 files · ~64,221 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 611 nodes · 773 edges · 86 communities (74 shown, 12 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 70 edges (avg confidence: 0.88)
- Token cost: 350,415 input · 0 output

## Community Hubs (Navigation)
- App Bootstrap & Routing
- Backend Config Loading
- Frontend Core Dependencies
- CI/CD & Docker Pipeline
- Frontend Dev Tooling
- Shared UI Components
- Auth & Owner Wireframes
- Architecture Modules & ADRs
- DB Schema & API Endpoints
- Frontend TS Config (App)
- Backend Error Handling
- Frontend TS Config (Node)
- Backend Pagination
- MCP Server Config
- React/TS Rules Duplication
- Graphify Extraction Spec
- Frontend App Shell
- Graphify Export Formats
- Rust Rules Duplication
- Graphify Query/Path/Explain
- DB Rules (Agents)
- General Coding Guidelines
- DB Rules (Claude)
- Graphify Setup & Detection
- Graphify Add/Watch
- Graphify GitHub Merge & Update
- Backend Code Review Skill
- Graphify Update/Cluster-Only
- Backend DB Connection
- Graphify Manifest & Cost
- Graphify Build Pipeline Steps
- Codex Multi-Agent Extraction
- GitHub Ticket Conventions
- Graphify Commit Hook
- CodeQL & Gitleaks Workflows
- Claude Hooks
- Frontend TSConfig Root
- Moka Cache ADR
- Claude MD Graphify Trigger
- ADR: Modular Monolith
- ADR: Postgres Fulltext Search
- ADR: Seeker Default Role
- Architecture AppError Doc
- Health & Graceful Shutdown Doc
- Pagination Standard Doc
- Risk: No Price Index
- Refresh Token API Doc

## God Nodes (most connected - your core abstractions)
1. `AppState` - 18 edges
2. `compilerOptions` - 18 edges
3. `compilerOptions` - 16 edges
4. `ConfigError` - 13 edges
5. `ARCHITECTURE_v1.2.md — arc42 Software Architecture Document` - 13 edges
6. `scripts` - 12 edges
7. `require_parsed()` - 11 edges
8. `cn()` - 11 edges
9. `merged_router()` - 10 edges
10. `AppConfig` - 9 edges

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

## Communities (86 total, 12 thin omitted)

### Community 0 - "App Bootstrap & Routing"
Cohesion: 0.09
Nodes (28): Arc, ApiDoc, AppServer, Error, Result, Self, shutdown_signal(), AppState (+20 more)

### Community 1 - "Backend Config Loading"
Cohesion: 0.16
Nodes (26): app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), rejects_invalid_app_env() (+18 more)

### Community 2 - "Frontend Core Dependencies"
Cohesion: 0.06
Nodes (32): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+24 more)

### Community 3 - "CI/CD & Docker Pipeline"
Cohesion: 0.09
Nodes (33): docker-compose.yml (root), backend-dev service (dev profile, cargo-watch), backend-prod service (prod profile, release binary), frontend-dev service (dev profile, vite dev server), frontend-prod service (prod profile, nginx static), postgres service, ADR-06: utoipa + openapi-typescript over manual TS types, backend/Dockerfile (multi-stage build) (+25 more)

### Community 4 - "Frontend Dev Tooling"
Cohesion: 0.06
Nodes (31): eslint, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks (+23 more)

### Community 5 - "Shared UI Components"
Cohesion: 0.13
Nodes (22): Button(), ButtonProps, ButtonSize, ButtonVariant, sizeClasses, variantClasses, Card(), CardPadding (+14 more)

### Community 6 - "Auth & Owner Wireframes"
Cohesion: 0.07
Nodes (30): ADR-08: Single OTP endpoint over separate login/register, No-proxy principle for public file reads (§7.3), OTP Login/Signup Runtime Flow (§6.1), Owner Request and Validation Runtime Flow (§6.2), Owner-requests documents proxy exception (admin-only read), R-09: Single admin account, no operational redundancy, GET /admin/owner-requests/:id/documents/:doc_id endpoint, POST /auth/otp/request endpoint (+22 more)

### Community 7 - "Architecture Modules & ADRs"
Cohesion: 0.12
Nodes (27): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), ARCHITECTURE_v1.2.md — arc42 Software Architecture Document, ADR-02: OTP Passwordless over Password/OAuth (+19 more)

### Community 8 - "DB Schema & API Endpoints"
Cohesion: 0.11
Nodes (25): ADR-05: StorageProvider trait + LocalFsStorage over S3-at-MVP, LocalFsStorage implementation, R-06: Filesystem storage not shared across instances, R-08: fn_update_listing_search_vector trigger SELECT-per-row cost, StorageProvider trait, TECHNICAL_SPEC_MVP_v1.2.md — Technical Specification MVP, GET /listings/:id/contact endpoint, GET/POST /listings endpoints (+17 more)

### Community 9 - "Frontend TS Config (App)"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 10 - "Backend Error Handling"
Cohesion: 0.17
Nodes (16): AppError, ErrorBody, ErrorEnvelope, parse_envelope(), Error, Self, StatusCode, String (+8 more)

### Community 11 - "Frontend TS Config (Node)"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 12 - "Backend Pagination"
Cohesion: 0.18
Nodes (15): PaginatedResponse, PaginatedResponse<T>, PaginationMeta, Self, T, test_defaults_applied_when_none(), test_offset_calculation(), test_page_floors_at_one() (+7 more)

### Community 13 - "MCP Server Config"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 14 - "React/TS Rules Duplication"
Cohesion: 0.15
Nodes (15): Functional Component Conventions (no React.FC), api.ts Single Fetch Entry Point Rule, Locked Frontend Folder Structure (app/features/shared), React/TypeScript Rules (Agents), Functional Component Conventions (Claude rules), api.ts Single Fetch Entry Point Rule (Claude rules), Locked Frontend Folder Structure (Claude rules), React/TypeScript Rules (Claude rules) (+7 more)

### Community 15 - "Graphify Extraction Spec"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 16 - "Frontend App Shell"
Cohesion: 0.23
Nodes (6): App(), RootLayout(), Providers(), ProvidersProps, queryClient, router

### Community 17 - "Graphify Export Formats"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 18 - "Rust Rules Duplication"
Cohesion: 0.20
Nodes (10): Concurrency and Async Rules (no blocking locks across await), Error Handling (thiserror/anyhow), Rust General Rules (Agents), Ownership and Types Conventions, Testing and Quality Gates (cargo check/fmt/clippy), Concurrency and Async Rules (Claude rules), Error Handling (thiserror/anyhow) (Claude rules), MCP Usage Policy (GitHub/PostgreSQL/Git/Context7/Filesystem) (+2 more)

### Community 19 - "Graphify Query/Path/Explain"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 20 - "DB Rules (Agents)"
Cohesion: 0.29
Nodes (8): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern

### Community 21 - "General Coding Guidelines"
Cohesion: 0.25
Nodes (8): General Coding Guidelines (Agents), Architecture Invariants (modular monolith, layering, AppError), APP_ENV Environment Scoping Rule, MyHouse Project Instructions (Agents), Locked Tech Stack (Rust/Axum/React/Postgres/moka/Docker), General Coding Guidelines (Claude rules), Docker Rules Skill (MyHouse), Dockerfile Security Rules (non-root, pinned versions, no secrets)

### Community 22 - "DB Rules (Claude)"
Cohesion: 0.25
Nodes (8): Cascade and Filesystem Cleanup Ordering (Claude rules), Migration Conventions (Claude rules), Database Rules (Claude, sqlx/PostgreSQL), Listings/Search Index Performance Rules (Claude rules), sqlx Query Conventions (Claude rules), Schema Conventions (Claude rules), SQL Injection Prevention / Sensitive Column Exclusion (Claude rules), Repository Test Transaction Rollback Pattern (Claude rules)

### Community 23 - "Graphify Setup & Detection"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 25 - "Graphify Add/Watch"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 26 - "Graphify GitHub Merge & Update"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 27 - "Backend Code Review Skill"
Cohesion: 0.33
Nodes (6): Key Locked Decisions (OTP auth, roles, refresh tokens, upload security), Code Review Backend Skill (Rust/Axum/MyHouse), Four-Phase Backend Review Process, MyHouse Backend Invariants (auth, refresh tokens, storage keys), Backend Review Severity Labels (blocking/important/nit/suggestion), Backend Review Checklist Reference

### Community 28 - "Graphify Update/Cluster-Only"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 29 - "Backend DB Connection"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 30 - "Graphify Manifest & Cost"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 31 - "Graphify Build Pipeline Steps"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 32 - "Codex Multi-Agent Extraction"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 33 - "GitHub Ticket Conventions"
Cohesion: 0.50
Nodes (4): MH-XXX Ticket Conventions, MH-XX Ticket Format Template, GitHub Ticket Generation Skill (MyHouse), Vertical Slicing Rule (BE/FE sub-tickets)

### Community 34 - "Graphify Commit Hook"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 35 - "CodeQL & Gitleaks Workflows"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `React/TypeScript Rules (Agents)` → `README Writing Rules (MyHouse)`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: conceptually_related_to

## Knowledge Gaps
- **164 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+159 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `React/TypeScript Rules (Agents)` and `README Writing Rules (MyHouse)`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **Why does `docker-compose.yml (root)` connect `CI/CD & Docker Pipeline` to `DB Schema & API Endpoints`, `Architecture Modules & ADRs`?**
  _High betweenness centrality (0.014) - this node is a cross-community bridge._
- **Why does `ARCHITECTURE_v1.2.md — arc42 Software Architecture Document` connect `Architecture Modules & ADRs` to `DB Schema & API Endpoints`, `CI/CD & Docker Pipeline`?**
  _High betweenness centrality (0.012) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _164 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `App Bootstrap & Routing` be split into smaller, more focused modules?**
  _Cohesion score 0.09024390243902439 - nodes in this community are weakly interconnected._
- **Should `Frontend Core Dependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.06060606060606061 - nodes in this community are weakly interconnected._