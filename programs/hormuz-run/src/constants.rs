use anchor_lang::prelude::*;

pub const DEMAND_PRECISION:     u64 = 1_000_000;

pub const JACKPOT:              u64 =   1_00;
pub const CORRUPTION:           u64 =  10_00;
pub const HUNDRED:              u64 = 100_00;

pub const SALE_PRICE_MIN:       u64 =   1_00;
pub const SALE_PRICE_MAX:       u64 = 100_00;
pub const SALE_PRICE_PRECISION: u64 =   1_00;

pub const DEFAULT_BARREL_COST:          u64 = 20_000_000;
pub const DEFAULT_BARRELS_MAX:          u32 = 500_000;
pub const DEFAULT_DEMAND_MULTIPLIER:    u8 = 3;
pub const DEFAULT_DEMAND_DURATION:      u64 = 24 * 60 * 60;


pub const RELEASE_TIME:         i64 =   30;

pub const PROGRAM_ID:   Pubkey = pubkey!("HhbTBeyzaGyWYWjb8bAJqhjxVQbYUBSCU9xYLZMWjAis");
