use anchor_lang::prelude::*;

use crate::state::{Game};
use crate::constants::{DEFAULT_BARREL_COST, DEFAULT_BARRELS_MAX, DEFAULT_DEMAND_MULTIPLIER, DEFAULT_DEMAND_DURATION};

#[derive(Accounts)]
#[instruction(vrf: Pubkey)]
pub struct Initialise <'info>{
    #[account(
        init,
        seeds = [
            b"game".as_ref()
        ],
        bump,
        payer = signer,
        space = Game::INIT_SPACE + 16
    )]
    pub game: Account<'info, Game>,


    #[account(mut)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<Initialise>, vrf: Pubkey ) -> Result<()> {
    let clock: Clock = Clock::get()?;

    let game     = &mut ctx.accounts.game;
    let signer   = &mut ctx.accounts.signer;

    // Init Game
    if vrf != game.key(){
        game.vrf = vrf;
    }else{
        game.vrf = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY;
    }
    game.creator = signer.key();

    game.barrel_cost        = DEFAULT_BARREL_COST;
    game.barrels_max        = DEFAULT_BARRELS_MAX;
    game.demand_multiplier  = DEFAULT_DEMAND_MULTIPLIER;
    game.demand_duration    = DEFAULT_DEMAND_DURATION;

    let timestamp = clock.unix_timestamp as u64;
    game.demand_start_time = timestamp;
    game.demand_end_time = timestamp + game.demand_duration;
    
    
    Ok(())
}
