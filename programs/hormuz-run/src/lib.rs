pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod event;
mod internal;


use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HhbTBeyzaGyWYWjb8bAJqhjxVQbYUBSCU9xYLZMWjAis");

#[program]
pub mod hormuz_run {
    use super::*;

    pub fn add_funds(ctx: Context<AddFunds>, amount: u64) -> Result<()> {
        add_funds::handler(ctx, amount)
    }

    pub fn initialise(ctx: Context<Initialise>, vrf: Pubkey) -> Result<()> {
        initialise::handler(ctx, vrf)
    }

    pub fn release(ctx: Context<Release>, player_key: Pubkey) -> Result<()> {
        release::handler(ctx, player_key)
    }
    pub fn resolve(ctx: Context<Resolve>, randomness: [u8; 32]) -> Result<()> {
        resolve::handler(ctx, randomness)
    }


    pub fn send_ship(ctx: Context<SendShip>, 
        barrels: u32, 
        sale_price: u64
    ) -> Result<()> {
        send_ship::handler(ctx, barrels, sale_price)
    }

    pub fn set_token(ctx: Context<SetToken>, token_account: Pubkey, token_program: Pubkey) -> Result<()> {
        set_token::handler(ctx, token_account, token_program)
    }


    pub fn update_creator(ctx: Context<UpdateCreator>, new_creator: Pubkey) -> Result<()> {
        update_creator::handler(ctx, new_creator)
    }

    pub fn update_settings(ctx: Context<UpdateSettings>, 
        barrel_cost:        u64,
        barrels_max:        u32,
        demand_multiplier:  u8,
        demand_duration:    u64
    ) -> Result<()> {
        update_settings::handler(ctx, 
            barrel_cost,
            barrels_max,
            demand_multiplier,
            demand_duration
        )
    }


    pub fn withdraw_creator(ctx: Context<WithdrawCreator>, amount: u64) -> Result<()> {
        withdraw_creator::handler(ctx, amount)
    }


    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::handler(ctx)
    }


    





}
