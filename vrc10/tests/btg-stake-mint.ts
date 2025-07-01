import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtgStakeMint } from "../target/types/btg_stake_mint";
import { PublicKey } from "@solana/web3.js";

describe("btg-stake-mint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BtgStakeMint as Program<BtgStakeMint>;

  it("Is initialized!", async () => {
    // Add your test here.
    const [address, bump] = await PublicKey.findProgramAddress(
    [Buffer.from("config")],
    program.programId
  );

    const tx = await program.methods.initialize().accounts({
      config: address,
      authority: anchor.Wallet.local().publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Add to whitelist", async () => {
  //   const tx = await program.methods.addToWhitelist("BTG", 1.0).rpc();
  //   console.log("Add to whitelist tx:", tx);
  // });

  // it("Remove from whitelist", async () => {
  //   const tx = await program.methods.removeFromWhitelist().rpc();
  //   console.log("Remove from whitelist tx:", tx);
  // });

  // it("Query config", async () => {
  //   const provider = anchor.getProvider();
  //   const configAccountPubkey = (await program.account.config.all())[0].publicKey;
  //   const accountData = await program.account.stakeConfig.fetch(configAccountPubkey);
  //   console.log("Current config:", accountData);
  // });
});
