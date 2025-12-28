
```
use anchor_lang::prelude::*;


declare_id!("6h5A111111111111111111111111111111111111111");


#[program]
pub mod week_five {
   use super::*;
   pub fn initialize(ctx: Context<Initialize>, asset_id: u64) -> Result<()> {
       let vault = &mut ctx.accounts.vault_account;
       vault.asset_id = asset_id; // Persistence rule
       Ok(())
   }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
   // 8 bytes discriminator + 8 bytes u64
   #[account(init, payer = user, space = 8 + 8)]
   pub vault_account: Account<'info, VaultAccount>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub system_program: Program<'info, System>,
}


#[account]
pub struct VaultAccount {
   pub asset_id: u64,
}


// --- RUST NATIVE TEST CASE (Week 5) ---
#[cfg(test)]
mod tests {
   use super::*;
   use anchor_lang::InstructionData;
   use solana_program_test::*;      
   use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};


   #[tokio::test]
   async fn test_init_success() {
       let program_id = id();
       let mut program_test = ProgramTest::new("week_five", program_id, None);
      
       let vault_kp = Keypair::new(); // Account to be created
       let (mut banks, payer, hash) = program_test.start().await;


       let ix = solana_sdk::instruction::Instruction {
           program_id,
           accounts: anchor_lang::ToAccountMetas::to_account_metas(
               &Initialize {
                   vault_account: vault_kp.pubkey(),
                   user: payer.pubkey(),
                   system_program: solana_program::system_program::ID,
               },
               None,
           ),
           data: week_five::instruction::Initialize { asset_id: 1337 }.data(),
       };


       let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
       tx.sign(&[&payer, &vault_kp], hash); // Both must sign (Payer and the new account)
      
       // Assert: Creation must succeed
       banks.process_transaction(tx).await.unwrap();


       // Verify Data: Fetch account from the mock ledger
       let account = banks.get_account(vault_kp.pubkey()).await.unwrap().unwrap();
       let vault_data = VaultAccount::try_deserialize(&mut &account.data[..]).unwrap();
       assert_eq!(vault_data.asset_id, 1337);
   }
}
```