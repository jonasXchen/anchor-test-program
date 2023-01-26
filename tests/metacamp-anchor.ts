import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MetacampAnchor } from "../target/types/metacamp_anchor";

describe("metacamp-anchor", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.MetacampAnchor as Program<MetacampAnchor>;

  it("Is initialized!", async () => {

    const keypair = anchor.web3.Keypair.generate()

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(keypair.publicKey, 1e9)
    );


    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("1")), keypair.publicKey.toBuffer()],
      program.programId
    ) 

console.log("pda: ",pda.toBase58())
console.log("keypair: ",keypair.publicKey.toBase58())
    // Add your test here.
    const tx = await program.methods
      .initialize(
        "Account1",
      "1"
      )
      .accounts({ onchainAccount: pda, initializer: keypair.publicKey })
      .signers([keypair])
      .rpc();
      console.log("Your transaction signature", tx);
      });
});
