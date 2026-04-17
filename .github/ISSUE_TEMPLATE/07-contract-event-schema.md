---
name: "[Stellar Wave] Contract Event Schema & Emission Spec"
about: Define and document canonical event schema for smart contract emissions
title: "[Stellar Wave] Contract Event Schema & Emission Spec"
labels: type:smart-contract, type:blockchain, type:spec, phase:foundation, priority:p1
assignees: ''
---

## Type
`smart-contract` · `blockchain` · `spec`

## Phase
**Phase 1 — Foundation (parallel track)**

## Component
**Smart Contract**

## Description
Define the canonical event topic/payload schema for Soroban smart contracts in this project. Align the emitted event shape with what the XDR decoder (Issue #4) expects, so the contract and indexer evolve together.

## Tasks
- [ ] Enumerate all event types the contracts will emit (e.g., `transfer`, `mint`, `swap`, `approve`)
- [ ] Define topic and data structure for each event type using Soroban `env.events().publish()`
- [ ] Document schema in a shared spec file (`docs/event-schema.md` or inline in contract source)
- [ ] Implement a minimal sample Soroban contract that emits at least one event of each type
- [ ] Export fixture XDR payloads from the sample contract for use in decoder unit tests (#4)
- [ ] Validate schema alignment with the decoder layer before #4 is closed

## Acceptance Criteria
- [ ] Every event type has a documented topic + payload structure
- [ ] Sample contract compiles and emits expected events in test environment
- [ ] XDR fixtures are committed and referenced in decoder tests
- [ ] Schema doc is cross-linked in both the contract and indexer READMEs

## Goal
Establish a contract between the smart contract layer and the indexer — preventing schema drift between emitter and decoder.

## Depends On
- #1 — Core Project Architecture & Dependency Setup
- Ideally drafted before or alongside #4
