use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Game};
use crate::internal::{_take_token};
use crate::event::{FundsAdded};


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct AddFunds <'info>{
    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,


    /// Token
    pub token_program: Interface<'info, TokenInterface>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub game_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,

    
}

pub fn handler(ctx: Context<AddFunds>,
    amount: u64
) -> Result<()> {

    let game = &mut ctx.accounts.game;
    let signer = &mut ctx.accounts.signer;

    let mint                    = &mut ctx.accounts.mint;    
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let player_token_account    = &mut ctx.accounts.player_token_account;
    let token_program           = ctx.accounts.token_program.clone();


    _take_token(
        token_program,
        game_token_account,
        player_token_account,

        mint,
        game,
        signer, 

        amount
    )?;

    game.funds += amount;

    emit!(FundsAdded{
        amount: amount,
        new_funds: game.funds,
    });

    Ok(())
}
