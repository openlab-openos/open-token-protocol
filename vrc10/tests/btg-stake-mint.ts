import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtgStakeMint } from "../target/types/btg_stake_mint";
import { TOKEN_2022_PROGRAM_ID } from "open-token-web3";

describe("btg-stake-mint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BtgStakeMint as Program<BtgStakeMint>;
  const [configPda, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
    );
    console.log("configPda:", configPda.toBase58());
  //  var mint:anchor.web3.PublicKey;

  const mint  = new anchor.web3.PublicKey("USDo1uHcFo9H6aHWcqCkhBiWiMhUqQJFienbKDBPEhN")

   it("Support Tokens", async function(){
    this.timeout(10000);
    const accountData = await program.account.stakeConfig.fetch(configPda);
    console.log("Tokens:", accountData.tokens);
  });

  // it("Is initialized!", async function(){
  //   this.timeout(100000);
  //   // Add your test here.
  //   const tx = await program.methods.initialize().accounts([
  //     { authority: provider.wallet.publicKey, isWritable: true, isSigner: true },
  //   ]).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Add to whitelist", async function() {
  //   this.timeout(100000);
  //   const tx = await program.methods.addToWhitelist("JPY", 0.1467)
  //   .accounts({ authority:provider.wallet.publicKey, mint:mint}).rpc();
  //   console.log("Add to whitelist tx:", tx);
   
  // });

  // it("Remove from whitelist", async function() {
  //   this.timeout(100000);
  //   const tx = await program.methods.removeFromWhitelist()
  //   .accounts({ authority:provider.wallet.publicKey, mint:mint}).rpc();
  //   console.log("Remove from whitelist tx:", tx);
  // });
  // it("Stake", async function() { 
  //   this.timeout(100000);
  //   const keypair = anchor.web3.Keypair.generate()
  //   const tx = await program.methods.stakeBtg(new anchor.BN(1000000000))
  //   .accounts({ stakingVault: keypair.publicKey,
  //     user:provider.wallet.publicKey, 
  //     mint:mint,
  //     tokenProgram: TOKEN_2022_PROGRAM_ID,
    
  //   }).signers([keypair]).rpc();
  // })

  // it("RedeemToken", async function() { 
  //   this.timeout(100000);
  //   const tx = await program.methods.redeemToken()
  //   .accounts({
  //     stakingVault: new anchor.web3.PublicKey("7cgbvJjtTWPHvSu34dwexU8quGyNQwR1fRCvQWNeazsM"),
  //     user:provider.wallet.publicKey, 
  //     mint:mint,
  //     tokenProgram: TOKEN_2022_PROGRAM_ID,
  //   }).rpc();
  //   console.log("RedeemToken tx:", tx);}
  // )
// 
  it ("query stake list", async function() { 
    this.timeout(100000);
    const filters: anchor.web3.GetProgramAccountsFilter[] = [
      {
        memcmp: {
          offset: 8, // user 字段偏移量
          bytes: provider.wallet.publicKey.toBase58(), // 匹配的公钥字符串
        },
      },
    ];
    const accountData = await program.account.stakingVault.all(filters);
    console.log("accountData:", accountData);
  } )

 
});
