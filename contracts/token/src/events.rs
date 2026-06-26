use soroban_sdk::{symbol_short, Address, Env};

pub fn minted(env: &Env, to: &Address, amount: i128) {
    env.events().publish((symbol_short!("mint"), to.clone()), amount);
}

pub fn transferred(env: &Env, from: &Address, to: &Address, amount: i128) {
    env.events().publish((symbol_short!("transfer"), from.clone(), to.clone()), amount);
}

pub fn burned(env: &Env, from: &Address, amount: i128) {
    env.events().publish((symbol_short!("burn"), from.clone()), amount);
}

pub fn admin_transfer_started(env: &Env, current: &Address, pending: &Address) {
    env.events().publish((symbol_short!("adm_init"), current.clone()), pending.clone());
}

pub fn admin_transfer_completed(env: &Env, new_admin: &Address) {
    env.events().publish((symbol_short!("adm_done"),), new_admin.clone());
}
