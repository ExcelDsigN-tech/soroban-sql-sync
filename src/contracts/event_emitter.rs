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
    use super::{EventEmitter, EventEmitterClient};
    use soroban_sdk::{
        testutils::{Address as _, Events},
        Address, Env, String,
    };

    fn setup() -> (Env, EventEmitterClient<'static>) {
        let env = Env::default();
        let contract_id = env.register(EventEmitter, ());
        let client = EventEmitterClient::new(&env, &contract_id);
        (env, client)
    }

    #[test]
    fn emits_transfer_event() {
        let (env, client) = setup();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");

        client.emit_transfer(&from, &to, &usdc, &10_000_000);

        assert!(
            !env.events().all().is_empty(),
            "sample contract should publish the transfer event"
        );
    }

    #[test]
    fn emits_mint_event() {
        let (env, client) = setup();
        let to = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");

        client.emit_mint(&to, &usdc, &50_000_000);

        assert!(
            !env.events().all().is_empty(),
            "sample contract should publish the mint event"
        );
    }

    #[test]
    fn emits_swap_event() {
        let (env, client) = setup();
        let trader = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");
        let xlm = String::from_str(&env, "XLM");

        client.emit_swap(&trader, &usdc, &xlm, &10_000_000, &9_960_000, &50);

        assert!(
            !env.events().all().is_empty(),
            "sample contract should publish the swap event"
        );
    }

    #[test]
    fn emits_approve_event() {
        let (env, client) = setup();
        let from = Address::generate(&env);
        let spender = Address::generate(&env);
        let usdc = String::from_str(&env, "USDC");

        client.emit_approve(&from, &spender, &usdc, &25_000_000, &1_234_567);

        assert!(
            !env.events().all().is_empty(),
            "sample contract should publish the approve event"
        );
    }
}
