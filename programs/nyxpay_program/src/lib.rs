//! NyxPay Solana Program
//!
//! This program implements the on-chain enforcement layer for NyxPay,
//! a privacy-focused payment protocol on Solana.

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

use processor::Processor;

entrypoint!(process_instruction);

/// Program entrypoint
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("NyxPay program entrypoint");
    Processor::process(program_id, accounts, instruction_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entrypoint() {
        // Placeholder test
        assert!(true);
    }
}
