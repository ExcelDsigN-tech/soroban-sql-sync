# Soroban Event Schema

This document defines the canonical event contract between sample Soroban
contracts and the `soroban-sql-sync` decoder layer. Contracts should emit one
of the event families below and keep topic order stable so indexed rows can be
normalized into the same `DecodedEvent.event_type` values.

## Topic Rules

- Topic 0 is always the event family: `transfer`, `mint`, `swap`, or `approve`.
- Asset identifiers use canonical Stellar asset codes when available.
- Account identifiers are contract addresses serialized by the Soroban runtime.
- Amounts are signed 128-bit integer token units.
- Slippage and prices are represented in basis points or integer token units;
  contracts must not emit floating point values.

## Events

### `transfer`

Emitted when value moves from one account to another.

| Topic Position | Value | Description |
|---|---|---|
| 0 | `transfer` | Event family |
| 1 | `from` | Sender address |
| 2 | `to` | Recipient address |

Payload:

```json
{
  "asset": "USDC",
  "amount": "10000000"
}
```

Decoder output:

```json
{
  "event_type": "transfer",
  "data": {
    "asset": "USDC",
    "amount": "10000000"
  }
}
```

### `mint`

Emitted when new units are issued.

| Topic Position | Value | Description |
|---|---|---|
| 0 | `mint` | Event family |
| 1 | `to` | Recipient address |

Payload:

```json
{
  "asset": "USDC",
  "amount": "50000000"
}
```

Decoder output:

```json
{
  "event_type": "mint",
  "data": {
    "asset": "USDC",
    "amount": "50000000"
  }
}
```

### `swap`

Emitted when one asset is exchanged for another.

| Topic Position | Value | Description |
|---|---|---|
| 0 | `swap` | Event family |
| 1 | `trader` | Address executing the swap |
| 2 | `asset_in` | Input asset code |
| 3 | `asset_out` | Output asset code |

Payload:

```json
{
  "amount_in": "10000000",
  "amount_out": "9960000",
  "max_slippage_bps": 50
}
```

Decoder output:

```json
{
  "event_type": "swap",
  "data": {
    "amount_in": "10000000",
    "amount_out": "9960000",
    "max_slippage_bps": 50
  }
}
```

### `approve`

Emitted when an owner approves a spender.

| Topic Position | Value | Description |
|---|---|---|
| 0 | `approve` | Event family |
| 1 | `owner` | Owner address |
| 2 | `spender` | Spender address |

Payload:

```json
{
  "asset": "USDC",
  "amount": "25000000",
  "expires_at_ledger": 1234567
}
```

Decoder output:

```json
{
  "event_type": "approve",
  "data": {
    "asset": "USDC",
    "amount": "25000000",
    "expires_at_ledger": 1234567
  }
}
```

## Fixture Contract

The sample emitter in `src/contracts/event_emitter.rs` emits each canonical
event family with the topic and payload layout above. Fixture metadata lives in
`fixtures/event-schema/sample-events.json` and points to Soroban test snapshots
under `test_snapshots/contracts/event_emitter/tests/`. The decoder tests read
the fixture metadata so schema changes are visible to the indexing layer.
