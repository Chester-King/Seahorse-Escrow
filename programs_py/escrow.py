# escrow
# Built with Seahorse v0.1.6

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
  vault_account.init(
    payer = initializer,
    seeds = ["token-seed"],
    mint = mint,
    authority = initializer,
  )
  assert initializer_deposit_token_account.amount >= initializer_amount, 'In-sufficent balance'
  # Initialize the calculator and set the owner
  escrow_account.initializer_key = initializer.key()
  # init a vault
  