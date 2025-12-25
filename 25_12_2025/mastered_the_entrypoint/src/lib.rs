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
use solana_program::{
    account_info::{
        next_account_info,AccountInfo
    },
    pubkey::Pubkey,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    system_program
};
entrypoint!{process_instruction}

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    _instruction_data:&[u8]
)->ProgramResult{
    let accounts_iter=&mut accounts.iter();
    let target_account=next_account_info(accounts_iter)?;
    let account_pubkey=target_account.key;
    let lamports=target_account.lamports();
    let sol_balance=lamports as f64/1_000_000_000.0;
    if target_account.owner==&system_program::ID{
        return Err{}
    };
    if target_account,is_signer==&system_program::ID{

    }
    Ok(())
}




#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    #[test]
    fn test_hello_balance() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 2_500_000_000;
        let mut data = vec![0;0];
        let owner = system_program::ID;
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );
        let accounts = vec![account];
        println!("\n--- STARTING SIMULATION ---");
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
        println!("--- SIMULATION SUCCESSFUL ---\n");
    }
}

// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     program_error::ProgramError,
//     msg,
//     system_program,
// };

// // 1. THE ENTRYPOINT (Your "Front Door")
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> ProgramResult {
    
//     // 2. ACCOUNT ITERATION
//     let accounts_iter = &mut accounts.iter();
//     let target_account = next_account_info(accounts_iter)?;

//     // --- GATE 1: THE SIGNER RULE ---
//     // Rule: Never trust an account's identity unless it has signed.
//     // HINT: Use an 'if' statement to check 'target_account.is_signer'.
//     // Return: ProgramError::MissingRequiredSignature
//     /* TODO: Implement Signer Check here */

//     // --- GATE 2: THE OWNERSHIP RULE ---
//     // Rule: Ensure the account is actually owned by the expected program.
//     // HINT: Compare 'target_account.owner' with '&system_program::ID'.
//     // Return: ProgramError::IncorrectProgramId
//     /* TODO: Implement Ownership Check here */

//     // --- GATE 3: THE WRITABLE RULE ---
//     // Rule: If you plan to change data later, the account must be writable.
//     // HINT: Check 'target_account.is_writable'.
//     // Return: ProgramError::InvalidAccountData
//     /* TODO: Implement Writable Check here */

//     // 3. LOGIC & MATH (The HFT Performance Path)
//     // Rule: NO FLOATS. Use integer scaling for 2025 consensus stability.
//     let lamports = target_account.lamports();
    
//     // Instead of f64, keep it as u64.
//     // Logic: In HFT, we represent 1 SOL as 1,000,000,000 u64.
//     msg!("RWA Vault Balance: {} lamports", lamports);

//     Ok(())
// }