---
name: "[Stellar Wave] PostgreSQL Schema Design & Migrations"
about: Design tables and migration flow for Soroban event persistence
title: "[Stellar Wave] PostgreSQL Schema Design & Migrations"
labels: type:backend, type:database, type:migration, phase:foundation, priority:p0
assignees: ''
---

## Type
`backend` · `database` · `migration`

## Phase
**Phase 1 — Foundation**

## Component
**Data Layer**

## Description
Define relational schema and migration flow for all Soroban event indexing data.

## Tasks
- [ ] Choose migration strategy (sqlx migrations preferred)
- [ ] Create `ledgers` table (sequence, hash, timestamp, tx_count)
- [ ] Create `contract_events` table (id, contract_id, ledger_seq, tx_hash, event_type, topics, data, created_at)
- [ ] Create `transaction_meta` table (tx_hash, ledger_seq, fee, status, raw_meta)
- [ ] Add indexes on `contract_id`, `ledger_seq`, `created_at` for query patterns
- [ ] Write rollback (down) migration scripts
- [ ] Document migration run steps in README

## Acceptance Criteria
- [ ] Fresh database migrates up/down cleanly with `sqlx migrate run` / `sqlx migrate revert`
- [ ] Schema supports querying by `contract_id`, ledger range, and timestamp
- [ ] All indexes validated against expected API query patterns
- [ ] Migration steps documented in README

## Goal
Show a clear and executable persistence plan before ingestion is wired.

## Depends On
- #1 — Core Project Architecture & Dependency Setup
