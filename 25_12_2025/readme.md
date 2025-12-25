# Total code explanation 
```
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_program,
};

// Entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // 1. Fetch Accounts
    let vault_authority = next_account_info(accounts_iter)?;
    let system_prog = next_account_info(accounts_iter)?;

    // 2. SECURITY GATE: Signer Check
    if !vault_authority.is_signer {
        msg!("Error: Vault authority must sign this transaction.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 3. SECURITY GATE: Writable Check
    if !vault_authority.is_writable {
        msg!("Error: Vault authority account must be writable.");
        return Err(ProgramError::InvalidAccountData);
    }

    // 4. SECURITY GATE: Ownership Check
    if vault_authority.owner != &system_program::ID {
        msg!("Error: Account is not owned by the System Program.");
        return Err(ProgramError::IncorrectProgramId);
    }

    // 5. SECURITY GATE: Program ID Check
    if system_prog.key != &system_program::ID {
        msg!("Error: Invalid System Program provided.");
        return Err(ProgramError::IncorrectProgramId);
    }

    // 6. Logic: Integer-only math (No f64)
    let lamports = vault_authority.lamports();
    let sol_whole = lamports / 1_000_000_000;
    let sol_fraction = lamports % 1_000_000_000;

    msg!("=== RWA Vault Verified ===");
    msg!("Authority: {}", vault_authority.key);
    msg!("Balance: {}.{:09} SOL", sol_whole, sol_fraction);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_security_gates() {
        let program_id = Pubkey::new_unique();
        let auth_key = Pubkey::new_unique();
        let mut lamports = 1_000_000_000;
        let mut data = vec![0; 0];
        let owner = system_program::ID;

        // Simulate an account that IS a signer and IS writable
        let account = AccountInfo::new(
            &auth_key,
            true,  // is_signer
            true,  // is_writable
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let system_prog_info = AccountInfo::new(
            &system_program::ID,
            false,
            false,
            &mut 0,
            &mut vec![0;0],
            &system_program::ID,
            true,
            Epoch::default()
        );

        let accounts = vec![account, system_prog_info];
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
    }
}

```