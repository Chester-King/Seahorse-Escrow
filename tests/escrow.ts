import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Escrow } from "../target/types/escrow";
import * as spl from "@solana/spl-token";
const {
  Connection,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
  SystemProgram
} = anchor.web3;

describe("escrow", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  let basemintKey;

  const program = anchor.workspace.Escrow as Program<Escrow>;

  // Set up some common accounts we'll be using later
  const owner = provider.wallet.publicKey

  const escrowAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    basemintKey = anchor.web3.Keypair.generate();

      // Deterministically finding out the project token's ATA owned by provider.wallet
      let base_ata = await spl.getAssociatedTokenAddress(basemintKey.publicKey, provider.wallet.publicKey, false, spl.TOKEN_PROGRAM_ID, spl.ASSOCIATED_TOKEN_PROGRAM_ID);

      // Creating transaction to Initialize basemintKey keypair as a token and then minting tokens to  ATA owned by provider.wallet

      let create_mint_tx = new Transaction().add(
        // create mint account
        SystemProgram.createAccount({
          fromPubkey: provider.wallet.publicKey,
          newAccountPubkey: basemintKey.publicKey,
          space: spl.MintLayout.span,
          lamports: await spl.getMinimumBalanceForRentExemptMint(program.provider.connection),
          programId: spl.TOKEN_PROGRAM_ID,
        }),
        // init mint account
        spl.createInitializeMintInstruction(basemintKey.publicKey, 7, provider.wallet.publicKey, provider.wallet.publicKey, spl.TOKEN_PROGRAM_ID)
      )
      .add(
        spl.createAssociatedTokenAccountInstruction(
          provider.wallet.publicKey, base_ata, provider.wallet.publicKey, basemintKey.publicKey, spl.TOKEN_PROGRAM_ID, spl.ASSOCIATED_TOKEN_PROGRAM_ID
        )
      ).add(
        spl.createMintToInstruction(// always TOKEN_PROGRAM_ID
          basemintKey.publicKey, // mint
          base_ata, // receiver (sholud be a token account)
          provider.wallet.publicKey, // mint authority
          3e8,
          [], // only multisig account will use. leave it empty now.
          spl.TOKEN_PROGRAM_ID,  // amount. if your decimals is 8, you mint 10^8 for 1 token.
        ));

      await program.provider.sendAndConfirm(create_mint_tx, [basemintKey]);
  });
});
