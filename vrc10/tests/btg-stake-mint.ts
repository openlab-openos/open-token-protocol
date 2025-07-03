import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtgStakeMint } from "../target/types/btg_stake_mint";
import { TOKEN_2022_PROGRAM_ID} from "open-token-web3";

describe("btg-stake-mint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const keypair  = anchor.web3.Keypair.fromSecretKey(Buffer.from([16,4,81,162,237,130,82,129,66,194,184,173,91,81,185,175,236,116,28,178,12,103,216,254,6,8,42,78,115,167,148,245,116,19,214,53,118,216,188,3,212,136,185,128,226,26,6,53,206,34,164,82,169,134,129,109,74,78,106,250,248,28,18,137]));

  anchor.setProvider(provider);
  const program = anchor.workspace.BtgStakeMint as Program<BtgStakeMint>;
  const [configPda, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
    );
    console.log("Config PDA:", configPda.toBase58());
  const mint = new anchor.web3.PublicKey("E4vDrY61iWbsWso11fvPxNYFyZb7sMUSkrbCKRLbV4Wj")

  it("Is initialized!", async function(){
    this.timeout(100000);
    // Add your test here.
    const tx = await program.methods.initialize().accounts([
      { pubkey: keypair.publicKey, isWritable: true, isSigner: true },
    ]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Add to whitelist", async function() {
    this.timeout(100000);
    const tx = await program.methods.addToWhitelist("BTG", 0.5)
    .accounts({ authority:keypair.publicKey, mint:mint}).rpc();
    console.log("Add to whitelist tx:", tx);
   
  });

  it("Remove from whitelist", async function() {
    this.timeout(100000);
    const tx = await program.methods.removeFromWhitelist()
    .accounts({ authority:keypair.publicKey, mint:mint}).rpc();
    console.log("Remove from whitelist tx:", tx);
  });
  it("Stake", async function() { 
    this.timeout(100000);
    const keypair = anchor.web3.Keypair.generate()
    const tx = await program.methods.stakeBtg(new anchor.BN(1000000000))
    .accounts({ stakingVault: keypair.publicKey,
      user:keypair.publicKey, 
      mint:mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    
    }).signers([keypair]).rpc();
  })

  it("RedeemToken", async function() { 
    this.timeout(100000);
    const tx = await program.methods.redeemToken()
    .accounts({
      stakingVault: new anchor.web3.PublicKey("EE9RHTK9LwjLSGbkwUojpancEhnN8jdPXH2czdywbJBy"),
      user:keypair.publicKey, 
      mint:mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    }).rpc();
    console.log("RedeemToken tx:", tx);}
  )

  it ("query stake list", async function() { 
    this.timeout(100000);
    const filters: anchor.web3.GetProgramAccountsFilter[] = [
      {
        memcmp: {
          offset: 8, // user 字段偏移量
          bytes: keypair.publicKey.toBase58(), // 匹配的公钥字符串
        },
      },
    ];
    const accountData = await program.account.stakingVault.all(filters);
    console.log("accountData:", accountData);
  } )

  // it("Query config", async () => {
  //   const provider = anchor.getProvider();
  //   const accountData = await program.account.stakeConfig.fetch(configPda);
  //   console.log("Current config:", accountData);
  // });
});
