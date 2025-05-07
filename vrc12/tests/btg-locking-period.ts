import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtgLockingPeriod } from "../target/types/btg_locking_period";

describe("btg-locking-period", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const mint = new anchor.web3.PublicKey("GragM9tHgicpxtf9qrTkbY1fFZYA8CJaDgLuFnZikdqs");

  const program = anchor.workspace.BtgLockingPeriod as Program<BtgLockingPeriod>;
  const lockAccount = anchor.web3.Keypair.generate();



  // //lock test success
  it("run lock", async () => {
    const amount = 1_000_000_000; // Example amount in lamports
    const endTime = (new Date().getTime() / 1000) + 7 * 86400; // 6s from now

    const tx = await program.methods.lock(new anchor.BN(amount), new anchor.BN(endTime)).accounts({
      lockAccount: lockAccount.publicKey,
      mint: mint,
      owner: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
      .signers([lockAccount])
      .rpc();
    console.log(tx);

    await new Promise(resolve => setTimeout(resolve, 10000));
  });


  // //unlock test success
  // it("run unlock", async () => {
  //   const tx = await program.methods.unlock().accounts({
  //     lockAccount: lockAccount.publicKey,
  //     owner: provider.wallet.publicKey,
  //   })
  //     .rpc();
  //   console.log(tx);
  // });

  //all accounts list
  it("all lock accounts list", async () => {
    const filters = [
      {
        "memcmp": {
          "offset": 8,
          "bytes": mint.toBase58()
        }
      }
    ]
    const allLockAccounts = await program.account.lockAccount.all(filters);
    console.log("All lock Accouts length:", allLockAccounts.length);
    console.log("Locking length is:", allLockAccounts.filter(item => !item.account.isUnlocked).length);
    console.log("Unlocked length is:", allLockAccounts.filter(item => item.account.isUnlocked).length);
    allLockAccounts.sort((a, b) => a.account.startTime.toNumber() - b.account.startTime.toNumber());
    console.log("All Lock Accounts list:", allLockAccounts);
  });


  // my lock accounts list
  it("my lock schedule list", async () => {
      const filters = [
        {
          "memcmp": {
            "offset": 8,
            "bytes":  mint.toBase58()
          }
        },
        {
          "memcmp": {
            "offset": 40,
            "bytes": provider.publicKey.toBase58()
          }
        }
      ]
      const myLockAccounts = await program.account.lockAccount.all(filters);
      console.log("My lock Accouts length:", myLockAccounts.length);
      console.log("Locking length is:", myLockAccounts.filter(item => !item.account.isUnlocked).length);
      console.log("Unlocked length is:", myLockAccounts.filter(item => item.account.isUnlocked).length);
      myLockAccounts.sort((a, b) => a.account.startTime.toNumber() - b.account.startTime.toNumber());
      console.log("My Lock Accounts list:", myLockAccounts);
  });
});
