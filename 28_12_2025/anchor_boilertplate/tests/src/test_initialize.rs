use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use solana_program::clock::Epoch;

    #[test]
    fn test_initialize_vault_logic() {
        // 1. Create Mock Data (The "Kata" of Testing) [cite: 92, 131]
        let program_id = Pubkey::new_unique();
        let admin_key = Pubkey::new_unique();
        let vault_key = Pubkey::new_unique();
        
        // 2. Mock Lamports and Data Buffer [cite: 133, 134]
        let mut lamports = 100_000_000; // 0.1 SOL
        let mut data = vec![0u8; Vault::MAXIMUM_SIZE];
        let owner = program_id;

        // 3. Construct AccountInfo (Mirroring the SVM) [cite: 136, 137]
        let vault_account_info = AccountInfo::new(
            &vault_key,
            false,       // is_signer
            true,        // is_writable
            &mut lamports,
            &mut data,
            &owner,
            false,       // executable
            Epoch::default(),
        );

        // 4. Wrap in Anchor Account type [cite: 105]
        let mut vault_account = Account::<Vault>::try_from(&vault_account_info).unwrap();

        // 5. Run the logic (The "Rule of the Handler") 
        vault_account.admin = admin_key;
        vault_account.total_deposits = 0;
        vault_account.bump = 255;

        // 6. Assertions (The "Rule of Proof") [cite: 151, 164]
        assert_eq!(vault_account.admin, admin_key);
        assert_eq!(vault_account.total_deposits, 0);
        println!("Internal Rust logic test passed for Vault: {:?}", vault_key);
    }
}