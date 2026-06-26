#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Events}, Address, Env, FromVal};

fn setup() -> (Env, TokenContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(TokenContract, ());
    (env.clone(), TokenContractClient::new(&env, &id))
}

#[test]
fn test_initialize() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    c.initialize(&admin, &1_000_000);
    assert_eq!(c.total_supply(), 1_000_000);
    assert_eq!(c.balance(&admin), 1_000_000);
}

#[test]
fn test_transfer() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.transfer(&admin, &user, &400);
    assert_eq!(c.balance(&admin), 600);
    assert_eq!(c.balance(&user), 400);
}

#[test]
fn test_mint_burn() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.mint(&admin, &user, &500);
    assert_eq!(c.total_supply(), 1_500);
    c.burn(&admin, &user, &200);
    assert_eq!(c.total_supply(), 1_300);
}

#[test]
#[should_panic]
fn test_overdraft() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &100);
    c.transfer(&admin, &user, &999);
}

#[test]
#[should_panic]
fn test_mint_non_admin() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.mint(&user, &user, &500);
}

#[test]
fn test_balance_of() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    c.initialize(&admin, &1_000);
    assert_eq!(c.balance(&admin), 1_000);
    assert_eq!(c.balance(&user1), 0);
    c.transfer(&admin, &user1, &300);
    assert_eq!(c.balance(&admin), 700);
    assert_eq!(c.balance(&user1), 300);
    assert_eq!(c.balance(&user2), 0);
}

#[test]
fn test_transfer_atomicity() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    let before_total = c.total_supply();
    c.transfer(&admin, &user, &400);
    assert_eq!(c.balance(&admin) + c.balance(&user), before_total);
    assert_eq!(c.total_supply(), before_total);
}

#[test]
fn test_mint_increases_supply() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    assert_eq!(c.total_supply(), 1_000);
    c.mint(&admin, &user, &500);
    assert_eq!(c.balance(&user), 500);
    assert_eq!(c.total_supply(), 1_500);
}

#[test]
fn test_events_mint() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &0);
    c.mint(&admin, &user, &300);
    let events = env.events().all();
    let found = events.iter().any(|(_, topics, data)| {
        if topics.len() != 2 { return false; }
        let sym  = soroban_sdk::Symbol::from_val(&env, &topics.get(0).unwrap());
        let addr = Address::from_val(&env, &topics.get(1).unwrap());
        sym == symbol_short!("mint") && addr == user && i128::from_val(&env, &data) == 300_i128
    });
    assert!(found);
}

#[test]
fn test_events_transfer() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.transfer(&admin, &user, &200);
    let events = env.events().all();
    let found = events.iter().any(|(_, topics, data)| {
        if topics.len() != 3 { return false; }
        let sym  = soroban_sdk::Symbol::from_val(&env, &topics.get(0).unwrap());
        let from = Address::from_val(&env, &topics.get(1).unwrap());
        let to   = Address::from_val(&env, &topics.get(2).unwrap());
        sym == symbol_short!("transfer") && from == admin && to == user && i128::from_val(&env, &data) == 200_i128
    });
    assert!(found);
}

#[test]
fn test_events_burn() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.burn(&admin, &admin, &400);
    let events = env.events().all();
    let found = events.iter().any(|(_, topics, data)| {
        if topics.len() != 2 { return false; }
        let sym  = soroban_sdk::Symbol::from_val(&env, &topics.get(0).unwrap());
        let addr = Address::from_val(&env, &topics.get(1).unwrap());
        sym == symbol_short!("burn") && addr == admin && i128::from_val(&env, &data) == 400_i128
    });
    assert!(found);
}

// ── Issue-475: token admin role separation tests ──────────────────────────────

#[test]
fn test_token_transfer_admin_two_step() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.transfer_admin(&admin, &new_admin);
    c.accept_admin(&new_admin);
    // new_admin can mint; old admin cannot
    c.mint(&new_admin, &user, &500);
    assert_eq!(c.total_supply(), 1_500);
}

#[test]
#[should_panic]
fn test_token_transfer_admin_non_admin_reverts() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let attacker = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.transfer_admin(&attacker, &attacker);
}

#[test]
#[should_panic]
fn test_token_accept_admin_wrong_caller_reverts() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let attacker = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.transfer_admin(&admin, &new_admin);
    c.accept_admin(&attacker);
}

#[test]
#[should_panic]
fn test_token_mint_old_admin_after_transfer_reverts() {
    let (env, c) = setup();
    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let user = Address::generate(&env);
    c.initialize(&admin, &1_000);
    c.transfer_admin(&admin, &new_admin);
    c.accept_admin(&new_admin);
    c.mint(&admin, &user, &100); // old admin should be rejected
}
