# Stellar Wave — Roadmap

A high-performance Soroban event indexer bridging Stellar to PostgreSQL.

## Current Status: 🚀 In Active Development

| Phase | Status | Issues | Target |
|-------|--------|--------|--------|
| **Phase 1: Foundation** | In Progress | [#1–#7](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues) | Proof of technical readiness |
| **Phase 2: Ingestion** | Planned | — | Live blockchain event ingestion |
| **Phase 3: Visibility** | Planned | — | REST API + React dashboard |

---

## Phase 1: Foundation (Current)

### Goal
Prove the project is technically ready to ingest and persist Soroban events.

### Deliverables
1. **[#6] Core Project Architecture & Dependency Setup**
   - Rust module scaffold (`ingest`, `storage`, `api`, `types`, `config`)
   - Dependencies: tokio, sqlx, soroban-sdk, stellar-rpc-client, serde, tracing
   - `cargo check` passing

2. **[#1] PostgreSQL Schema Design & Migrations**
   - Tables: `ledgers`, `contract_events`, `transaction_meta`
   - Indexes for query performance
   - Up/down migrations working

3. **[#5] Contract Event Schema & Emission Spec** (optional, parallel)
   - Define canonical event schema for contracts
   - Sample Soroban contract with event emissions

---

## Phase 2: Ingestion (Planned)

### Goal
Demonstrate live connectivity to Stellar RPC and event processing pipeline.

### Deliverables
- **[#2] Soroban RPC Event Listener** — async worker polling for events
- **[#3] XDR Decoding Layer** — transform raw XDR into normalized structs
- Events persisted to PostgreSQL

---

## Phase 3: Visibility (Planned)

### Goal
Make indexed data accessible and visible to reviewers and users.

### Deliverables
- **[#7] REST API** — `/health`, `/events/:contract_id` endpoints
- **[#4] React Dashboard** — live sync status, events table, Tailwind UI

---

## Development Instructions

### Local Setup
```bash
# Clone and install dependencies
cargo build

# Run migrations
sqlx migrate run

# Start local development
cargo run
```

### Contribution Guidelines
- Link PRs to issues: `Closes #N`
- Include proof screenshots (compile output, test results, migrations)
- All acceptance criteria must be met before closing an issue

### See Also
- [GitHub Issues](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues)
- [GitHub Projects](https://github.com/ExcelDsigN-tech/soroban-sql-sync/projects)
