use anchor_lang::prelude::*;

use crate::state::{Game, Player};
use crate::error::*;
use crate::event::{Released};
use crate::constants::{RELEASE_TIME};


#[derive(Accounts)]
#[instruction(player_key: Pubkey)]
pub struct Release <'info>{
    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>,


    // City owner account
    #[account(
        mut,
        seeds = [
            b"player".as_ref(),
            player_key.as_ref()
        ],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<Release>,
    player_key: Pubkey,
) -> Result<()> {
    let clock: Clock = Clock::get()?;

    let game            = &mut ctx.accounts.game;
    let player          = &mut ctx.accounts.player;

    if clock.unix_timestamp < player.time +  RELEASE_TIME {
        return err!(Errors::PlayerTime);
    } 

    if !player.pending {
        return err!(Errors::PlayerPending);
    }

    let funds_refund = player.cost - player.to_jackpot;
    let mut jackpot_refund = player.to_jackpot;


    game.funds_locked -= player.payout;
    game.funds -= funds_refund;

    player.pending = false;

    if game.jackpot < jackpot_refund {
        jackpot_refund = game.jackpot;
        game.jackpot = 0;
    } else {
        game.jackpot -= jackpot_refund;
    }

    player.banked += funds_refund + jackpot_refund;


    emit!(Released{
        player: player_key,

        new_funds:          game.funds,
        new_funds_locked:   game.funds_locked,
        new_jackpot:        game.jackpot,

        new_player_banked:  player.banked
    });

    Ok(())
}
