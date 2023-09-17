use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock},
};
use rand::Rng;
use solana_program::sysvar::Sysvar;

entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the program was loaded into
    accounts: &[AccountInfo], // Accounts provided to the program
    _instruction_data: &[u8], // Instruction data (not used in this example)
) -> ProgramResult {
    // Ensure that the program is invoked by the correct account
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Ensure that the account is owned by the program
    if account.owner != _program_id {
        msg!("Account is not owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the current timestamp from the Clock sysvar
    let clock_account = next_account_info(accounts_iter)?;
    let clock = Clock::from_account_info(clock_account)?;

    // Generate a random number between 0 and 100 based on the current timestamp
    let seed = clock.unix_timestamp as u64;
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=100);

    // Store the random number in the account's data field
    account.data[0] = random_number as u8;

    msg!("Generated random number: {}", random_number);

    Ok(())
}
