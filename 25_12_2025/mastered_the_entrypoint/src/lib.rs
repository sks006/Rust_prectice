// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 14 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 13 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 12 ------------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target ==system"}
//     };
//     Ok(())
// }
//----------------------------- 11 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())

// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 10 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 9 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64 /1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 8 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result=process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 7 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// }
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner=&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//                 let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 14 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 13 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 12 ------------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target ==system"}
//     };
//     Ok(())
// }
//----------------------------- 11 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())

// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 10 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 9 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64 /1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 8 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result=process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
// //----------------------------- 7 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//                 let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 7 ------------------------------

// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
// };
// entrypoint! {
//     process_instruction
// }
// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();
//     let target_account = next_account_info(accounts_iter)?;
//     let account_pubkey = target_account.key;
//     let lamports = target_account.lamports();
//     let sol_balance = (lamports as f64) / 1_000_000_000.0;
//     if target_account.owner == &system_program::ID {
//         msg! {
//             "target == system"
//         }
//     }
//     Ok(())
// }
// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 6 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 5 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 4 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports= target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 3 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports= target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system "}
//     };
//     Ok(())
// }
//----------------------------- 2 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 1 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     program_error::ProgramError,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         return Err{}
//     };
//     if target_account,is_signer==&system_program::ID{

//     }
//     Ok(())
// }




// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }








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
        
        // --- Account 1: User ---
        let auth_key = Pubkey::new_unique();
        let mut lamports = 1_000_000_000;
        let mut data = vec![0; 0];
        let owner = system_program::ID;

        let account = AccountInfo::new(
            &auth_key,
            true,  // is_signer
            true,  // is_writable
            &mut lamports, // Points to 'lamports' variable above
            &mut data,     // Points to 'data' variable above
            &owner,
            false,
            Epoch::default(),
        );

        // --- Account 2: System Program ---
        // RULE: We must bind these to variables so they live through the whole test
        let mut system_lamports = 0; 
        let mut system_data = vec![0; 0]; 

        let system_prog_info = AccountInfo::new(
            &system_program::ID,
            false,
            false,
            &mut system_lamports, // Now points to a long-lived variable
            &mut system_data,     // Now points to a long-lived variable
            &system_program::ID,
            true,
            Epoch::default()
        );

        // Now both 'account' and 'system_prog_info' point to valid variables
        let accounts = vec![account, system_prog_info];
        
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
    }
}