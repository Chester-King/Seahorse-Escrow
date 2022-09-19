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

