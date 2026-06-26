use soroban_sdk::{Env, Address, symbol_short};
use crate::types::{ProposalStatus, Vote};

pub fn proposal_created(env: &Env, id: u64, proposer: &Address) {
    env.events().publish((symbol_short!("created"), id), proposer.clone());
}

pub fn vote_cast(env: &Env, id: u64, voter: &Address, vote: &Vote, weight: i128) {
    env.events().publish((symbol_short!("vote"), id), (voter.clone(), vote.clone(), weight));
}

pub fn proposal_finalised(env: &Env, id: u64, status: &ProposalStatus) {
    env.events().publish((symbol_short!("final"), id), status.clone());
}

pub fn admin_transfer_started(env: &Env, current: &Address, pending: &Address) {
    env.events().publish((symbol_short!("adm_init"), current.clone()), pending.clone());
}

pub fn admin_transfer_completed(env: &Env, new_admin: &Address) {
    env.events().publish((symbol_short!("adm_done"),), new_admin.clone());
}
