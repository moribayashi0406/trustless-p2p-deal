use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tpd {
    use super::*;

    pub fn initializeDeal(ctx: Context<InitializeDeal>) -> Result<()> {
        Ok(())
    }

    pub fn registerTaker(ctx: Context<RegisterTaker>) -> Result<()> { 
        Ok(())
    }

    pub fn finalizeDeal(ctx: Context<FinalizeDeal>) -> Result<()> {
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct InitializeDeal {}
pub struct RegisterTaker{}
pub struct FinalizeDeal{}

#[account]
#[derive(Default)]

pub struct State {
    // Primary Key that allows us to derive other important accounts
    idx: u64,
    // deal offere
    user_offerer: Pubkey,
    // deal taker
    user_taker: Pubkey,
    // Mint amount of token
    mint_of_token_being_sent: Pubkey,
    // The escrower
    escrow_wallet: Pubkey,
    // Deposit ammount of tokens
    amount_tokens: u64,
    // State Machine Enum
    stage: u8
}