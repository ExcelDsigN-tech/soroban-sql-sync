#!/usr/bin/env bash
# =============================================================================
# create-issues.sh
# Creates all Stellar Wave project issues on GitHub via the `gh` CLI.
#
# Prerequisites:
#   - GitHub CLI installed: https://cli.github.com/
#   - Authenticated: gh auth login
#   - Run from within the repo root, or set REPO below.
#
# Usage:
#   chmod +x create-issues.sh
#   ./create-issues.sh
# =============================================================================

set -euo pipefail

REPO="ExcelDsigN-tech/soroban-sql-sync"  # Change if your remote differs

# ---------------------------------------------------------------------------
# Helper: create a label if it doesn't already exist
# ---------------------------------------------------------------------------
create_label() {
  local name="$1" color="$2" desc="$3"
  gh label create "$name" --color "$color" --description "$desc" --repo "$REPO" 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# 0. Ensure all labels exist
# ---------------------------------------------------------------------------
echo ">> Creating labels..."

create_label "stellar-wave"               "7B2FBE" "Stellar Wave project"
create_label "type:backend"               "0075ca" "Backend / Rust service"
create_label "type:frontend"              "e4e669" "Frontend / React UI"
create_label "type:database"              "c2e0c6" "Database schema or queries"
create_label "type:migration"             "bfd4f2" "SQL migration scripts"
create_label "type:api"                   "d93f0b" "REST API endpoints"
create_label "type:blockchain-integration" "f9d0c4" "Soroban / Stellar RPC integration"
create_label "type:data-processing"       "fef2c0" "XDR decode / transform logic"
create_label "type:ingestion"             "0e8a16" "Event ingestion pipeline"
create_label "type:ui"                    "e99695" "UI components and layout"
create_label "type:dashboard"             "fbca04" "Dashboard views"
create_label "type:smart-contract"        "5319e7" "Soroban smart contract code"
create_label "type:blockchain"            "1d76db" "On-chain logic"
create_label "type:spec"                  "cccccc" "Specification / documentation"
create_label "type:platform"              "006b75" "Dev tooling / project setup"
create_label "type:devex"                 "c5def5" "Developer experience"
create_label "phase:foundation"           "0052cc" "Phase 1 — Foundation"
create_label "phase:ingestion"            "e11d48" "Phase 2 — Ingestion"
create_label "phase:visibility"           "16a34a" "Phase 3 — Visibility"
create_label "priority:p0"               "b60205" "Critical / blocking"
create_label "priority:p1"               "e4e669" "High priority"
create_label "priority:p2"               "0e8a16" "Normal priority"

echo ">> Labels ready."
echo ""

# ---------------------------------------------------------------------------
# 1. Phase 1 — Foundation
# ---------------------------------------------------------------------------
echo ">> Creating Phase 1 issues..."

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] Core Project Architecture & Dependency Setup" \
  --label "stellar-wave,type:backend,type:platform,type:devex,phase:foundation,priority:p0" \
  --body "## Type
\`backend\` · \`platform\` · \`devex\`

## Phase
**Phase 1 — Foundation**

## Component
**Backend Core**

## Description
Initialize the Rust project structure and establish the runtime/dependency baseline for ingestion, storage, and API modules.

## Tasks
- [ ] Create source layout: \`src/ingest\`, \`src/storage\`, \`src/api\`, \`src/types\`, \`src/config\`
- [ ] Configure Cargo.toml dependencies: \`tokio\`, \`sqlx\`, \`soroban-sdk\`, \`stellar-rpc-client\`, \`serde\`, \`tracing\`, \`anyhow\`/\`thiserror\`
- [ ] Add app entrypoint and module wiring in \`src/main.rs\`
- [ ] Load and validate environment variables at startup (\`DATABASE_URL\`, \`SOROBAN_RPC_URL\`, \`CONTRACT_IDS\`)
- [ ] Confirm clean \`cargo check\` and \`cargo clippy\`

## Acceptance Criteria
- [ ] Project compiles with zero errors on \`cargo check\`
- [ ] Module scaffolding exists and is importable
- [ ] Missing or malformed env vars produce a clear startup error
- [ ] README includes local setup and compile instructions

## Goal
Prove the project is technically ready to compile and wire together.

## Depends On
None — this is the root issue."

echo ">> Issue 1 created."

# ---------------------------------------------------------------------------

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] PostgreSQL Schema Design & Migrations" \
  --label "stellar-wave,type:backend,type:database,type:migration,phase:foundation,priority:p0" \
  --body "## Type
\`backend\` · \`database\` · \`migration\`

## Phase
**Phase 1 — Foundation**

## Component
**Data Layer**

## Description
Define relational schema and migration flow for all Soroban event indexing data.

## Tasks
- [ ] Choose migration strategy (sqlx migrations preferred)
- [ ] Create \`ledgers\` table (sequence, hash, timestamp, tx_count)
- [ ] Create \`contract_events\` table (id, contract_id, ledger_seq, tx_hash, event_type, topics, data, created_at)
- [ ] Create \`transaction_meta\` table (tx_hash, ledger_seq, fee, status, raw_meta)
- [ ] Add indexes on \`contract_id\`, \`ledger_seq\`, \`created_at\`
- [ ] Write rollback (down) migration scripts
- [ ] Document migration run steps in README

## Acceptance Criteria
- [ ] Fresh database migrates up/down cleanly
- [ ] Schema supports querying by contract_id, ledger range, and timestamp
- [ ] All indexes validated against expected API query patterns
- [ ] Migration steps documented in README

## Goal
Show a clear and executable persistence plan before ingestion is wired.

## Depends On
- #1 — Core Project Architecture & Dependency Setup"

echo ">> Issue 2 created."

# ---------------------------------------------------------------------------
# 2. Phase 2 — Ingestion
# ---------------------------------------------------------------------------
echo ""
echo ">> Creating Phase 2 issues..."

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] Soroban RPC Event Listener Implementation" \
  --label "stellar-wave,type:backend,type:blockchain-integration,type:ingestion,phase:ingestion,priority:p0" \
  --body "## Type
\`backend\` · \`blockchain-integration\` · \`ingestion\`

## Phase
**Phase 2 — Ingestion**

## Component
**Blockchain Connector**

## Description
Implement an async listener task using \`tokio\` and \`stellar-rpc-client\` that continuously polls the Soroban RPC for new events emitted by configured Contract IDs.

## Tasks
- [ ] Build tokio polling worker in \`src/ingest\` with configurable interval and backoff
- [ ] Filter events by configured contract IDs (\`CONTRACT_IDS\` env var)
- [ ] Track ledger cursor/checkpoint in database to support resume on restart
- [ ] Log raw XDR payloads and event metadata via structured tracing
- [ ] Implement retry with exponential backoff for transient RPC failures
- [ ] Add graceful shutdown signal handling

## Acceptance Criteria
- [ ] Listener connects to \`SOROBAN_RPC_URL\` and receives events
- [ ] New events from watched contracts are continuously detected and logged
- [ ] After restart, ingestion resumes from last recorded ledger
- [ ] RPC failures are caught, logged, and retried without crashing

## Goal
Demonstrate live connectivity and event ingestion from the Stellar network.

## Depends On
- #1 — Core Project Architecture & Dependency Setup
- #2 — PostgreSQL Schema Design & Migrations"

echo ">> Issue 3 created."

# ---------------------------------------------------------------------------

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] XDR Decoding & Data Transformation Layer" \
  --label "stellar-wave,type:backend,type:data-processing,type:blockchain-integration,phase:ingestion,priority:p1" \
  --body "## Type
\`backend\` · \`data-processing\` · \`blockchain-integration\`

## Phase
**Phase 2 — Ingestion**

## Component
**Decode & Transform**

## Description
Build a service in \`src/ingest\` that takes raw XDR event data and decodes it into strongly-typed, JSON-friendly Rust structs ready for storage and API use.

## Tasks
- [ ] Implement XDR decode module using \`soroban-sdk\` types
- [ ] Map Soroban types to Rust: \`Symbol\`, \`Address\`, \`i128\`/\`u128\`, \`Bytes\`, \`Vec<Val>\`
- [ ] Build normalized output DTOs with \`serde::Serialize\`
- [ ] Categorize decode errors without crashing the worker loop
- [ ] Write unit tests with real/sample XDR fixture payloads
- [ ] Plug decoder into the listener pipeline (raw XDR → decoded struct → DB write)

## Acceptance Criteria
- [ ] Known XDR fixtures decode to expected Rust structs
- [ ] Output serializes to valid JSON without manual intervention
- [ ] Type coverage: Symbol, Address, all integer families
- [ ] Decode errors are non-fatal and observable in logs

## Goal
Convert raw blockchain payloads into human-readable, storable data structures.

## Depends On
- #3 — Soroban RPC Event Listener Implementation"

echo ">> Issue 4 created."

# ---------------------------------------------------------------------------
# 3. Phase 3 — Visibility
# ---------------------------------------------------------------------------
echo ""
echo ">> Creating Phase 3 issues..."

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] REST API for Indexed Event Retrieval" \
  --label "stellar-wave,type:backend,type:api,phase:visibility,priority:p1" \
  --body "## Type
\`backend\` · \`api\`

## Phase
**Phase 3 — Visibility**

## Component
**API**

## Description
Build a lightweight REST API in \`src/api\` using \`axum\` that exposes indexed Soroban event data and system health for frontend and external consumers.

## Tasks
- [ ] Add \`axum\` and \`tower\` dependencies
- [ ] Implement \`GET /health\` — DB status, ingestion status, latest ledger synced
- [ ] Implement \`GET /events/:contract_id\` — paginated events with optional filters (\`from_ledger\`, \`to_ledger\`, \`event_type\`)
- [ ] Define consistent JSON error response model
- [ ] Add per-request tracing with correlation IDs
- [ ] Write integration tests for both routes
- [ ] Document endpoint contract in README

## Acceptance Criteria
- [ ] API serves health data and event results from PostgreSQL
- [ ] Pagination works correctly for 1000+ event datasets
- [ ] All error responses use consistent schema
- [ ] Endpoint contract documented in README

## Goal
Make indexed data accessible to the dashboard and external consumers.

## Depends On
- #2 — PostgreSQL Schema Design & Migrations
- #4 — XDR Decoding & Data Transformation Layer"

echo ">> Issue 5 created."

# ---------------------------------------------------------------------------

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] Live Dashboard UI - Phase 1" \
  --label "stellar-wave,type:frontend,type:ui,type:dashboard,phase:visibility,priority:p1" \
  --body "## Type
\`frontend\` · \`ui\` · \`dashboard\`

## Phase
**Phase 3 — Visibility**

## Component
**Frontend Dashboard**

## Description
Build the "Control Room" dashboard using React and Tailwind CSS. Connect the UI to the backend API to display live sync status and a feed of recent Soroban contract events.

## Tasks
- [ ] Scaffold frontend app (Vite + React + Tailwind CSS)
- [ ] Build dashboard layout: header, status bar, events table
- [ ] Implement health widget connected to \`GET /health\`
- [ ] Implement events table connected to \`GET /events/:contract_id\` with contract ID selector
- [ ] Add polling refresh every 5–10s (configurable)
- [ ] Handle loading, empty, and error UI states
- [ ] Ensure responsive layout (mobile + desktop)
- [ ] Add README with frontend setup and run instructions

## Acceptance Criteria
- [ ] Dashboard renders live health status and event feed
- [ ] UI refreshes automatically without page reload
- [ ] Loading, empty, and error states are visually distinct
- [ ] Layout is usable on mobile and desktop
- [ ] Frontend README covers \`npm install\` + \`npm run dev\`

## Goal
Provide visual proof of indexing progress that reviewers and users can interact with.

## Depends On
- #5 — REST API for Indexed Event Retrieval"

echo ">> Issue 6 created."

# ---------------------------------------------------------------------------
# Optional smart contract track
# ---------------------------------------------------------------------------
echo ""
echo ">> Creating optional smart contract issue..."

gh issue create \
  --repo "$REPO" \
  --title "[Stellar Wave] Contract Event Schema & Emission Spec" \
  --label "stellar-wave,type:smart-contract,type:blockchain,type:spec,phase:foundation,priority:p1" \
  --body "## Type
\`smart-contract\` · \`blockchain\` · \`spec\`

## Phase
**Phase 1 — Foundation (parallel track)**

## Component
**Smart Contract**

## Description
Define the canonical event topic/payload schema for Soroban smart contracts in this project. Align the emitted event shape with what the XDR decoder (#4) expects.

## Tasks
- [ ] Enumerate all event types the contracts will emit (e.g., \`transfer\`, \`mint\`, \`swap\`, \`approve\`)
- [ ] Define topic + data structure for each type using \`env.events().publish()\`
- [ ] Document schema in \`docs/event-schema.md\`
- [ ] Implement a minimal sample Soroban contract that emits one event of each type
- [ ] Export XDR fixture payloads from sample contract for decoder unit tests (#4)
- [ ] Validate schema alignment with the decoder layer before #4 is closed

## Acceptance Criteria
- [ ] Every event type has a documented topic + payload structure
- [ ] Sample contract compiles and emits expected events in test environment
- [ ] XDR fixtures are committed and referenced in decoder tests
- [ ] Schema doc is cross-linked in both contract and indexer READMEs

## Goal
Establish a contract between the smart contract layer and the indexer — preventing schema drift between emitter and decoder.

## Depends On
- #1 — Core Project Architecture & Dependency Setup
- Ideally drafted before or alongside #4"

echo ">> Issue 7 created."
echo ""
echo "================================================================"
echo " All 7 issues created successfully on $REPO"
echo "================================================================"
