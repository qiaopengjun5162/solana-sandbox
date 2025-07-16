import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { CrowdfundingRedpacket } from "../target/types/crowdfunding_redpacket";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

/**
 * 合约的整个生命周期，包括：

1. 创建项目
2. 用户支持（众筹）
3. 用户领取空投
4. 结算（成功和失败两种情况）
5. 成功后的操作（领取奖励代币、领取开发基金、分配费用）
6. 失败后的操作（退款）
 */

// Helper function to sleep for a given time
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

describe("crowdfunding_redpacket", () => {
  // --- 全局设置 ---
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .crowdfundingRedpacket as Program<CrowdfundingRedpacket>;

  // 定义测试中使用的密钥对
  const creator = Keypair.generate();
  const backer1 = Keypair.generate();
  const backer2 = Keypair.generate();
  const airdropClaimer1 = Keypair.generate();
  const developerWallet = Keypair.generate();
  // 一个全局的 admin，用于初始化 Config
  const admin = Keypair.generate();

  // 定义代币和PDA
  let mint: PublicKey;
  let creatorTokenAccount: PublicKey;
  let claimer1TokenAccount: PublicKey;
  let backer1TokenAccount: PublicKey;
  let backer2TokenAccount: PublicKey;

  // PDA 地址
  let configPDA: PublicKey;
  let redPacketPDA: PublicKey;
  let solVaultPDA: PublicKey;
  let tokenVaultPDA: PublicKey;

  // 定义一些常量，便于测试
  const MINT_TOTAL_SUPPLY = new BN(1_000_000 * 10 ** 9); // 假设代币精度为9
  const FUNDING_GOAL = new BN(0.5 * LAMPORTS_PER_SOL); // 目标：0.5 SOL
  const SMALL_SUPPORT_AMOUNT = new BN(0.05 * LAMPORTS_PER_SOL); // 0.05 SOL
  const LARGE_SUPPORT_AMOUNT = new BN(0.5 * LAMPORTS_PER_SOL); // 0.5 SOL

  // --- 测试开始前的准备工作 ---
  before(async () => {
    // 给所有测试账户空投SOL，用于支付gas费
    await provider.connection.requestAirdrop(
      creator.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      backer1.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      backer2.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      airdropClaimer1.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      admin.publicKey,
      5 * LAMPORTS_PER_SOL
    );

    // 等待空投确认
    await new Promise((resolve) => setTimeout(resolve, 2000));

    console.log("Test wallets funded.");
  });

  // --- 0. 全局配置初始化 ---
  describe("0. Global Configuration", () => {
    it("Initializes the global config account", async () => {
      [configPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        program.programId
      );

      await program.methods
        .initializeConfig(developerWallet.publicKey)
        .accounts({
          admin: admin.publicKey,
          config: configPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      const configAccount = await program.account.config.fetch(configPDA);
      assert.equal(configAccount.admin.toBase58(), admin.publicKey.toBase58());
      assert.equal(
        configAccount.developerWallet.toBase58(),
        developerWallet.publicKey.toBase58()
      );
      console.log("Global config initialized successfully.");
    });
  });

  // --- 1. 初始化和创建项目 ---
  describe("1. Initialization and Creation", () => {
    it("Sets up the test environment (mints tokens)", async () => {
      // 创建一个新的代币
      mint = await createMint(
        provider.connection,
        creator, // payer
        creator.publicKey, // mint authority
        null, // freeze authority
        9 // decimals
      );

      // 为各个账户创建关联代币账户 (ATA)
      creatorTokenAccount = await createAssociatedTokenAccount(
        provider.connection,
        creator,
        mint,
        creator.publicKey
      );
      claimer1TokenAccount = await createAssociatedTokenAccount(
        provider.connection,
        creator, // airdrop claimer doesn't have SOL yet, so creator pays
        mint,
        airdropClaimer1.publicKey
      );
      backer1TokenAccount = await createAssociatedTokenAccount(
        provider.connection,
        creator,
        mint,
        backer1.publicKey
      );
      backer2TokenAccount = await createAssociatedTokenAccount(
        provider.connection,
        creator,
        mint,
        backer2.publicKey
      );

      // 给创建者的ATA里铸造代币
      await mintTo(
        provider.connection,
        creator,
        mint,
        creatorTokenAccount,
        creator.publicKey,
        MINT_TOTAL_SUPPLY.toNumber()
      );

      const creatorAtaInfo = await getAccount(
        provider.connection,
        creatorTokenAccount
      );
      assert.ok(
        new BN(creatorAtaInfo.amount.toString()).eq(MINT_TOTAL_SUPPLY),
        "Creator should have the total supply"
      );
      console.log(`Mint Address: ${mint.toBase58()}`);
      console.log(`Creator ATA: ${creatorTokenAccount.toBase58()}`);
    });

    it("Successfully creates a new red packet with default allocations", async () => {
      // 找到 PDA 地址
      [redPacketPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("red_packet"), creator.publicKey.toBuffer()],
        program.programId
      );
      [solVaultPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("sol_vault"), redPacketPDA.toBuffer()],
        program.programId
      );
      [tokenVaultPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("token_vault"), redPacketPDA.toBuffer()],
        program.programId
      );

      // 定义创建参数 (使用默认分配)
      const params = {
        mint: mint,
        totalAmount: MINT_TOTAL_SUPPLY,
        tokenName: "TEST",
        tokenSymbol: "TST",
        fundingGoal: FUNDING_GOAL,
        allocations: [], // 空数组以触发默认分配
        airdropMaxCount: new anchor.BN(100),
        expiryDuration: new anchor.BN(3), // 3 秒
      };

      await program.methods
        .createCustomRedpacket(params)
        .accounts({
          creator: creator.publicKey,
          redPacket: redPacketPDA,
          creatorTokenAccount: creatorTokenAccount,
          solVault: solVaultPDA,
          tokenVault: tokenVaultPDA,
          mint: mint,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([creator])
        .rpc();

      // 验证状态
      const redPacketAccount = await program.account.redPacket.fetch(
        redPacketPDA
      );
      assert.equal(
        redPacketAccount.creator.toBase58(),
        creator.publicKey.toBase58()
      );
      assert.ok(redPacketAccount.fundingGoal.eq(FUNDING_GOAL));
      assert.equal(redPacketAccount.tokenName, "TEST");

      // 验证代币是否已转入金库
      const tokenVaultInfo = await getAccount(
        provider.connection,
        tokenVaultPDA
      );
      assert.ok(new BN(tokenVaultInfo.amount.toString()).eq(MINT_TOTAL_SUPPLY));
      console.log("Red Packet created successfully.");
    });
  });

  // --- 2. 众筹进行中 ---
  describe("2. Crowdfunding In Progress", () => {
    it("Allows users to support the crowdfunding", async () => {
      // Backer 1 支持
      const [backer1StatePDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("backer_state"),
          redPacketPDA.toBuffer(),
          backer1.publicKey.toBuffer(),
        ],
        program.programId
      );
      await program.methods
        .supportCrowdfunding(LARGE_SUPPORT_AMOUNT)
        .accounts({
          redPacket: redPacketPDA,
          backer: backer1.publicKey,
          backerState: backer1StatePDA,
          solVault: solVaultPDA,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([backer1])
        .rpc();

      // Backer 2 支持
      const [backer2StatePDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("backer_state"),
          redPacketPDA.toBuffer(),
          backer2.publicKey.toBuffer(),
        ],
        program.programId
      );
      await program.methods
        .supportCrowdfunding(SMALL_SUPPORT_AMOUNT)
        .accounts({
          redPacket: redPacketPDA,
          backer: backer2.publicKey,
          backerState: backer2StatePDA,
          solVault: solVaultPDA,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([backer2])
        .rpc();

      // 验证状态
      const redPacketAccount = await program.account.redPacket.fetch(
        redPacketPDA
      );
      const expectedRaised = LARGE_SUPPORT_AMOUNT.add(SMALL_SUPPORT_AMOUNT);
      assert.ok(
        redPacketAccount.solRaised.eq(expectedRaised),
        "SOL raised should be updated correctly."
      );
      console.log(`Total SOL raised: ${redPacketAccount.solRaised.toString()}`);
    });

    it("Allows users to claim airdrops", async () => {
      const [airdropStatePDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("airdrop"),
          redPacketPDA.toBuffer(),
          airdropClaimer1.publicKey.toBuffer(),
        ],
        program.programId
      );

      await program.methods
        .claimAirdrop()
        .accounts({
          redPacket: redPacketPDA,
          claimer: airdropClaimer1.publicKey,
          airdropState: airdropStatePDA,
          tokenVault: tokenVaultPDA,
          claimerAta: claimer1TokenAccount,
          mint: mint,
          creator: creator.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([airdropClaimer1])
        .rpc();

      // 验证状态
      const redPacketAccount = await program.account.redPacket.fetch(
        redPacketPDA
      );
      assert.equal(
        redPacketAccount.airdropClaimed,
        1,
        "Airdrop claimed count should be 1."
      );

      const airdropStateAccount = await program.account.airdropState.fetch(
        airdropStatePDA
      );
      assert.isTrue(
        airdropStateAccount.claimed,
        "Airdrop state should be marked as claimed."
      );
      console.log("Airdrop claimed successfully.");
    });
  });

  // --- 3. 结算（成功路径） ---
  describe("3. Settlement (Success Path)", () => {
    it("Successfully settles a SUCCESSFUL crowdfund", async () => {
      // 注意：在真实世界中，我们需要等待 expiry_time 到达。
      // 在测试中，我们假设时间已到，直接调用结算。
      console.log("Waiting for crowdfund to expire...");
      await sleep(4000); // 等待4秒，确保超过3秒的有效期
      console.log("Simulating time has passed, settling crowdfund...");

      await program.methods
        .settleCrowdfunding()
        .accounts({
          redPacket: redPacketPDA,
          creator: creator.publicKey,
          solVault: solVaultPDA,
          tokenVault: tokenVaultPDA,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([creator])
        .rpc();

      // 验证状态
      const redPacketAccount = await program.account.redPacket.fetch(
        redPacketPDA
      );
      assert.isTrue(redPacketAccount.settled, "Red packet should be settled.");
      assert.isTrue(
        redPacketAccount.success,
        "Crowdfund should be successful."
      );
      assert.ok(
        redPacketAccount.devFundSolAmount.gtn(0),
        "Dev fund SOL should be allocated."
      );
      assert.ok(
        redPacketAccount.protocolFeeAmount.gtn(0),
        "Protocol fee SOL should be allocated."
      );
      console.log("Crowdfund settled successfully.");
    });

    it("Allows backers to claim their vested TOKENS after success", async () => {
      const [backer1StatePDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("backer_state"),
          redPacketPDA.toBuffer(),
          backer1.publicKey.toBuffer(),
        ],
        program.programId
      );
      await program.methods
        .claimTokens()
        .accounts({
          redPacket: redPacketPDA,
          claimer: backer1.publicKey,
          backerState: backer1StatePDA,
          tokenVault: tokenVaultPDA,
          claimerAta: backer1TokenAccount,
          mint: mint,
          creator: creator.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([backer1])
        .rpc();

      // 验证状态
      const backer1State = await program.account.backerState.fetch(
        backer1StatePDA
      );
      assert.ok(
        backer1State.claimedAmount.gtn(0),
        "Backer should have claimed some tokens."
      );
      console.log(`Backer 1 claimed ${backer1State.claimedAmount} tokens.`);
    });
    it.skip("Allows the creator to claim the dev fund SOL after success", async () => {
      // 暂时跳过这个测试，因为它需要模拟时间流逝
      const balanceBefore = await provider.connection.getBalance(
        creator.publicKey
      );

      await program.methods
        .claimDevFund()
        .accounts({
          redPacket: redPacketPDA,
          creator: creator.publicKey,
          solVault: solVaultPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const balanceAfter = await provider.connection.getBalance(
        creator.publicKey
      );
      assert.ok(
        balanceAfter > balanceBefore,
        "Creator's SOL balance should increase."
      );
      console.log("Creator claimed dev fund SOL.");
    });

    it("Allows the creator to distribute fees after success", async () => {
      const devWalletBalanceBefore = await provider.connection.getBalance(
        developerWallet.publicKey
      );

      await program.methods
        .distributeFees()
        .accounts({
          redPacket: redPacketPDA,
          creator: creator.publicKey,
          developerWallet: developerWallet.publicKey,
          solVault: solVaultPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const devWalletBalanceAfter = await provider.connection.getBalance(
        developerWallet.publicKey
      );
      assert.ok(
        devWalletBalanceAfter > devWalletBalanceBefore,
        "Developer wallet should receive fees."
      );

      const redPacketAccount = await program.account.redPacket.fetch(
        redPacketPDA
      );
      assert.isTrue(
        redPacketAccount.feesDistributed,
        "fees_distributed flag should be true."
      );
      console.log("Fees distributed successfully.");
    });
  });

  // --- 4. 结算（失败路径） ---
  describe("4. Settlement (Failure Path)", () => {
    // 为失败测试创建一个全新的环境
    const creatorFail = Keypair.generate();
    const backerFail = Keypair.generate();
    let mintFail: PublicKey;
    let creatorTokenAccountFail: PublicKey;
    let redPacketPDAFail: PublicKey;
    let solVaultPDAFail: PublicKey;
    let tokenVaultPDAFail: PublicKey;

    before(async () => {
      await provider.connection.requestAirdrop(
        creatorFail.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      await provider.connection.requestAirdrop(
        backerFail.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      await new Promise((resolve) => setTimeout(resolve, 1000));

      mintFail = await createMint(
        provider.connection,
        creatorFail,
        creatorFail.publicKey,
        null,
        9
      );
      creatorTokenAccountFail = await createAssociatedTokenAccount(
        provider.connection,
        creatorFail,
        mintFail,
        creatorFail.publicKey
      );
      await mintTo(
        provider.connection,
        creatorFail,
        mintFail,
        creatorTokenAccountFail,
        creatorFail.publicKey,
        MINT_TOTAL_SUPPLY.toNumber()
      );
    });

    it("Correctly handles a FAILED crowdfund and allows refunds", async () => {
      // 找到 PDAs
      [redPacketPDAFail] = PublicKey.findProgramAddressSync(
        [Buffer.from("red_packet"), creatorFail.publicKey.toBuffer()],
        program.programId
      );
      [solVaultPDAFail] = PublicKey.findProgramAddressSync(
        [Buffer.from("sol_vault"), redPacketPDAFail.toBuffer()],
        program.programId
      );

      [tokenVaultPDAFail] = PublicKey.findProgramAddressSync(
        [Buffer.from("token_vault"), redPacketPDAFail.toBuffer()],
        program.programId
      );

      // 创建一个注定会失败的项目 (高目标，低支持)
      const params = {
        mint: mintFail,
        totalAmount: MINT_TOTAL_SUPPLY,
        tokenName: "FAIL",
        tokenSymbol: "FAL",
        fundingGoal: new BN(10 * LAMPORTS_PER_SOL), // 目标很高
        allocations: [],
        airdropMaxCount: new anchor.BN(100),
        expiryDuration: new anchor.BN(1), // 立即过期
      };

      await program.methods
        .createCustomRedpacket(params)
        .accounts({
          creator: creatorFail.publicKey,
          redPacket: redPacketPDAFail,
          creatorTokenAccount: creatorTokenAccountFail,
          solVault: solVaultPDAFail,
          tokenVault: tokenVaultPDAFail,
          mint: mintFail,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([creatorFail])
        .rpc();

      // 用户支持一点点
      const [backerFailStatePDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("backer_state"),
          redPacketPDAFail.toBuffer(),
          backerFail.publicKey.toBuffer(),
        ],
        program.programId
      );
      await program.methods
        .supportCrowdfunding(SMALL_SUPPORT_AMOUNT)
        .accounts({
          redPacket: redPacketPDAFail,
          backer: backerFail.publicKey,
          backerState: backerFailStatePDA,
          solVault: solVaultPDAFail,
          creator: creatorFail.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([backerFail])
        .rpc();

      // 模拟等待
      await new Promise((resolve) => setTimeout(resolve, 2000));

      // 结算失败的项目
      await program.methods
        .settleCrowdfunding()
        .accounts({
          redPacket: redPacketPDAFail,
          creator: creatorFail.publicKey,
          solVault: solVaultPDAFail,
          tokenVault: tokenVaultPDAFail,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([creatorFail])
        .rpc();

      let redPacketAccount = await program.account.redPacket.fetch(
        redPacketPDAFail
      );
      assert.isTrue(redPacketAccount.settled, "Should be settled.");
      assert.isFalse(redPacketAccount.success, "Should be marked as failed.");
      console.log("Failed crowdfund settled correctly.");

      // Backer 进行退款
      const backerBalanceBefore = await provider.connection.getBalance(
        backerFail.publicKey
      );
      await program.methods
        .refund()
        .accounts({
          redPacket: redPacketPDAFail,
          backer: backerFail.publicKey,
          backerState: backerFailStatePDA,
          solVault: solVaultPDAFail,
          creator: creatorFail.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([backerFail])
        .rpc();

      const backerBalanceAfter = await provider.connection.getBalance(
        backerFail.publicKey
      );
      assert.ok(
        backerBalanceAfter > backerBalanceBefore,
        "Backer's balance should increase after refund."
      );
      console.log("Backer refunded successfully.");
    });
  });
});
