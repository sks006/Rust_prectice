fn main() {
    println!("Hello, world!");
}
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_program,
};

// Entrypoint declaration
entrypoint!(process_instruction);

// Main function to practice
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    msg!("=== HELLO BALANCE PROGRAM ===");
    msg!("Program ID: {}", program_id);
    msg!("Instruction data length: {} bytes", instruction_data.len());
    
    // Get account iterator
    let accounts_iter = &mut accounts.iter();
    
    // Try to get first account (returns error if no accounts)
    let account = match accounts_iter.next() {
        Some(acc) => acc,
        None => {
            msg!("ERROR: No accounts provided");
            return Err(ProgramError::NotEnoughAccountKeys);
        }
    };
    
    // Check if account is signer and writable
    msg!("--- Account Analysis ---");
    msg!("Pubkey: {}", account.key);
    msg!("Owner: {}", account.owner);
    msg!("Lamports: {}", account.lamports());
    msg!("Executable: {}", account.executable);
    msg!("Is Signer: {}", account.is_signer);
    msg!("Is Writable: {}", account.is_writable);
    
    // Convert lamports to SOL
    let sol_balance = account.lamports() as f64 / 1_000_000_000.0;
    msg!("Balance in SOL: {:.6}", sol_balance);
    
    // Conditional check for transfer capability
    if account.is_signer && account.is_writable {
        msg!("✅ Status: CAN TRANSFER FUNDS");
        
        // Check for minimum rent-exempt balance
        const MIN_RENT_EXEMPT: u64 = 890_880;
        if account.lamports() >= MIN_RENT_EXEMPT {
            let available = account.lamports() - MIN_RENT_EXEMPT;
            msg!("   Available after rent: {} lamports", available);
            msg!("   Available SOL: {:.6}", available as f64 / 1_000_000_000.0);
        } else {
            msg!("   ⚠️ Warning: Below rent-exempt minimum");
        }
    } else {
        msg!("❌ Status: CANNOT TRANSFER FUNDS");
        
        if !account.is_signer {
            msg!("   Reason: Not a signer (no authority)");
        }
        if !account.is_writable {
            msg!("   Reason: Not writable (read-only)");
        }
    }
    
    // Process instruction data if present
    if !instruction_data.is_empty() {
        msg!("--- Instruction Data ---");
        msg!("Raw: {:?}", instruction_data);
        
        // Try to parse as string
        if let Ok(text) = std::str::from_utf8(instruction_data) {
            msg!("As text: {}", text);
        }
        
        // Try to parse as u64
        if instruction_data.len() >= 8 {
            let bytes: [u8; 8] = instruction_data[0..8].try_into().unwrap();
            let value = u64::from_le_bytes(bytes);
            msg!("First 8 bytes as u64: {}", value);
        }
    }
    
    msg!("=== END ===");
    Ok(())
}

// SIMULATION CODE - To see output without deploying
// This is for practice only - not part of the Solana program
#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{clock::Epoch, rent::Rent, sysvar::Sysvar};
    
    // Helper to create mock account
    fn create_mock_account(lamports: u64, is_signer: bool, is_writable: bool) -> AccountInfo<'static> {
        let key = Pubkey::new_unique();
        let mut lamports_data = lamports;
        let mut data = vec![];
        let owner = system_program::id();
        let executable = false;
        let rent_epoch = Epoch::default();
        
        AccountInfo::new(
            &key,
            is_signer,
            is_writable,
            &mut lamports_data,
            &mut data,
            &owner,
            executable,
            rent_epoch,
        )
    }
    
    #[test]
    fn test_balance_output() {
        println!("\n=== SIMULATED OUTPUT ===");
        
        // Mock program ID
        let program_id = Pubkey::new_unique();
        
        // Create mock accounts
        let account1 = create_mock_account(1_500_000_000, true, true); // 1.5 SOL, signer+writable
        let account2 = create_mock_account(500_000_000, false, true);  // 0.5 SOL, writable only
        let accounts = vec![account1, account2];
        
        // Mock instruction data
        let instruction_data = b"check_balance";
        
        // Call the function
        let result = process_instruction(&program_id, &accounts, instruction_data);
        
        println!("Program Result: {:?}", result);
        println!("=== END SIMULATION ===\n");
    }
}

// Main function to run the simulation
fn main() {
    println!("Running Hello Balance simulation...");
    
    // Call the test to see output
    tests::test_balance_output();
    
    println!("\nPractice the process_instruction signature:");
    println!("pub fn process_instruction(");
    println!("    program_id: &Pubkey,");
    println!("    accounts: &[AccountInfo],");
    println!("    instruction_data: &[u8],");
    println!(") -> ProgramResult {{");
    println!("    // Your code here");
    println!("    Ok(())");
    println!("}}");
}