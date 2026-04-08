use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {    
    #[msg("Token:Set")]
    TokenSet,
    #[msg("Token:Account")]
    TokenAccount,
    #[msg("Token:Program")]
    TokenProgram,
    #[msg("Token:Amount")]
    TokenAmount,

    #[msg("Funds:Available")]
    FundsAvailable,


    #[msg("Player:Pending")]
    PlayerPending,

    #[msg("Player:Time")]
    PlayerTime,

    
    #[msg("Barrel:Count")]
    BarrelCount,
    #[msg("Barrel:Cost")]
    BarrelCost,
    #[msg("Barrel:Max")]
    BarrelMax,
    #[msg("Sale:Price")]
    SalePrice,

    #[msg("Demand:Multiplier")]
    DemandMultiplier,
    #[msg("Demand:Duration")]
    DemandDuration,
}

