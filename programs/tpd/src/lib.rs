use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tpd {
    use super::*;
    pub fn initialize_deal(ctx: Context<InitializeDeal>, amount_tokens:u64) -> Result<()> {
        
        let state = &mut ctx.accounts.application_state;
        state.amount_tokens = amount_tokens;
        
        Ok(())
    }

    // pub fn registerTaker(ctx: Context<RegisterTaker>) -> Result<()> { 
    //     Ok(())
    // }

    // pub fn finalizeDeal(ctx: Context<FinalizeDeal>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn cancelDeal(ctx: Context<CancelDeal>) -> Result<()> {
    //     Ok(())
    // }
    
}

#[account]
#[derive(Default)]
pub struct State {
    // Primary Key that allows us to derive other important accounts
    // idx: u64,
    // // deal offere
    user_offerer: Pubkey,
    // // deal taker
    // user_taker: Pubkey,
    // // Mint amount of token
    // mint_of_token_being_sent: Pubkey,
    // // The escrower
    // escrow_wallet: Pubkey,
    // // Deposit ammount of tokens
    amount_tokens: u64
    // // State Machine Enum
    // stage: u8
}



#[derive(Accounts)]
pub struct InitializeDeal<'info> {
         
    #[account(
        init,
        payer=user_offerer,
        space=1234
    )]
    application_state: Account<'info, State>,

    #[account(mut)]
    user_offerer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}
// pub struct RegisterTaker<'info>{}
// pub struct FinalizeDeal<'info>{}
// pub struct CancelDeal<'info>{}

