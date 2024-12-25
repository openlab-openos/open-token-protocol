import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtgLockingPeriod } from "../target/types/btg_locking_period";

describe("btg-locking-period", () => {
  // Configure the client to use the local cluster.
  const provider  = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BtgLockingPeriod as Program<BtgLockingPeriod>;
  const lockAccount = anchor.web3.Keypair.generate();
  it("run lock", async () => {
    // Add your test here.
    const amount = 1_000_000_000; // Example amount in lamports
    const endTime = (new Date().getTime() / 1000) + 6; // One hour from now

    const tx = await program.methods.lock(new anchor.BN(amount) , new anchor.BN(endTime)).accounts({
      lockAccount: lockAccount.publicKey,
      mint: new anchor.web3.PublicKey("GragM9tHgicpxtf9qrTkbY1fFZYA8CJaDgLuFnZikdqs"),
      owner: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([lockAccount])
    .rpc();
    console.log(tx);

    await new Promise(resolve => setTimeout(resolve, 10000));
  });


  it("run unlock", async () => {
    const tx = await program.methods.unlock().accounts({
      lockAccount: lockAccount.publicKey,
      owner: provider.wallet.publicKey,
    })
    .rpc();
    console.log(tx);
  });

});
