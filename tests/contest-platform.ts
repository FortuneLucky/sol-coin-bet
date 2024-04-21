import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ContestPlatform } from "../target/types/contest_platform";
import { SystemProgram, Keypair, PublicKey, Transaction, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, getAssociatedTokenAddress , ASSOCIATED_TOKEN_PROGRAM_ID,createMint, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync,  } from "@solana/spl-token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
describe("contest-platform", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ContestPlatform as Program<ContestPlatform>;
  const provider = anchor.AnchorProvider.env();

  let globalState, contest, tokenVaultAccountAlice, tokenOwnerAccountAlice, tokenOwnerAccountBob, tokenVaultAccountBob, contestAliceInfo,contestBobInfo, tokenFeeAccountAlice, tokenFeeAccountBob: PublicKey;
  let globalStateBump, contestBump, tokenVaultAliceBump,tokenVaultBobBump, contestAliceInfoBump,contestBobInfoBump: Number;

  const contestIndex = 2;

  const GLOBAL_STATE_SEED = "GLOBAL-STATE-SEED";
  const CONTEST_CREATE_SEED = "CONTEST-CREATE-SEED"; 
  const TOKEN_VAULT_SEED = "TOKEN-VAULT-SEED";
  const CONTEST_INFO_SEED = "CONTEST-INFO-SEED";

  const tokenMintAlice = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE"); //Alice Meme
  const tokenMintBob = new PublicKey("5hyJ6h3ABjF7zEBhc32LWT5ZUCkNx4AZkdRzKC1MUHRb"); // Bob Meme

  const baseAliceVault = new PublicKey("D4um39AKjMcBHj5XGTfDHMVSpFUrG1GundkVcgiCuqRG");
  const quoteAliceSolVault = new PublicKey("CZfQEs1bdWDP4sqLVmhRFi5g95ft6H89Viv6ENSYAJhN");
  const baseBobVault = new PublicKey("HVDF4i1ZYXgt6oGjoNym33Gx9N1oKb5jweNfGJfmwGo9");
  const quoteBobSolVault = new PublicKey("BLYWqzQ6AjZ7GfVe7rfTHo82WvjrTjXN4AjSmsAWUMXc");

  const tokenAliceName = "Alice Meme";
  const tokenBobName = "Bob Meme";
  const tokenAliceImage = "https://ipfs.io/ipfs/QmRwFECSjNcCtH6rBfs5AqgfXiZyPyYR4PUaWzS6R5aCmV/Alice.png";
  const tokenBobImage = "https://ipfs.io/ipfs/QmRwFECSjNcCtH6rBfs5AqgfXiZyPyYR4PUaWzS6R5aCmV/Bob.png";

  let payer = Keypair.fromSecretKey(Uint8Array.from([40,99,26,70,105,80,7,101,254,157,6,15,246,207,151,29,5,142,33,154,246,128,6,190,239,191,147,115,241,217,13,169,63,7,158,42,242,198,39,230,40,85,41,68,22,57,86,10,229,14,159,81,159,159,3,218,116,30,3,106,54,57,221,134]));
  let depositer = Keypair.fromSecretKey(
    Uint8Array.from([66,206,49,111,232,143,171,223,90,21,174,154,103,27,177,123,67,40,52,33,37,167,75,76,167,195,34,48,203,118,95,213,160,4,246,134,216,28,38,1,182,255,216,123,146,224,96,81,175,128,250,34,55,16,243,170,237,66,212,214,200,255,225,131])
  );
    
  it("is init accounts", async () => {
    [globalState, globalStateBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(GLOBAL_STATE_SEED)
      ],
      program.programId
    );

    [contest, contestBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(CONTEST_CREATE_SEED),
        new anchor.BN(contestIndex).toBuffer('le', 2)
      ],
      program.programId
    );
    [contestAliceInfo, contestAliceInfoBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(CONTEST_INFO_SEED),
        new anchor.BN(contestIndex).toBuffer('le', 2),
        depositer.publicKey.toBuffer(),
        tokenMintAlice.toBuffer()
      ],
      program.programId
    );

    tokenOwnerAccountAlice = await getAssociatedTokenAddress(
      tokenMintAlice,
      depositer.publicKey
    );
    
    [contestBobInfo, contestBobInfoBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(CONTEST_INFO_SEED),
        new anchor.BN(contestIndex).toBuffer('le', 2),
        depositer.publicKey.toBuffer(),
        tokenMintBob.toBuffer(),
      ],
      program.programId
    );

    tokenOwnerAccountBob = await getAssociatedTokenAddress(
      tokenMintBob,
      depositer.publicKey
    );

    [tokenVaultAccountAlice, tokenVaultAliceBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(TOKEN_VAULT_SEED),
        tokenMintAlice.toBuffer()
      ],
      program.programId
    );
    [tokenVaultAccountBob, tokenVaultBobBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(TOKEN_VAULT_SEED),
        tokenMintBob.toBuffer()
      ],
      program.programId
    );
    console.log("tokenVaultAccountBob->", tokenVaultAccountBob.toString());
    
    tokenFeeAccountAlice = await getAssociatedTokenAddress(
      tokenMintAlice,
      payer.publicKey
    );
    console.log("tokenFeeAccountAlice->", tokenFeeAccountAlice.toString());

    tokenFeeAccountBob = await getAssociatedTokenAddress(
      tokenMintBob,
      payer.publicKey
    );
  });

  it("Is initialized!", async () => {
    // Add your test here.
  
    const tx = await program.rpc.initialize(
      new anchor.BN(2),
      {
        accounts: {
          admin: payer.publicKey,
          globalState,
          systemProgram: SystemProgram.programId
        },
        signers: [payer]
      }
    );
    console.log("Your transaction signature", tx);
  }); 
  /*
  it("init the token alice valut account", async() => {
    
    const tx1 = await program.rpc.initTokenAccountForNewContest(
      contestIndex,
      {
        accounts: {
          owner: payer.publicKey,
          globalState,
          tokenMint: tokenMintAlice,
          tokenVaultAccount: tokenVaultAccountAlice,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [payer]
      }
    );
    console.log("Init the alice token vault account:->", tx1);

    const tx2 = await program.rpc.initTokenAccountForNewContest(
      contestIndex,
      {
        accounts: {
          owner: payer.publicKey,
          globalState,
          tokenMint: tokenMintBob,
          tokenVaultAccount: tokenVaultAccountBob,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [payer]
      }
    );
    console.log("Init the mint token vault account:->", tx2);
  });  


  it("create the contest", async()=> {

    try {
      const tx = await program.rpc.createContest(
        contestIndex,
        new anchor.BN(200),
        new anchor.BN(100),
        tokenAliceName,
        tokenBobName,
        tokenAliceImage,
        tokenBobImage,
        tokenFeeAccountAlice,
        tokenFeeAccountBob,
        baseAliceVault,
        quoteAliceSolVault,
        baseBobVault,
        quoteBobSolVault,
        {
          accounts: {
            owner: payer.publicKey,
            globalState,
            contest,
            tokenMintAlice,
            tokenMintBob,
            baseAliceVault,
            quoteAliceSolVault,
            baseBobVault,
            quoteBobSolVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY
          },
          signers: [payer]
        }
      );

      const contestData = await program.account.contest.fetch(contest);
      console.log(contestData);
      console.log(Number(contestData.startTime));
      console.log(Number(contestData.startTime) + Number(contestData.contestTime));
      console.log(Number(contestData.waitTime));
      console.log(Date.now() / 1000);
    } catch (error) {
      console.log(error);
    }
  }); */
  /*
  it("depost the alice token to the contest", async() => {
    try {
      const deposit_contest_tx = await program.rpc.depositAliceTokenContest(
        contestIndex,
        new anchor.BN(250000),
        {
          accounts:{
            user: depositer.publicKey,
            globalState,
            contest,
            contestInfo: contestAliceInfo,
            tokenMint: tokenMintAlice,
            tokenOwnerAccount: tokenOwnerAccountAlice,
            tokenVaultAccount:tokenVaultAccountAlice,
            baseAliceVault: baseAliceVault,
            quoteAliceSolVault: quoteAliceSolVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY
          },
          signers: [depositer]
        }
      );

      console.log("deposit tx: ", deposit_contest_tx);
      const contestData = await program.account.contest.fetch(contest);
      const contestInfoData = await program.account.contestInfo.fetch(contestAliceInfo);
      console.log(Number(contestInfoData.bet));
    } catch (error) {
      console.log(error);
    }
  }); 
  
  it("depost the bob token to the contest", async() => {
    try {
      const deposit_contest_tx = await program.rpc.depositBobTokenContest(
        contestIndex,
        new anchor.BN(100000),
        {
          accounts:{
            user: depositer.publicKey,
            globalState,
            contest,
            contestInfo: contestBobInfo,
            tokenMint: tokenMintBob,
            tokenOwnerAccount: tokenOwnerAccountBob,
            tokenVaultAccount:tokenVaultAccountBob,
            baseBobVault: baseBobVault,
            quoteBobSolVault: quoteBobSolVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY
          },
          signers: [depositer]
        }
      );
      console.log("deposit tx: ", deposit_contest_tx);
      const contestData = await program.account.contest.fetch(contest);
      const contestInfoData = await program.account.contestInfo.fetch(contestBobInfo);
      console.log(Number(contestInfoData.bet));
    } catch (error) {
      console.log(error);
    }
  });*/
  
  /*
  it("set the winner in the contest", async() => {
    const tx = await program.rpc.setWinner(
      contestIndex,{
        accounts: {
          owner: payer.publicKey,
          globalState,
          contest,
          tokenMintAlice,
          tokenMintBob,
          systemProgram: SystemProgram.programId,
          clock: SYSVAR_CLOCK_PUBKEY
        },
        signers: [payer]
      }
    );
    const contestData = await program.account.contest.fetch(contest);
    console.log(contestData);
  }) */
  /*
  it("winner claim", async() => {
    const contestAliceData = await program.account.contestInfo.fetch(contestAliceInfo);

    const contestData = await program.account.contest.fetch(contest);
    try{
      const tx = await program.rpc.winnerClaim(
        contestIndex,
        {
          accounts: {
            user: depositer.publicKey,
            globalState,
            contest,
            contestInfo: contestAliceInfo,
            tokenMintAlice,
            tokenVaultAccountAlice,
            tokenOwnerAccountAlice,
            tokenFeeAccountAlice,
            tokenMintBob,
            tokenVaultAccountBob,
            tokenOwnerAccountBob,
            tokenFeeAccountBob,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY         
          },
          signers: [depositer]
        }
      )
    } catch(error) {
      console.log(error);
    }
  }) */
  /*
  it("withdraw", async() => {
    try{
      const tx = await program.rpc.withdrawToken(
        contestIndex,
        {
          accounts: {
            owner: payer.publicKey,
            globalState,
            contest,
            tokenMintAlice,
            tokenVaultAccountAlice,
            tokenFeeAccountAlice,
            tokenMintBob,
            tokenVaultAccountBob,
            tokenFeeAccountBob,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY         
          },
          signers: [payer]
        }
      )
    } catch(error) {
      console.log(error);
    }
  }) */
  /*
  it("update fee", async() => {
    const new_fee = 3;
    const tx = await program.rpc.updateFee(
      new anchor.BN(new_fee), 
      {
        accounts: {
          owner: payer.publicKey,
          globalState,
          systemProgram: SystemProgram.programId
        },
        signers: [payer]
      }
    );
  });

  it("update owner", async() => {
    const new_owner = new PublicKey("sfsdfsdfsd");
    const tx = await program.rpc.updateOwner(
      new_owner, 
      {
        accounts: {
          owner: payer.publicKey,
          globalState,
          systemProgram: SystemProgram.programId
        },
        signers: [payer]
      }
    );
  });
  */
  
});
// Utility function to sleep for a specified duration
async function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
