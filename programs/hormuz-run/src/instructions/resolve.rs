use anchor_lang::prelude::*;

use crate::state::{Game, Player};
// use crate::internal::{_give_sol_account};
use crate::error::*;
use crate::event::{Resolved};
use crate::constants::{HUNDRED, CORRUPTION, SALE_PRICE_MIN};


#[derive(Accounts)]
pub struct Resolve <'info>{

    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program trough CPI
    #[account(address = game.vrf)]
    pub vrf_program_identity: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>, // This will also be the signer


    #[account(mut)]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<Resolve>,
    randomness: [u8; 32],
) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let timestamp = clock.unix_timestamp as u64;

    let game          = &mut ctx.accounts.game;
    let player        = &mut ctx.accounts.player;

    // Make sure they don't call when shits not goin on. 
    //  Safeguard to prevent depending on how their system works
    if !player.pending {
        return err!(Errors::PlayerPending);
    }
    

    let rnd_u64 = ephemeral_vrf_sdk::rnd::random_u64(&randomness); 

    player.result = rnd_u64;
    let denominator = player.sale_price;
    let numerator = SALE_PRICE_MIN * ( HUNDRED - CORRUPTION) / HUNDRED / (player.demand_multiplier as u64);

    let win = rnd_u64 % denominator < numerator;
    player.pending = false;

    let payout = player.payout + game.jackpot;

    if win {
        player.banked += payout;
        game.funds -= player.payout;
        game.jackpot = 0;

        let seconds = player.barrels as u64 * player.demand_duration / player.barrels_max as u64; 
        game.demand_start_time += seconds;
        if game.demand_start_time > timestamp {
            game.demand_start_time = timestamp;
        }
        game.demand_end_time = game.demand_start_time + game.demand_duration;

        if game.demand_end_time < timestamp {
            game.demand_start_time = timestamp - game.demand_duration/2;
            game.demand_end_time = game.demand_start_time + game.demand_duration;
        }

    }else{

    }
    game.funds_locked -= player.payout;
    
    emit!(Resolved{
        player:             player.wallet,
        result: rnd_u64,

        barrels:            player.barrels,

        won: win,

        new_funds:          game.funds,
        new_funds_locked:   game.funds_locked,
        
        payout: payout,
        new_player_banked:  player.banked,

        new_start_time:     game.demand_start_time,
        new_end_time:       game.demand_end_time,
    });

    Ok(())
}
