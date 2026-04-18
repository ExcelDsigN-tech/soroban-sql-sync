# Database Migrations

All schema migrations are managed with [sqlx-cli](https://github.com/launchbadge/sqlx).

## Setup

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features rustls,postgres

# Copy env template and fill in your credentials
cp .env.example .env

# Create the database
createdb soroban_sql_sync

# Apply all migrations
sqlx migrate run
```

## Schema (Phase 1)

### `ledgers`
Tracks each finalized Stellar ledger header.

| Column | Type | Notes |
|--------|------|-------|
| `sequence` | `BIGINT` | Primary key |
| `hash` | `TEXT` | Ledger hash (unique) |
| `close_time` | `TIMESTAMPTZ` | When the ledger closed |
| `created_at` | `TIMESTAMPTZ` | Row insert time |

### `contract_events`
One row per decoded Soroban contract event.

| Column | Type | Notes |
|--------|------|-------|
| `id` | `UUID` | Primary key |
| `ledger_sequence` | `BIGINT` | FK → `ledgers.sequence` |
| `contract_id` | `TEXT` | Soroban contract address |
| `topic` | `TEXT` | Event topic (XDR decoded) |
| `data` | `JSONB` | Full event payload |
| `transaction_hash` | `TEXT` | Parent transaction |
| `created_at` | `TIMESTAMPTZ` | Row insert time |

Indexes: `contract_id`, `ledger_sequence`

### `transaction_meta`
Metadata for transactions containing contract events.

| Column | Type | Notes |
|--------|------|-------|
| `id` | `UUID` | Primary key |
| `transaction_hash` | `TEXT` | Unique |
| `ledger_sequence` | `BIGINT` | FK → `ledgers.sequence` |
| `status` | `TEXT` | `success` / `failed` |
| `fee_charged` | `BIGINT` | Stroops |
| `created_at` | `TIMESTAMPTZ` | Row insert time |

## Adding a New Migration

```bash
sqlx migrate add <descriptive_name>
# Edit the generated file in migrations/
sqlx migrate run
```

## CI

The GitHub Actions `migrate` job spins up a Postgres 17 service container and runs `sqlx migrate run` on every push and PR — ensuring migrations are always verified against a clean database.
