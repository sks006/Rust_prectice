import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorBoilerplate } from "../target/types/anchor_boilerplate";
import { 
  createMint, 
  getOrCreateAssociatedTokenAccount, 
  mintTo, 
  TOKEN_PROGRAM_ID 
} from "@solana/spl-token";
import { expect } from "chai";

describe("Lending Protocol: Week 6 Genesis", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
  const admin = anchor.web3.Keypair.generate();
  
  let mint: anchor.web3.PublicKey;
  let userTokenAccount: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;

  before(async () => {
    // 1. FUND ADMIN
    const sig = await provider.connection.requestAirdrop(admin.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(sig);

    // 2. CREATE MINT (Simulating USDC)
    mint = await createMint(provider.connection, admin, admin.publicKey, null, 6);

    // 3. DERIVE VAULT PDA (Must match Rust seeds)
    [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool_vault"), mint.toBuffer()],
      program.programId
    );

    // 4. SETUP USER ATA
    const ata = await getOrCreateAssociatedTokenAccount(provider.connection, admin, mint, admin.publicKey);
    userTokenAccount = ata.address;
    await mintTo(provider.connection, admin, mint, userTokenAccount, admin, 1000_000_000);
  });

  it("Initializes the Global Vault", async () => {
    await program.methods
      .initialize()
      .accounts({
        admin: admin.publicKey,
        mint: mint,
        vault: vaultPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([admin])
      .rpc();

    const vaultAccount = await provider.connection.getAccountInfo(vaultPda);
    expect(vaultAccount).to.not.be.null;
  });

  it("Executes a Secure Deposit", async () => {
    const depositAmount = new anchor.BN(500_000_000); // 500 Tokens

    const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("obligation"), admin.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .deposit(depositAmount)
      .accounts({
        user: admin.publicKey,
        mint: mint,
        userTokenAccount: userTokenAccount,
        vault: vaultPda,
        obligation: obligationPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    // Verification
    const state = await program.account.userObligation.fetch(obligationPda);
    expect(state.deposited.toNumber()).to.equal(depositAmount.toNumber());
  });
});