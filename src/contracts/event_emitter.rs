//! Minimal Soroban contract that emits each canonical event family.
//!
//! See `docs/event-schema.md` for the topic and payload contract consumed by
//! the decoder layer.

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

#[contract]
pub struct EventEmitter;

#[derive(Clone)]
#[contracttype]
pub struct AssetAmount {
    pub asset: String,
    pub amount: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct SwapPayload {
    pub amount_in: i128,
    pub amount_out: i128,
    pub max_slippage_bps: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct ApprovalPayload {
    pub asset: String,
    pub amount: i128,
    pub expires_at_ledger: u32,
}

#[contractimpl]
impl EventEmitter {
    pub fn emit_transfer(env: Env, from: Address, to: Address, asset: String, amount: i128) {
        let payload = AssetAmount { asset, amount };
        env.events()
            .publish((symbol_short!("transfer"), from, to), payload);
    }

    pub fn emit_mint(env: Env, to: Address, asset: String, amount: i128) {
        let payload = AssetAmount { asset, amount };
        env.events().publish((symbol_short!("mint"), to), payload);
    }

    pub fn emit_swap(
        env: Env,
        trader: Address,
        asset_in: String,
        asset_out: String,
        amount_in: i128,
        amount_out: i128,
        max_slippage_bps: u32,
    ) {
        let payload = SwapPayload {
            amount_in,
            amount_out,
            max_slippage_bps,
        };
        env.events().publish(
            (symbol_short!("swap"), trader, asset_in, asset_out),
            payload,
        );
    }

    pub fn emit_approve(
        env: Env,
        owner: Address,
        spender: Address,
        asset: String,
        amount: i128,
        expires_at_ledger: u32,
    ) {
        let payload = ApprovalPayload {
            asset,
            amount,
            expires_at_ledger,
        };
        env.events()
            .publish((symbol_short!("approve"), owner, spender), payload);
    }
}

#[cfg(test)]
mod tests {
    use super::{ApprovalPayload, AssetAmount, EventEmitter, EventEmitterClient, SwapPayload};
    use soroban_sdk::{
        symbol_short,
        testutils::{Address as _, Events},
        Address, Env, String, Symbol, TryFromVal, Val, Vec as SorobanVec,
    };

    fn setup() -> (Env, EventEmitterClient<'static>) {
        let env = Env::default();
        let contract_id = env.register(EventEmitter, ());
        let client = EventEmitterClient::new(&env, &contract_id);
        (env, client)
    }

    fn only_event(env: &Env) -> (SorobanVec<Val>, Val) {
        let events = env.events().all();
        assert_eq!(events.len(), 1, "sample contract should publish one event");
        let (_, topics, data) = events.into_iter().next().unwrap();
        (topics, data)
    }

    #[test]
    fn emits_transfer_event() {
        let (env, client) = setup();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");

        client.emit_transfer(&from, &to, &usdc, &10_000_000);

        let (topics, data) = only_event(&env);
        assert_eq!(
            Symbol::try_from_val(&env, &topics.get_unchecked(0)).unwrap(),
            symbol_short!("transfer")
        );
        assert_eq!(
            Address::try_from_val(&env, &topics.get_unchecked(1)).unwrap(),
            from
        );
        assert_eq!(
            Address::try_from_val(&env, &topics.get_unchecked(2)).unwrap(),
            to
        );
        let payload = AssetAmount::try_from_val(&env, &data).unwrap();
        assert_eq!(payload.asset, usdc);
        assert_eq!(payload.amount, 10_000_000);
    }

    #[test]
    fn emits_mint_event() {
        let (env, client) = setup();
        let to = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");

        client.emit_mint(&to, &usdc, &50_000_000);

        let (topics, data) = only_event(&env);
        assert_eq!(
            Symbol::try_from_val(&env, &topics.get_unchecked(0)).unwrap(),
            symbol_short!("mint")
        );
        assert_eq!(
            Address::try_from_val(&env, &topics.get_unchecked(1)).unwrap(),
            to
        );
        let payload = AssetAmount::try_from_val(&env, &data).unwrap();
        assert_eq!(payload.asset, usdc);
        assert_eq!(payload.amount, 50_000_000);
    }

    #[test]
    fn emits_swap_event() {
        let (env, client) = setup();
        let trader = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");
        let xlm = String::from_str(&env, "XLM");

        client.emit_swap(&trader, &usdc, &xlm, &10_000_000, &9_960_000, &50);

        let (topics, data) = only_event(&env);
        assert_eq!(
            Symbol::try_from_val(&env, &topics.get_unchecked(0)).unwrap(),
            symbol_short!("swap")
        );
        assert_eq!(
            Address::try_from_val(&env, &topics.get_unchecked(1)).unwrap(),
            trader
        );
        assert_eq!(
            String::try_from_val(&env, &topics.get_unchecked(2)).unwrap(),
            usdc
        );
        assert_eq!(
            String::try_from_val(&env, &topics.get_unchecked(3)).unwrap(),
            xlm
        );
        let payload = SwapPayload::try_from_val(&env, &data).unwrap();
        assert_eq!(payload.amount_in, 10_000_000);
        assert_eq!(payload.amount_out, 9_960_000);
        assert_eq!(payload.max_slippage_bps, 50);
    }

    #[test]
    fn emits_approve_event() {
        let (env, client) = setup();
        let from = Address::generate(&env);
        let spender = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");

        client.emit_approve(&from, &spender, &usdc, &25_000_000, &1_234_567);

        let (topics, data) = only_event(&env);
        assert_eq!(
            Symbol::try_from_val(&env, &topics.get_unchecked(0)).unwrap(),
            symbol_short!("approve")
        );
        assert_eq!(
            Address::try_from_val(&env, &topics.get_unchecked(1)).unwrap(),
            from
        );
        assert_eq!(
            Address::try_from_val(&env, &topics.get_unchecked(2)).unwrap(),
            spender
        );
        let payload = ApprovalPayload::try_from_val(&env, &data).unwrap();
        assert_eq!(payload.asset, usdc);
        assert_eq!(payload.amount, 25_000_000);
        assert_eq!(payload.expires_at_ledger, 1_234_567);
    }
}
