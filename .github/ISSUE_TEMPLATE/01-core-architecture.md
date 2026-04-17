---
name: "[Stellar Wave] Core Project Architecture & Dependency Setup"
about: Initialize the Rust project structure and dependency baseline
title: "[Stellar Wave] Core Project Architecture & Dependency Setup"
labels: type:backend, type:platform, phase:foundation, priority:p0
assignees: ''
---

## Type
`backend` · `platform` · `devex`

## Phase
**Phase 1 — Foundation**

## Component
**Backend Core**

## Description
Initialize the Rust project structure and establish the runtime/dependency baseline for ingestion, storage, and API modules.

## Tasks
- [ ] Create source layout: `src/ingest`, `src/storage`, `src/api`, `src/types`, `src/config`
- [ ] Configure Cargo.toml dependencies: `tokio`, `sqlx`, `soroban-sdk`, `stellar-rpc-client`, `serde`, `tracing`, `anyhow`/`thiserror`
- [ ] Add app entrypoint and module wiring in `src/main.rs`
- [ ] Load and validate environment variables at startup (`DATABASE_URL`, `SOROBAN_RPC_URL`, `CONTRACT_IDS`)
- [ ] Confirm clean `cargo check` and `cargo clippy`

## Acceptance Criteria
- [ ] Project compiles with zero errors on `cargo check`
- [ ] Module scaffolding exists and is importable
- [ ] Missing or malformed env vars produce a clear startup error
- [ ] README includes local setup and compile instructions

## Goal
Prove the project is technically ready to compile and wire together.

## Depends On
None — this is the root issue.
