use anchor_lang::prelude::*;

// Withdraw
#[event]
pub struct Withdrawn {
    pub player: Pubkey,
    pub amount: u64,}

// WithdrawCreator
#[event]
pub struct WithdrawnCreator {
    pub amount:    u64,
    pub new_funds: u64,
}

// AddFunds
#[event]
pub struct FundsAdded {
    pub amount:    u64,
    pub new_funds: u64,
}

// Release
#[event]
pub struct Released {
    pub player:             Pubkey,
    
    pub new_funds:          u64,
    pub new_funds_locked:   u64,
    pub new_jackpot:        u64,

    pub new_player_banked:  u64,
}

// Resolve
#[event]
pub struct Resolved {
    pub player:             Pubkey,
    pub result:             u64,
    pub barrels:            u32,

    pub won:                bool,


    pub new_funds:          u64,
    pub new_funds_locked:   u64,

    pub payout:             u64,
    pub new_player_banked:  u64,

    pub new_start_time:  u64,
    pub new_end_time:  u64,
}


// SendShip
#[event]
pub struct ShipSent {
    pub player:             Pubkey,

    pub barrels:    u32,
    pub sale_price: u64,
    
    pub new_funds:          u64,
    pub new_funds_locked:   u64,
    pub new_jackpot:        u64,

    pub new_player_banked:  u64,
}


// UpdateSettings
#[event]
pub struct SettingsUpdated {
    pub barrel_cost:        u64,
    pub barrels_max:        u32,
    pub demand_multiplier:  u8,
    pub demand_duration:    u64,    
}