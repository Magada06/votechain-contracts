use soroban_sdk::{contracterror, contracttype, Address};

/// All revert conditions for the token contract.
#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    /// 1 – Admin address is not set
    AdminNotSet = 1,
    /// 2 – Caller is not the admin
    NotAdmin = 2,
    /// 3 – Transfer/mint/burn amount must be positive
    InvalidAmount = 3,
    /// 4 – Sender has insufficient balance
    InsufficientBalance = 4,
    /// 5 – Spender allowance is insufficient
    AllowanceExceeded = 5,
    /// 6 – No pending admin transfer in progress
    NoPendingAdmin = 6,
    /// 7 – Caller is not the pending admin
    NotPendingAdmin = 7,
}

#[contracttype]
pub enum TokenDataKey {
    Balance(Address),
    Allowance(Address, Address),
    TotalSupply,
    Admin,
    PendingAdmin,
}
