use anchor_lang::prelude::*;

use crate::state::{Game};
use crate::error::*;
use crate::event::{SettingsUpdated};

#[derive(Accounts)]
#[instruction()]
pub struct UpdateSettings <'info>{
    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>,

    #[account(mut, address = game.creator)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<UpdateSettings>, 
    barrel_cost:        u64,
    barrels_max:        u32,
    demand_multiplier:  u8,
    demand_duration:    u64,
) -> Result<()> {
    let game     = &mut ctx.accounts.game;

    if barrel_cost == 0 {
        return err!(Errors::BarrelCost);
    }
    if barrels_max == 0 {
        return err!(Errors::BarrelMax);
    }

    if demand_multiplier == 0 {
        return err!(Errors::DemandMultiplier);
    }
    if demand_duration == 0 {
        return err!(Errors::DemandDuration);
    }


    game.barrel_cost        = barrel_cost;
    game.barrels_max        = barrels_max;
    game.demand_multiplier  = demand_multiplier;
    game.demand_duration    = demand_duration;

    emit!(SettingsUpdated{
        barrel_cost:        barrel_cost,
        barrels_max:        barrels_max,
        demand_multiplier:  demand_multiplier,
        demand_duration:    demand_duration,
    });

    Ok(())
}