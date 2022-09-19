use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct EscrowAccount {
    initializer_key: Pubkey,
    initializer_deposit_token_account: Pubkey,
    initializer_receive_token_account: Pubkey,
    initializer_amount: u64,
    taker_amount: u64,
}

pub fn initialize_handler(
    mut ctx: Context<Initialize>,
    mut initializer_amount: u64,
    mut taker_amount: u64,
) -> Result<()> {
    let mut initializer = &mut ctx.accounts.initializer;
    let mut mint = &mut ctx.accounts.mint;
    let mut vault_account = &mut ctx.accounts.vault_account;
    let mut initializer_deposit_token_account = &mut ctx.accounts.initializer_deposit_token_account;
    let mut initializer_receive_token_account = &mut ctx.accounts.initializer_receive_token_account;
    let mut escrow_account = &mut ctx.accounts.escrow_account;
    let mut author = vault_account;

    require!(
        initializer_deposit_token_account.amount >= initializer_amount,
        ProgramError::E000
    );

    escrow_account.initializer_key = initializer.key();

    escrow_account.initializer_amount = initializer_amount;

    escrow_account.taker_amount = taker_amount;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: initializer_deposit_token_account.to_account_info(),
                authority: initializer.to_account_info(),
                to: author.to_account_info(),
            },
        ),
        initializer_amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub mint: Box<Account<'info, token::Mint>>,
    #[account(
        init,
        payer = initializer,
        seeds = ["token-seed".as_bytes().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault_account
    )]
    pub vault_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub initializer_deposit_token_account: Box<Account<'info, token::TokenAccount>>,
    // #[account(mut)]
    pub initializer_receive_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = initializer,
        seeds = ["escrow-main".as_bytes().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<EscrowAccount>()
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        initialize_handler(ctx, initializer_amount, taker_amount)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("In-sufficient balance")]
    E000,
}
