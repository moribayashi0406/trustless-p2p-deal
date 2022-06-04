use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tpd {
    use super::*;
    pub fn initialize_deal(ctx: Context<InitializeDeal>, amount_tokens:u64,application_idx:u64) -> Result<()> {
        
        let state = &mut ctx.accounts.application_state;
    
        // amount and idx are just pass the data so there are no ctx stuff!
        state.amount_tokens = amount_tokens;
        state.idx = application_idx;
        
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
    idx: u64,
    // // deal offere
    user_offerer: Pubkey,
    // // deal taker
    user_taker: Pubkey,
    // // Mint amount of token
    mint_of_token_being_sent: Pubkey,
    // // The vault
    valult_wallet: Pubkey,
    // // Deposit ammount of tokens
    amount_tokens: u64,
    // // State Machine Enum
    stage: u8
}



#[derive(Accounts)]
#[instruction(application_idx: u64, state_bump: u8, wallet_bump: u8)]
pub struct InitializeDeal<'info> {
    #[account(
        init,
        payer=user_offerer,
        // space place holder(must change later!!!)
        space=1234,
        seeds=[b"state".as_ref(), user_offerer.key().as_ref(), user_taker.key.as_ref(), mint_of_token_being_sent.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump,
    )]
    application_state: Account<'info, State>,
    #[account(
        init,
        payer=user_offerer,
        // space place holder(must change later!!!)
        space=1234,
        seeds=[b"wallet".as_ref(), user_offerer.key().as_ref(), user_taker.key.as_ref(), mint_of_token_being_sent.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump,
    )]
    vault_wallet: Account<'info, TokenAccount>,
    
    #[account(mut)]
    user_offerer: Signer<'info>,
    user_taker: AccountInfo<'info>,
    mint_of_token_being_sent: Account<'info, Mint>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}
// pub struct RegisterTaker<'info>{}
// pub struct FinalizeDeal<'info>{}
// pub struct CancelDeal<'info>{}

