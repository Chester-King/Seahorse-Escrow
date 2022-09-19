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

pub fn cancel_handler(mut ctx: Context<Cancel>, mut bump1: u8, mut bump2: u8) -> Result<()> {
    let mut initializer = &mut ctx.accounts.initializer;
    let mut mint = &mut ctx.accounts.mint;
    let mut vault_account = &mut ctx.accounts.vault_account;
    let mut initializer_deposit_token_account = &mut ctx.accounts.initializer_deposit_token_account;
    let mut escrow_account = &mut ctx.accounts.escrow_account;

    require!(
        (escrow_account.initializer_key == initializer.key()),
        ProgramError::E001
    );

    let bump_vector = bump1.to_le_bytes();
    let inner = vec![b"token-seed".as_ref(),bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: vault_account.to_account_info(),
                authority: vault_account.to_account_info(),
                to: initializer_deposit_token_account.to_account_info(),
            },
            outer.as_slice()
        ),
        escrow_account.initializer_amount,
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
    #[account(mut)]
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

#[derive(Accounts)]
#[instruction(_b1 : u8,_b2 : u8)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub mint: Box<Account<'info, token::Mint>>,
    #[account(mut,
        seeds = ["token-seed".as_bytes().as_ref()],
        bump=_b1)]
    pub vault_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub initializer_deposit_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        seeds = ["escrow-main".as_bytes().as_ref()],
        bump=_b2)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub token_program: Program<'info, token::Token>,
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

    pub fn cancel(ctx: Context<Cancel>, bump1: u8, bump2: u8) -> Result<()> {
        cancel_handler(ctx, bump1, bump2)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("In-sufficient balance")]
    E000,
    #[msg("Constraint violation")]
    E001,
}
