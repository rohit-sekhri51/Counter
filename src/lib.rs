use borsh::{BorshDeserialize, BorshSerialize};
// use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod instructions;

use crate::instructions::CounterInstructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {     // Stateless in Solana, so no need to store the owner
    pub counter: u32,           // Counter value is stored in the different account / address / [AccountInfo] ???
} 


entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],   // all the accounts passed from wallet stored in [AccountInfo]
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    // Increment, Decrement, Update, Reset from browser
    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;     

    // account is from wallet or CounterAccount ???
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    
    // counter_account is from blockchain
    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstructions::Decrement(args) => {
            if args.value <= counter_account.counter {
                counter_account.counter -= args.value;
            }
            else {
                counter_account.counter = 0;
            }
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    // Initally unpack() deserialized the instructions_data into human readable format
    // account.data.borrow_mut() is the account data from wallet
    // Now, need to serialize the counter_account back to the account
    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

// cargo test

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use solana_sdk::account;
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

       let account = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
       );

       let accounts = vec![account];

       let mut increment_instruction_data: Vec<u8> = vec![0];
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        let inc_value = 7u32;
        increment_instruction_data.extend_from_slice(&inc_value.to_le_bytes());     
        // 7u32.to_le_bytes() = [7, 0, 0, 0]

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            7
        ); 

        let dec_value = 12u32;
        decrement_instruction_data.extend_from_slice(&dec_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        ); 

        let update_value = 51251u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        // 51251u32.to_le_bytes() = [131, 200, 0, 0]

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            51251
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        ); 

    }

}