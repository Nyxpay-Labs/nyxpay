//! Account state definitions for NyxPay program

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Account state
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AccountState {
    /// Account not yet initialized
    Uninitialized,
    /// Account is active and can send/receive
    Active,
    /// Account is frozen (administrative action)
    Frozen,
    /// Account is closed
    Closed,
}

/// User account state
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserAccount {
    /// Account authority
    pub authority: Pubkey,
    /// Account balance in lamports
    pub balance: u64,
    /// Nonce for replay prevention
    pub nonce: u64,
    /// Account state
    pub state: AccountState,
    /// Associated stealth addresses
    pub stealth_addresses: Vec<Pubkey>,
}

impl UserAccount {
    /// Account data size
    pub const LEN: usize = 32 + 8 + 8 + 1 + 4 + (32 * 10); // Approximate size

    /// Create a new user account
    pub fn new(authority: Pubkey, initial_balance: u64) -> Self {
        Self {
            authority,
            balance: initial_balance,
            nonce: 0,
            state: AccountState::Active,
            stealth_addresses: Vec::new(),
        }
    }
}

/// Stealth address account
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct StealthAccount {
    /// Owner of the stealth address
    pub owner: Pubkey,
    /// The stealth address public key
    pub stealth_pubkey: [u8; 32],
    /// Nonce used in generation
    pub nonce: [u8; 32],
    /// Creation timestamp
    pub created_at: i64,
}

impl StealthAccount {
    /// Account data size
    pub const LEN: usize = 32 + 32 + 32 + 8;
}

/// Proof account for storing zero-knowledge proofs
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProofAccount {
    /// Serialized proof
    pub proof_data: Vec<u8>,
    /// Public inputs
    pub public_inputs: Vec<u8>,
    /// Type of proof
    pub proof_type: ProofType,
    /// Creation timestamp
    pub created_at: i64,
}

/// Proof type
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum ProofType {
    /// Transfer proof
    Transfer,
    /// Disclosure proof
    Disclosure,
}
