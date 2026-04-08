// Need to use player accounts 

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct Player {
    pub wallet: Pubkey,

    pub time: i64,
    pub pending: bool,

    pub result: u64,

    
    pub barrels:    u32,    //  
    pub sale_price: u64,    //
    pub cost:           u64,    // for refunds
    pub to_jackpot:     u64,    // for refunds
    pub payout:     u64,    //

    pub banked: u64,

    pub demand_multiplier: u8, // because it affects odds
    pub barrels_max: u32,       // because it affects time extension
    pub demand_duration: u64,   // because it affects time extension

}


