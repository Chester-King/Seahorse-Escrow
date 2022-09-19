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
                escrow_account: EscrowAccount,
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

  
  
  