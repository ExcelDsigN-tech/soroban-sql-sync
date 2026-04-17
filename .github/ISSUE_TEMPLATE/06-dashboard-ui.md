---
name: "[Stellar Wave] Live Dashboard UI - Phase 1"
about: Build React + Tailwind Control Room dashboard connected to the API
title: "[Stellar Wave] Live Dashboard UI - Phase 1"
labels: type:frontend, type:ui, type:dashboard, phase:visibility, priority:p1
assignees: ''
---

## Type
`frontend` · `ui` · `dashboard`

## Phase
**Phase 3 — Visibility**

## Component
**Frontend Dashboard**

## Description
Build the "Control Room" dashboard using React and Tailwind CSS. Connect the UI to the backend API to display live sync status and a feed of recent Soroban contract events.

## Tasks
- [ ] Scaffold frontend app (Vite + React + Tailwind CSS)
- [ ] Build dashboard layout: header, status bar, events table
- [ ] Implement health widget connected to `GET /health` (DB status, latest ledger, sync lag)
- [ ] Implement events table connected to `GET /events/:contract_id` with contract ID selector
- [ ] Add polling refresh mechanism (every 5–10s, configurable)
- [ ] Handle loading, empty, and error UI states
- [ ] Ensure responsive layout (mobile + desktop)
- [ ] Add README with frontend setup and run instructions

## Acceptance Criteria
- [ ] Dashboard renders live health status and event feed from backend
- [ ] UI refreshes automatically without page reload
- [ ] Loading, empty, and error states are visually distinct
- [ ] Layout is usable on both mobile and desktop viewports
- [ ] Frontend README covers `npm install` + `npm run dev` setup

## Goal
Provide clear visual proof of indexing progress that reviewers and users can interact with.

## Depends On
- #5 — REST API for Indexed Event Retrieval
