---
name: "[Stellar Wave] Soroban RPC Event Listener Implementation"
about: Implement async tokio task to poll Soroban RPC for contract events
title: "[Stellar Wave] Soroban RPC Event Listener Implementation"
labels: type:backend, type:blockchain-integration, type:ingestion, phase:ingestion, priority:p0
assignees: ''
---

## Type
`backend` · `blockchain-integration` · `ingestion`

## Phase
**Phase 2 — Ingestion**

## Component
**Blockchain Connector**

## Description
Implement an async listener task using `tokio` and `stellar-rpc-client` that continuously polls the Soroban RPC for new events emitted by configured Contract IDs.

## Tasks
- [ ] Build tokio worker in `src/ingest` that polls RPC on interval with backoff
- [ ] Filter events by configured contract IDs (`CONTRACT_IDS` env var)
- [ ] Track ledger cursor/checkpoint in database to support resume on restart
- [ ] Log raw XDR payloads and event metadata to structured tracing output
- [ ] Implement retry with exponential backoff for transient RPC failures
- [ ] Add graceful shutdown signal handling (SIGTERM/CTRL+C)

## Acceptance Criteria
- [ ] Listener connects to configured `SOROBAN_RPC_URL` and receives events
- [ ] New events from watched contracts are continuously detected and logged
- [ ] After restart, ingestion resumes from last recorded ledger (no full replay)
- [ ] RPC failures are caught, logged, and retried without crashing the process

## Goal
Demonstrate live connectivity and event ingestion from the Stellar network.

## Depends On
- #1 — Core Project Architecture & Dependency Setup
- #2 — PostgreSQL Schema Design & Migrations
