//! Instruction definitions for NyxPay program

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

/// NyxPay program instructions
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum NyxPayInstruction {
    /// Initialize a new NyxPay account
    ///
    /// Accounts:
    /// 0. `[signer, writable]` Payer account
    /// 1. `[signer, writable]` Account to initialize
    /// 2. `[]` System program
    InitializeAccount {
        /// Initial balance in lamports
        initial_balance: u64,
    },

    /// Execute a private transfer
    ///
    /// Accounts:
    /// 0. `[signer]` Transaction authority
    /// 1. `[writable]` Source account
    /// 2. `[writable]` Destination account
    /// 3. `[readable]` Proof account containing zero-knowledge proof
    Transfer {
        /// Serialized zero-knowledge proof
        proof: Vec<u8>,
        /// Public inputs to the proof
        public_inputs: Vec<u8>,
    },

    /// Create a stealth address
    ///
    /// Accounts:
    /// 0. `[signer]` Account authority
    /// 1. `[writable]` Stealth account to create
    /// 2. `[]` System program
    CreateStealthAddress {
        /// Public key of the stealth address
        stealth_pubkey: [u8; 32],
        /// Nonce used in stealth address generation
        nonce: [u8; 32],
    },

    /// Verify a selective disclosure proof
    ///
    /// Accounts:
    /// 0. `[readable]` Account to verify
    /// 1. `[readable]` Proof account containing disclosure proof
    VerifyDisclosureProof {
        /// Serialized disclosure proof
        disclosure_proof: Vec<u8>,
        /// Properties that should be disclosed
        disclosed_properties: Vec<u8>,
    },
}

impl NyxPayInstruction {
    /// Unpacks a byte buffer into a [NyxPayInstruction]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let initial_balance = u64::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::InitializeAccount { initial_balance }
            }
            1 => {
                // Deserialize proof and public inputs
                let mut cursor = rest;
                if cursor.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let proof_len = u32::from_le_bytes([
                    cursor[0], cursor[1], cursor[2], cursor[3]
                ]) as usize;
                cursor = &cursor[4..];
                if cursor.len() < proof_len {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let proof = cursor[..proof_len].to_vec();
                cursor = &cursor[proof_len..];
                if cursor.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let public_inputs_len = u32::from_le_bytes([
                    cursor[0], cursor[1], cursor[2], cursor[3]
                ]) as usize;
                cursor = &cursor[4..];
                if cursor.len() < public_inputs_len {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let public_inputs = cursor[..public_inputs_len].to_vec();
                Self::Transfer { proof, public_inputs }
            }
            2 => {
                if rest.len() < 64 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let mut stealth_pubkey = [0u8; 32];
                stealth_pubkey.copy_from_slice(&rest[..32]);
                let mut nonce = [0u8; 32];
                nonce.copy_from_slice(&rest[32..64]);
                Self::CreateStealthAddress { stealth_pubkey, nonce }
            }
            3 => {
                let mut cursor = rest;
                if cursor.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let disclosure_proof_len = u32::from_le_bytes([
                    cursor[0], cursor[1], cursor[2], cursor[3]
                ]) as usize;
                cursor = &cursor[4..];
                if cursor.len() < disclosure_proof_len {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let disclosure_proof = cursor[..disclosure_proof_len].to_vec();
                cursor = &cursor[disclosure_proof_len..];
                if cursor.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let disclosed_properties_len = u32::from_le_bytes([
                    cursor[0], cursor[1], cursor[2], cursor[3]
                ]) as usize;
                cursor = &cursor[4..];
                if cursor.len() < disclosed_properties_len {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let disclosed_properties = cursor[..disclosed_properties_len].to_vec();
                Self::VerifyDisclosureProof {
                    disclosure_proof,
                    disclosed_properties,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
