//! Error types for NyxPay program

use solana_program::program_error::ProgramError;
use thiserror::Error;

/// NyxPay program errors
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum NyxPayError {
    #[error("Invalid zero-knowledge proof")]
    InvalidProof,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid account")]
    InvalidAccount,

    #[error("Balance preservation violation")]
    BalanceViolation,

    #[error("Negative transfer amount")]
    NegativeTransfer,

    #[error("Invalid stealth address")]
    InvalidStealthAddress,

    #[error("Invalid disclosure proof")]
    InvalidDisclosureProof,

    #[error("Disclosed properties do not match requested properties")]
    PropertyMismatch,

    #[error("Account is frozen")]
    AccountFrozen,

    #[error("Account is closed")]
    AccountClosed,

    #[error("Invalid account nonce")]
    InvalidNonce,

    #[error("Invalid instruction data")]
    InvalidInstruction,

    #[error("Account not initialized")]
    NotInitialized,

    #[error("Account already initialized")]
    AlreadyInitialized,
}

impl From<NyxPayError> for ProgramError {
    fn from(e: NyxPayError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
