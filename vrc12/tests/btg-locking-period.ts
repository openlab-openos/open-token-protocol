import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtgLockingPeriod } from "../target/types/btg_locking_period";

describe("btg-locking-period", () => {
  // Configure the client to use the local cluster.
  const provider  = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BtgLockingPeriod as Program<BtgLockingPeriod>;
  const lockAccount = anchor.web3.Keypair.generate();
  // it("run lock", async () => {
  //   const amount = 1_000_000_000; // Example amount in lamports
  //   const endTime = (new Date().getTime() / 1000) + 6; // 6s from now

  //   const tx = await program.methods.lock(new anchor.BN(amount) , new anchor.BN(endTime)).accounts({
  //     lockAccount: lockAccount.publicKey,
  //     mint: new anchor.web3.PublicKey("GragM9tHgicpxtf9qrTkbY1fFZYA8CJaDgLuFnZikdqs"),
  //     owner: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .signers([lockAccount])
  //   .rpc();
  //   console.log(tx);

  //   await new Promise(resolve => setTimeout(resolve, 10000));
  // });


  // it("run unlock", async () => {
  //   const tx = await program.methods.unlock().accounts({
  //     lockAccount: lockAccount.publicKey,
  //     owner: provider.wallet.publicKey,
  //   })
  //   .rpc();
  //   console.log(tx);
  // });


  it("all lock schedule list", async () => {
    const tx = await provider.connection.getProgramAccounts(new anchor.web3.PublicKey("99mpy2LmaD747bwWSLs5sNHPuRRPMdd1aR22HeAceqUh"), {
      "filters": [
        {
          "memcmp": {
            "offset": 8,
            "bytes": "GragM9tHgicpxtf9qrTkbY1fFZYA8CJaDgLuFnZikdqs"
          }
        }
      ]
    });
    console.log(tx);
  });
  it("my lock schedule list", async () => {
    const tx = await provider.connection.getProgramAccounts(new anchor.web3.PublicKey("99mpy2LmaD747bwWSLs5sNHPuRRPMdd1aR22HeAceqUh"), {
      "filters": [
        {
          "memcmp": {
            "offset": 8,
            "bytes": "GragM9tHgicpxtf9qrTkbY1fFZYA8CJaDgLuFnZikdqs"
          }
        },
        {
          "memcmp": {
            "offset": 40,
            "bytes": provider.publicKey.toBase58()
          }
        }
      ]
    });
    console.log(tx);
  });
});
