---
name: "[Stellar Wave] XDR Decoding & Data Transformation Layer"
about: Decode raw XDR event payloads into normalized Rust/JSON structures
title: "[Stellar Wave] XDR Decoding & Data Transformation Layer"
labels: type:backend, type:data-processing, type:blockchain-integration, phase:ingestion, priority:p1
assignees: ''
---

## Type
`backend` · `data-processing` · `blockchain-integration`

## Phase
**Phase 2 — Ingestion**

## Component
**Decode & Transform**

## Description
Build a service in `src/ingest` that takes raw XDR event data from the listener and decodes it into strongly-typed, JSON-friendly Rust structs ready for storage and API use.

## Tasks
- [ ] Implement XDR decode module using `soroban-sdk` types
- [ ] Map Soroban types to Rust equivalents: `Symbol`, `Address`, `i128`/`u128`, `Bytes`, `Vec<Val>`
- [ ] Build normalized output DTOs (Data Transfer Objects) with `serde::Serialize`
- [ ] Categorize decode errors (unsupported type, malformed payload) without crashing the worker
- [ ] Write unit tests with real/sample XDR fixture payloads
- [ ] Plug decoder into the listener pipeline (raw XDR → decoded struct → DB write)

## Acceptance Criteria
- [ ] Known XDR fixtures decode to expected Rust structs consistently
- [ ] Output serializes to valid JSON without manual intervention
- [ ] Type coverage includes Symbol, Address, and all integer families
- [ ] Decode errors are non-fatal to the worker loop and observable in logs

## Goal
Convert raw blockchain payloads into human-readable, storable data structures.

## Depends On
- #3 — Soroban RPC Event Listener Implementation
