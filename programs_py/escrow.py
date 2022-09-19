# escrow
# Built with Seahorse v0.1.6

from tokenize import Token
from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class EscrowAccount(Account):
  initializer_key: Pubkey
  initializer_deposit_token_account: Pubkey
  initializer_receive_token_account: Pubkey
  initializer_amount: u64
  taker_amount: u64

@instruction
def initialize(initializer: Signer, 
                mint: TokenMint,
                vault_account: Empty[TokenAccount],
                initializer_deposit_token_account : TokenAccount,
                initializer_receive_token_account : TokenAccount,
                escrow_account: Empty[EscrowAccount],
                initializer_amount: u64,
                taker_amount: u64
                ):
  # init a vault
  vault_account.init(
    payer = initializer,
    seeds = ["token-seed"],
    mint = mint,
    authority = vault_account,
  )
  escrow_account.init(
    payer = initializer,
    seeds = ["escrow-main"]
  )
  author : TokenAccount = vault_account
  assert initializer_deposit_token_account.amount >= initializer_amount, 'In-sufficient balance'
  # Initialize the calculator and set the owner
  escrow_account.initializer_key = initializer.key()
  escrow_account.initializer_amount = initializer_amount
  escrow_account.taker_amount = taker_amount

  initializer_deposit_token_account.transfer(
    authority=initializer,
    to=author,
    amount=initializer_amount
  )

@instruction
def cancel(initializer: Signer, 
            mint: TokenMint,
            vault_account: TokenAccount,
            initializer_deposit_token_account : TokenAccount,
            escrow_account: EscrowAccount,
            bump1 : u8,
            bump2 : u8
            ):
  assert ((escrow_account.initializer_key == initializer.key())), "Cancel Constraint violation"
  
  vault_account.transfer(
    authority=vault_account,
    to=initializer_deposit_token_account,
    amount=escrow_account.initializer_amount
  )
  

@instruction
def exchange(taker: Signer,
            vault_account: TokenAccount,
            taker_deposit_token_account : TokenAccount,
            taker_receive_token_account : TokenAccount,
            initializer_deposit_token_account : TokenAccount,
            initializer_receive_token_account : TokenAccount,
            escrow_account: EscrowAccount,
            bump1 : u8,
            bump2 : u8
            ):
  assert ((escrow_account.initializer_key == initializer_deposit_token_account.owner) and (escrow_account.initializer_key == initializer_receive_token_account.owner)), "Exchange Constraint violation"
  


  vault_account.transfer(
    authority=vault_account,
    to=taker_deposit_token_account,
    amount=escrow_account.initializer_amount
  )

  taker_receive_token_account.transfer(
    authority=taker,
    to=initializer_receive_token_account,
    amount=escrow_account.taker_amount
  )
  
  