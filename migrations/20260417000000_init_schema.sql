-- Migration: Create initial schema for Soroban event indexing
-- Description: Initialize tables for ledgers, contract events, and transaction metadata

-- Ledgers table: tracks synchronized ledger sequence numbers
CREATE TABLE IF NOT EXISTS ledgers (
    id BIGSERIAL PRIMARY KEY,
    sequence BIGINT NOT NULL UNIQUE,
    hash VARCHAR(255) NOT NULL UNIQUE,
    timestamp TIMESTAMPTZ NOT NULL,
    tx_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ledgers_sequence ON ledgers(sequence DESC);
CREATE INDEX IF NOT EXISTS idx_ledgers_timestamp ON ledgers(timestamp DESC);

-- Contract events table: stores decoded Soroban contract events
CREATE TABLE IF NOT EXISTS contract_events (
    id BIGSERIAL PRIMARY KEY,
    ledger_seq BIGINT NOT NULL,
    tx_hash VARCHAR(255) NOT NULL,
    contract_id VARCHAR(255) NOT NULL,
    event_type VARCHAR(255),
    topics TEXT,
    data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (ledger_seq) REFERENCES ledgers(sequence) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contract_events_contract_id ON contract_events(contract_id);
CREATE INDEX IF NOT EXISTS idx_contract_events_ledger_seq ON contract_events(ledger_seq DESC);
CREATE INDEX IF NOT EXISTS idx_contract_events_created_at ON contract_events(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_contract_events_event_type ON contract_events(event_type);

-- Transaction metadata table: stores supplementary transaction information
CREATE TABLE IF NOT EXISTS transaction_meta (
    id BIGSERIAL PRIMARY KEY,
    tx_hash VARCHAR(255) NOT NULL UNIQUE,
    ledger_seq BIGINT NOT NULL,
    fee BIGINT,
    status VARCHAR(50),
    raw_meta TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (ledger_seq) REFERENCES ledgers(sequence) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_transaction_meta_tx_hash ON transaction_meta(tx_hash);
CREATE INDEX IF NOT EXISTS idx_transaction_meta_ledger_seq ON transaction_meta(ledger_seq);
CREATE INDEX IF NOT EXISTS idx_transaction_meta_status ON transaction_meta(status);
