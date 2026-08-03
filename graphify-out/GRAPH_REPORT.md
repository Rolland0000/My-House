# Graph Report - /Users/hermann/Documents/M@Vie/My-/My-House  (2026-08-02)

## Corpus Check
- 129 files · ~63,088 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 341 nodes · 451 edges · 55 communities (53 shown, 2 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 4 edges (avg confidence: 0.73)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Community 0
- Community 1
- Community 2
- Community 3
- Community 4
- Community 5
- Community 6
- Community 7
- Community 8
- Community 9
- Community 10
- Community 11
- Community 12
- Community 14
- Community 15
- Community 16

## God Nodes (most connected - your core abstractions)
1. `AppState` - 18 edges
2. `compilerOptions` - 18 edges
3. `compilerOptions` - 16 edges
4. `ConfigError` - 13 edges
5. `scripts` - 12 edges
6. `require_parsed()` - 11 edges
7. `cn()` - 11 edges
8. `merged_router()` - 10 edges
9. `AppConfig` - 9 edges
10. `AppEnv` - 8 edges

## Surprising Connections (you probably didn't know these)
- `AppServer` --references--> `AppState`  [EXTRACTED]
  backend/src/app_server.rs → backend/src/app_state.rs
- `Inner` --references--> `AppConfig`  [EXTRACTED]
  backend/src/app_state.rs → backend/src/config/mod.rs
- `check()` --references--> `AppState`  [EXTRACTED]
  backend/src/infra/health.rs → backend/src/app_state.rs
- `admin_router()` --references--> `AppState`  [EXTRACTED]
  backend/src/route.rs → backend/src/app_state.rs
- `build_router()` --references--> `AppState`  [EXTRACTED]
  backend/src/route.rs → backend/src/app_state.rs

## Import Cycles
- None detected.

## Communities (55 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.16
Nodes (26): app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, loads_valid_config(), optional_or(), optional_u16_or(), rejects_invalid_app_env() (+18 more)

### Community 1 - "Community 1"
Cohesion: 0.06
Nodes (31): eslint, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks (+23 more)

### Community 2 - "Community 2"
Cohesion: 0.13
Nodes (22): Button(), ButtonProps, ButtonSize, ButtonVariant, sizeClasses, variantClasses, Card(), CardPadding (+14 more)

### Community 3 - "Community 3"
Cohesion: 0.14
Nodes (21): Arc, ApiDoc, AppState, Inner, PgPool, Self, check(), HealthStatus (+13 more)

### Community 4 - "Community 4"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 5 - "Community 5"
Cohesion: 0.17
Nodes (16): AppError, ErrorBody, ErrorEnvelope, parse_envelope(), Error, Self, StatusCode, String (+8 more)

### Community 6 - "Community 6"
Cohesion: 0.10
Nodes (19): name, typescript, overrides, openapi-typescript, private, scripts, build, dev (+11 more)

### Community 7 - "Community 7"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 8 - "Community 8"
Cohesion: 0.18
Nodes (15): PaginatedResponse, PaginatedResponse<T>, PaginationMeta, Self, T, test_defaults_applied_when_none(), test_offset_calculation(), test_page_floors_at_one() (+7 more)

### Community 9 - "Community 9"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 10 - "Community 10"
Cohesion: 0.15
Nodes (13): clsx, dependencies, clsx, lucide-react, react, react-dom, react-router, @tanstack/react-query (+5 more)

### Community 11 - "Community 11"
Cohesion: 0.21
Nodes (7): AppServer, Error, Result, Self, shutdown_signal(), Box, SocketAddr

### Community 12 - "Community 12"
Cohesion: 0.23
Nodes (6): App(), RootLayout(), Providers(), ProvidersProps, queryClient, router

### Community 14 - "Community 14"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

## Knowledge Gaps
- **94 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+89 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Community 3` to `Community 11`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `Community 0` to `Community 3`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _94 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.06451612903225806 - nodes in this community are weakly interconnected._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.13118279569892474 - nodes in this community are weakly interconnected._
- **Should `Community 3` be split into smaller, more focused modules?**
  _Cohesion score 0.13793103448275862 - nodes in this community are weakly interconnected._
- **Should `Community 4` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._