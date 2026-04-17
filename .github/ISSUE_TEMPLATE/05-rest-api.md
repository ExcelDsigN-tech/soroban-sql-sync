---
name: "[Stellar Wave] REST API for Indexed Event Retrieval"
about: Build Axum REST API exposing health and event query endpoints
title: "[Stellar Wave] REST API for Indexed Event Retrieval"
labels: type:backend, type:api, phase:visibility, priority:p1
assignees: ''
---

## Type
`backend` · `api`

## Phase
**Phase 3 — Visibility**

## Component
**API**

## Description
Build a lightweight REST API in `src/api` using `axum` that exposes indexed Soroban event data and system health for frontend and external consumers.

## Tasks
- [ ] Add `axum` and `tower` dependencies
- [ ] Implement `GET /health` — returns DB status, ingestion status, latest ledger synced
- [ ] Implement `GET /events/:contract_id` — returns paginated events with optional filters (`from_ledger`, `to_ledger`, `event_type`)
- [ ] Define consistent JSON error response model (`{ error: string, code: string }`)
- [ ] Add per-request tracing/logging with correlation IDs
- [ ] Write integration tests for both routes
- [ ] Document endpoint contract (request/response shape) in README

## Acceptance Criteria
- [ ] API serves health data and event results from PostgreSQL
- [ ] Pagination works correctly for datasets with 1000+ events
- [ ] All error responses use consistent schema
- [ ] Route contract documented in README or inline OpenAPI comments

## Goal
Make indexed data accessible to the dashboard and external consumers.

## Depends On
- #2 — PostgreSQL Schema Design & Migrations
- #4 — XDR Decoding & Data Transformation Layer
