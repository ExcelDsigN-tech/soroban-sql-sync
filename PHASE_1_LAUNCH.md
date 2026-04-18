# Phase 1 Launch — How to Ship & Close Issues

This scaffold is ready to ship in **two fast PRs** that will close issues #6 and #1, demonstrating technical readiness to Drips reviewers.

---

## PR #1: Core Project Architecture & Dependency Setup → Closes #6

### What's Included
- ✅ Module structure: `src/main.rs`, `src/ingest/`, `src/storage/`, `src/api/`, `src/config.rs`, `src/types.rs`
- ✅ Updated `Cargo.toml` with all Phase 1 dependencies
- ✅ Config loading from env vars with validation
- ✅ Tracing/logging infrastructure

### How to Ship
```bash
# 1. Set up local environment
cp .env.example .env
# Edit .env and fill in DATABASE_URL, SOROBAN_RPC_URL, CONTRACT_IDS

# 2. Test compilation
cargo check
cargo clippy

# 3. Take a screenshot of successful output (proof for reviewers)

# 4. Commit and push
git add src/ Cargo.toml Cargo.lock .env.example
git commit -m "Phase 1 PR #1: Core Architecture & Dependency Setup

- Initialize module structure (ingest, storage, api, config, types)
- Add dependencies: tokio, sqlx, soroban-sdk, serde, tracing, axum, tower, anyhow, thiserror
- Implement config loading with env var validation
- Add comprehensive error handling

Closes #6"

git push origin main

# 5. Create PR on GitHub with title and Closes #6 in description
# Include screenshot of cargo check output in PR description
```

### Acceptance Criteria (to verify before closing #6)
- [ ] `cargo check` passes with zero errors
- [ ] All modules are importable (compile verified)
- [ ] `README.md` updated with local setup instructions
- [ ] `.env.example` provided for developers

---

## PR #2: PostgreSQL Schema Design & Migrations → Closes #1

### What's Included
- ✅ Migration file: `migrations/20260417000000_init_schema.sql`
- ✅ Tables: `ledgers`, `contract_events`, `transaction_meta` with indexes
- ✅ Foreign keys and constraints for data integrity

### How to Ship
```bash
# 1. Install sqlx-cli if not already installed
cargo install sqlx-cli --no-default-features --features postgres

# 2. Ensure PostgreSQL is running locally
# macOS: brew services start postgresql
# Linux: systemctl start postgresql
# Windows: Use PostgreSQL installer or Docker

# 3. Create test database
createdb soroban_sql_sync

# 4. Update .env to point to the test DB
# Example: DATABASE_URL=postgres://localhost/soroban_sql_sync

# 5. Run migrations and take screenshot
sqlx migrate run
# Screenshot of: ✓ migration applied successfully

# 6. Verify schema
psql soroban_sql_sync
soroban_sql_sync=# \dt
soroban_sql_sync=# \di
# Screenshot showing all 3 tables and indexes

# 7. Test rollback (optional but impressive)
sqlx migrate revert
# Screenshot of: ✓ migration reverted

# 8. Re-apply migration
sqlx migrate run

# 9. Commit and push
git add migrations/ sqlx-data.json
git commit -m "Phase 1 PR #2: PostgreSQL Schema Design & Migrations

- Create ledgers table with sequence tracking and indexes
- Create contract_events table with contract_id/ledger_seq/timestamp indexes
- Create transaction_meta table with tx_hash and status indexes
- Add foreign key constraints for referential integrity
- Migrations up/down tested and verified

Closes #1"

git push origin main
```

### Acceptance Criteria (to verify before closing #1)
- [ ] Fresh database migrates cleanly with `sqlx migrate run`
- [ ] All 3 tables created with correct columns
- [ ] Indexes exist on `contract_id`, `ledger_seq`, `created_at`, `timestamp`
- [ ] Migration rollback works: `sqlx migrate revert`
- [ ] Schema queries by contract_id, ledger range, timestamp are optimized
- [ ] `README.md` includes migration run instructions

---

## Reapplication to Drips

After both PRs are merged:

1. **Create a Summary Comment** on both closed issues showing:
   - Screenshot of PR merge commit
   - Screenshot of Phase 1 acceptance criteria met
   - Link to main branch with both features visible

2. **Prepare Reapplication with**:
   - Link to repo with 7-issue backlog visible
   - Screenshots of both Phase 1 PRs merged
   - Quote: "Phase 1 Foundation complete — ready to proceed to ingestion & visibility phases"
   - Roadmap link showing progression

3. **Key Talking Points**:
   - ✅ Project structure proven (compiles, imports cleanly)
   - ✅ Database schema designed for Soroban events (migrations tested)
   - ✅ Phased execution plan with 7 labeled, linked issues
   - ✅ Ready to ingest live Stellar events in Phase 2

---

## Local Testing Before Shipping

```bash
# Full local test cycle
cargo clean
cargo check      # Should pass
cargo clippy      # Should pass
cargo build       # Full build

# Database setup
createdb soroban_sql_sync
export DATABASE_URL=postgres://localhost/soroban_sql_sync
sqlx migrate run   # Should succeed
```

If all pass, you're ready to ship both PRs within 1 hour.
