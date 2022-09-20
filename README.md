# Seahorse Escrow

This project was created by Seahorse 0.1.6.

To get started, just add your code to **programs_py/escrow.py** and run `seahorse build`.

Note - The code is originally written using seahorse but since seahorse is in beta, few changes were made in the rust code which then satisfy the test cases

## Introduction

This is a simple guide to build a Solana application using seahorse lang. This guide is perfect for new devs aiming to learn Solana development.

## Overview

Any blockchain which deals with currency needs to have a way in which two parties can swap currencies in a trustless way. To enable that we have developed this escrow solana program.
You can refer the original escrow program [here](https://hackmd.io/@ironaddicteddog/solana-anchor-escrow).

## Prerequisites

Let us install the command line tools required to build. We need [Solana](https://docs.solana.com/cli/install-solana-cli-tools), [Anchor](https://book.anchor-lang.com/), [NodeJS](https://nodejs.org/en/) and [Seahorse](https://seahorse-lang.org/docs/installation). The links provided contain the step-by-step guide on installing these tools and the dependencies required for them like Rust.

Once done, you should be able to run the following commands successfully:

solana -V
anchor -V
node -v
seahorse -V

## Initialize

Now we can initialize the project using seahorse init:  
`seahorse init escrow`  

This will create multiple files, you may be familiar with the structure of the project if you have used anchor, seahorse just adds a new folder to write python programs

## What does seahorse do ?

Seahorse reads the python code written by you and essentially ports it into rust and since anchor can read rust anchor is able to understand the code it is able to run tests against the code and deploy it on chain if needed.

## Code walk through

We start by creating an escrow account structure :  
```python
class EscrowAccount(Account):
  initializer_key: Pubkey
  initializer_deposit_token_account: Pubkey
  initializer_receive_token_account: Pubkey
  initializer_amount: u64
  taker_amount: u64
```
this translates to similar account structure in rust :  
```rust
#[derive(Debug)]
#[account]
pub struct EscrowAccount {
    initializer_key: Pubkey,
    initializer_deposit_token_account: Pubkey,
    initializer_receive_token_account: Pubkey,
    initializer_amount: u64,
    taker_amount: u64,
}
```

This account structure will be used to initialize an escrow account which will then be used to store and verify data on later stages.

The code is majorly divided into three major parts

### Part 1 - Initialization

```python
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
  escrow_account.initializer_key = initializer.key()
  escrow_account.initializer_amount = initializer_amount
  escrow_account.taker_amount = taker_amount

  initializer_deposit_token_account.transfer(
    authority=initializer,
    to=author,
    amount=initializer_amount
  )
```

This is the python instruction which initializes the escrow

#### Arguments

* `initializer` - Signer of the transaction(user)
* `mint` - Token address which will be stored in escrow
* `vault_account` - Vault account which will be storing tokens
* `initializer_deposit_token_account` - Token account owned by user from which the tokens will be received in the escrow
* `initializer_receive_token_account` - Token account owned by user which will receive tokens from the other party
* `escrow_account` - Account which stores escrow data
* `initializer_amount` - Amount to be submitted to the escrow
* `taker_amount` - Amount the initializer expects to receive in return

#### Code execution

```python
vault_account.init(
    payer = initializer,
    seeds = ["token-seed"],
    mint = mint,
    authority = vault_account,
  )
```
Translates to rust code to initialize the vault account with seeds


```python
escrow_account.init(
    payer = initializer,
    seeds = ["escrow-main"]
  )
```
Translates to rust code to initialize the escrow account with seeds

`author : TokenAccount = vault_account` - Saving this in author variable because unable to transfer to a type `Empty[TokenAccount]` account so need to typecast so that seahorse can build

Note - The seahorse translation at the time of writing is a bit buggy so manually updated the lib.rs with `let mut author = vault_account;`

```python
escrow_account.initializer_key = initializer.key()
escrow_account.initializer_amount = initializer_amount
escrow_account.taker_amount = taker_amount
```

Saving data in escrow account

```python
initializer_deposit_token_account.transfer(
    authority=initializer,
    to=author,
    amount=initializer_amount
  )
```
Builds the token transfer instruction and transfers token out of initializer_deposit_token_account and put them in vault_account


### Part 2 - Cancel

In case the initializer decides to cancel the transaction and get back his tokens he can use this.

```python
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
```

This is the python instruction which cancels the escrow

#### Arguments

* `initializer` - Signer of the transaction(user)
* `mint` - Token address which will be stored in escrow
* `vault_account` - Vault account which will be storing tokens
* `initializer_deposit_token_account` - Token account owned by user 
* `escrow_account` - Account which stores escrow data
* `bump1` - Bump to be used for signing transactions using PDA
* `bump2` - Bump to be used for signing transactions using PDA

#### Code execution

```python
assert ((escrow_account.initializer_key == initializer.key())), "Cancel Constraint violation"
  
  vault_account.transfer(
    authority=vault_account,
    to=initializer_deposit_token_account,
    amount=escrow_account.initializer_amount
  )
```
Translates to rust code to transfer token back from vault_account

Note - As of now Seahorse does not translate mutable PDA accounts with seeds so that part is manually added to the rust code. You can see lib.rs for reference

Making sure only initializer can cancel the escrow

```python
vault_account.transfer(
    authority=vault_account,
    to=initializer_deposit_token_account,
    amount=escrow_account.initializer_amount
  )
```

Builds the token transfer instruction and transfers token out of vault_account  and put them in initializer_deposit_token_account

### Part 3 - Exchange

Called by the other party to accept the exchange

```python
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
  
```

This is the python instruction which exchanges token using the escrow

#### Arguments

* `taker` - Signer of the transaction(second user)
* `vault_account` - Vault account which has first user's tokens
* `taker_deposit_token_account` - Token account of deposited tokens owned by second user
* `taker_receive_token_account` - Token account of receiving tokens owned by second user
* `initializer_deposit_token_account` - Token account of deposited tokens owned by first user 
* `initializer_receive_token_account` - Token account of receiving tokens owned by first user
* `escrow_account` - Account which stores escrow data
* `bump1` - Bump to be used for signing transactions using PDA
* `bump2` - Bump to be used for signing transactions using PDA

#### Code execution

```python
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
```
Translates to rust code to transfer token to correct parties

Note - As of now Seahorse does not translate mutable PDA accounts with seeds so that part is manually added to the rust code. You can see lib.rs for reference. Also Seahorse does not translate CPI using PDAs as signer as of now so that part also is added to rust code manually.

Making sure correct addresses are received in accounts.

```python
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
```

Builds the token transfer instruction and transfers token out of vault_account  and put them in taker_deposit_token_account. Also transfers token out of second user's taker_receive_token_account and transfers them to initializer_receive_token_account. Thus completing the escrow functionality. 

## Conclusion
Seahorse vastly simplifies developing programs on Solana at the same time it acts as a gateway for python programmers to get familiar with Rust programs as Seahorse automatically generated the Anchor Rust program for you.
