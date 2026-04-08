use anchor_lang::prelude::*;

use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};


use crate::state::{Game, Player};
use crate::error::*;
use crate::internal::{_take_token, _get_demand};
use crate::instruction::{Resolve};

use crate::event::{ShipSent};
use crate::constants::{DEMAND_PRECISION, SALE_PRICE_PRECISION, SALE_PRICE_MIN, SALE_PRICE_MAX, PROGRAM_ID, JACKPOT, HUNDRED};

#[vrf]
#[derive(Accounts)]
pub struct SendShip <'info>{

    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Box< Account<'info, Game>>, 
    
    /// CHECK: Oracle Queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = [
            b"player".as_ref(),
            signer.key().as_ref()
        ],
        bump,
        payer = signer,
        space = Player::INIT_SPACE + 16
    )]
    pub player: Account<'info, Player>,


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

pub fn handler(ctx: Context<SendShip>, barrels: u32, sale_price: u64) -> Result<()> {

    let clock: Clock = Clock::get()?;


    let signer = &mut ctx.accounts.signer;
    let signer_key = signer.key();
    let game = &mut ctx.accounts.game;
    let game_key = game.key();

    let player = &mut ctx.accounts.player;
    let player_key = player.key();

    let mint                    = &mut ctx.accounts.mint;    
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let player_token_account    = &mut ctx.accounts.player_token_account;
    let token_program           = ctx.accounts.token_program.clone();

    let system_program_key = ctx.accounts.system_program.key();

    let use_vrf = game.vrf == ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY;

    if player.pending {
        return err!(Errors::PlayerPending);
    }

    player.wallet = signer.key();

    if barrels == 0 || barrels > game.barrels_max{
        return err!(Errors::BarrelCount);
    }
    if sale_price < SALE_PRICE_MIN || sale_price > SALE_PRICE_MAX {
        return err!(Errors::SalePrice);
    }

    let cost = (barrels as u64) * game.barrel_cost;

    let payout_128 = 
        cost as u128 * 
        _get_demand(
            clock.unix_timestamp as u64,
            game
        ) as u128 * 
        ( 
            sale_price as u128
        ) / (
            DEMAND_PRECISION as u128 * 
            SALE_PRICE_PRECISION as u128
        );
    let payout = payout_128 as u64;

    let to_jackpot = cost * JACKPOT / HUNDRED;
    let to_funds = cost - to_jackpot; 

    game.funds += to_funds;
    let funds_available = game.funds - game.funds_locked;
    if payout > funds_available {
        return err!(Errors::FundsAvailable);
    }

    game.jackpot += to_jackpot;


    game.funds_locked += payout;
    

    player.time = clock.unix_timestamp;
    
    player.barrels = barrels;
    player.sale_price = sale_price;
    player.cost = cost;
    player.to_jackpot = to_jackpot;
    player.payout = payout;

    player.pending = true;

    // affects oods
    player.demand_multiplier = game.demand_multiplier;
    player.barrels_max = game.barrels_max;
    player.demand_duration = game.demand_duration;


    let mut to_pay: u64 = 0;
    if cost > player.banked {
        to_pay = cost - player.banked;
        player.banked = 0;
    }else{
        player.banked -= cost;
    }



    emit!(ShipSent{
        player:     player.key(),
        
        barrels:    barrels,
        sale_price: sale_price,

        new_funds:          game.funds,
        new_funds_locked:   game.funds_locked,
        new_jackpot:        game.jackpot,

        new_player_banked:  player.banked,

    });


    _take_token(
        token_program,
        game_token_account,
        player_token_account,

        mint,
        game,
        signer, 

        to_pay
    )?;

    let client_seed: u8 = 0;
    if use_vrf {
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: signer_key,
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: PROGRAM_ID,
            callback_discriminator: Resolve::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],

            accounts_metas: Some(vec![
                SerializableAccountMeta { //Game Account
                    pubkey: game_key,
                    is_signer: false,
                    is_writable: true,
                },

                SerializableAccountMeta { //Player Account
                    pubkey: player_key,
                    is_signer: false,
                    is_writable: true,
                },

                SerializableAccountMeta { //System program
                    pubkey: system_program_key,
                    is_signer: false,
                    is_writable: false,
                },

            ]),
            ..Default::default()
        });
        ctx.accounts.invoke_signed_vrf(
            &ctx.accounts.signer.to_account_info(), 
            &ix)?;
    }



    Ok(())
}
