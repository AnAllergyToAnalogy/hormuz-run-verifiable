use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct Game {
    pub vrf:        Pubkey,
    pub creator:    Pubkey,

    pub funds:              u64,
    pub funds_locked:       u64,

    pub jackpot:            u64,

    pub barrel_cost:    u64,    // set by settings
    pub barrels_max:    u32,    // set by settings, max per ship

    pub demand_multiplier:  u8,    // set by settings, max multiplier (should be 3?)
    pub demand_duration:    u64,    // set by settings

    pub demand_start_time:  u64,    //
    pub demand_end_time:    u64,    //

    // Token Settings
    pub token_account:  Pubkey,
    pub token_program:  Pubkey,
    pub token_set:      bool,

}