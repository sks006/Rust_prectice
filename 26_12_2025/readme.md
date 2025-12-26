# Total code explanation 
```
use solana_program::{
   account_info::{next_account_info, AccountInfo},
   entrypoint,
   entrypoint::ProgramResult,
   msg,
   program_error::ProgramError,
   pubkey::Pubkey,
   system_program,
};


entrypoint!(process_instruction);


pub fn process_instruction(
   program_id: &Pubkey,
   accounts: &[AccountInfo],
   _instruction_data: &[u8],
) -> ProgramResult {
   let accounts_iter = &mut accounts.iter();


   // --- RULE 1: Sequence Validation ---
   let target_account = next_account_info(accounts_iter)?;
   let authority = next_account_info(accounts_iter)?;


   // --- RULE 2: Security Gate - Ownership Check ---
   // If the account isn't owned by the System Program, it might be a fake RWA.
   if target_account.owner != &system_program::ID {
       msg!("Error: Target account is not owned by the System Program");
       return Err(ProgramError::IncorrectProgramId);
   }


   // --- RULE 3: Security Gate - Signer Check ---
   // Only the authorized key can trigger logic.
   if !authority.is_signer {
       msg!("Error: Authority must sign this transaction");
       return Err(ProgramError::MissingRequiredSignature);
   }


   // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---
   // HFT Rule: We stay in lamports (u64) to prevent non-deterministic rounding.
   let lamports = target_account.lamports();
  
   // Instead of balance / 10^9, we use checked division for safety
   let sol_whole_part = lamports
       .checked_div(1_000_000_000)
       .ok_or(ProgramError::ArithmeticOverflow)?;


   msg!("Simulation: Verified balance of {} SOL", sol_whole_part);


   Ok(())
}


#[cfg(test)]
mod test {
   use super::*;
   use solana_program::clock::Epoch;


   #[test]
   fn test_security_gates() {
       let program_id = Pubkey::new_unique();
      
       // --- Setup Target Account ---
       let target_key = Pubkey::new_unique();
       let mut target_lamports = 2_500_000_000; // 2.5 SOL
       let mut target_data = vec![0; 0];
      
       let target_info = AccountInfo::new(
           &target_key,
           false,
           true,
           &mut target_lamports,
           &mut target_data,
           &system_program::ID, // Matches Rule 2
           false,
           Epoch::default(),
       );


       // --- Setup Authority (Signer) ---
       let auth_key = Pubkey::new_unique();
       let mut auth_lamports = 0;
       let mut auth_data = vec![0; 0];


       let auth_info = AccountInfo::new(
           &auth_key,
           true, // Matches Rule 3 (is_signer = true)
           false,
           &mut auth_lamports,
           &mut auth_data,
           &system_program::ID,
           false,
           Epoch::default(),
       );


       let accounts = vec![target_info, auth_info];


       // Execute
       let result = process_instruction(&program_id, &accounts, &[]);
       assert!(result.is_ok());
   }
}


```