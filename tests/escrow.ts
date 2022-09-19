import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Escrow } from "../target/types/escrow";

describe("escrow", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)


  const program = anchor.workspace.Escrow as Program<Escrow>;

  // Set up some common accounts we'll be using later
  const owner = provider.wallet.publicKey

  const escrowAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({ owner , escrow : escrowAccount.publicKey}).rpc();
    console.log("Your transaction signature", tx);
  });
});
