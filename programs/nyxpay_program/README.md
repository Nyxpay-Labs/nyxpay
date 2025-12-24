# NyxPay Solana Program

The NyxPay on-chain program is the canonical enforcement layer for the NyxPay protocol. It verifies zero-knowledge proofs, validates account states, and executes state transitions atomically.

## Architecture

The program operates as a stateless verifier that:
1. Receives transactions with embedded zero-knowledge proofs
2. Verifies proof validity
3. Validates account states and constraints
4. Executes state transitions if all checks pass

## Instruction Set

### InitializeAccount

Initializes a new NyxPay account for a user.

**Accounts:**
- `payer`: The account paying for initialization (signer, writable)
- `account`: The account to initialize (signer, writable)
- `system_program`: System program account

**Parameters:**
- `initial_balance`: Initial balance in lamports

**Validation:**
- Account must not already be initialized
- Initial balance must be non-negative
- Account must be a signer

**State Transition:**
- Creates account with specified initial balance
- Sets account state to `Active`

### Transfer

Executes a private transfer between accounts.

**Accounts:**
- `authority`: Transaction authority (signer)
- `source`: Source account (writable)
- `destination`: Destination account (writable)
- `proof_account`: Account containing the zero-knowledge proof (readable)

**Parameters:**
- `proof`: Serialized zero-knowledge proof
- `public_inputs`: Public inputs to the proof

**Validation:**
1. Verify zero-knowledge proof using public inputs
2. Verify source account has sufficient balance
3. Verify destination account exists and is active
4. Verify balance preservation (sum of inputs = sum of outputs)
5. Verify non-negative transfers
6. Verify stealth address correctness (if applicable)

**State Transition:**
- Debits amount from source account
- Credits amount to destination account
- Updates account nonces to prevent replay

**Errors:**
- `InvalidProof`: Zero-knowledge proof verification failed
- `InsufficientBalance`: Source account has insufficient funds
- `InvalidAccount`: Destination account is invalid or inactive
- `BalanceViolation`: Balance preservation check failed
- `NegativeTransfer`: Transfer amount is negative

### CreateStealthAddress

Creates a stealth address for receiving payments.

**Accounts:**
- `authority`: Account authority (signer)
- `stealth_account`: Account to store stealth address (writable)
- `system_program`: System program account

**Parameters:**
- `stealth_pubkey`: Public key of the stealth address
- `nonce`: Nonce used in stealth address generation

**Validation:**
- Stealth address must be correctly derived from authority's public key and nonce
- Stealth account must not already exist

**State Transition:**
- Creates stealth account
- Stores stealth address metadata

### VerifyDisclosureProof

Verifies a selective disclosure proof for compliance or auditing.

**Accounts:**
- `account`: Account to verify (readable)
- `proof_account`: Account containing disclosure proof (readable)

**Parameters:**
- `disclosure_proof`: Serialized disclosure proof
- `disclosed_properties`: Properties that should be disclosed

**Validation:**
- Verify disclosure proof validity
- Verify disclosed properties match requested properties

**State Transition:**
- None (read-only operation)

**Errors:**
- `InvalidDisclosureProof`: Disclosure proof verification failed
- `PropertyMismatch`: Disclosed properties do not match requested properties

## Account Models

### UserAccount

Represents a user's NyxPay account.

```rust
pub struct UserAccount {
    pub authority: Pubkey,           // Account authority
    pub balance: u64,                 // Account balance in lamports
    pub nonce: u64,                   // Nonce for replay prevention
    pub state: AccountState,          // Account state
    pub stealth_addresses: Vec<Pubkey>, // Associated stealth addresses
}
```

**Account State:**
- `Uninitialized`: Account not yet initialized
- `Active`: Account is active and can send/receive
- `Frozen`: Account is frozen (administrative action)
- `Closed`: Account is closed

### StealthAccount

Represents a stealth address.

```rust
pub struct StealthAccount {
    pub owner: Pubkey,                // Owner of the stealth address
    pub stealth_pubkey: Pubkey,       // The stealth address public key
    pub nonce: [u8; 32],              // Nonce used in generation
    pub created_at: i64,              // Creation timestamp
}
```

### ProofAccount

Temporary account used to store zero-knowledge proofs for verification.

```rust
pub struct ProofAccount {
    pub proof_data: Vec<u8>,          // Serialized proof
    pub public_inputs: Vec<u8>,       // Public inputs
    pub proof_type: ProofType,        // Type of proof
    pub created_at: i64,              // Creation timestamp
}
```

## State Transitions

### Transfer State Transition

Given:
- Source account `S` with balance `b_s`
- Destination account `D` with balance `b_d`
- Transfer amount `a`

The state transition is:
1. Verify `b_s >= a`
2. Verify zero-knowledge proof demonstrates:
   - Ownership of `S`
   - Balance preservation: `b_s + b_d = (b_s - a) + (b_d + a)`
   - Non-negative transfer: `a >= 0`
3. Update balances:
   - `S.balance = b_s - a`
   - `D.balance = b_d + a`
4. Increment nonces:
   - `S.nonce = S.nonce + 1`
   - `D.nonce = D.nonce + 1`

### Validation Logic

All state transitions must satisfy:

1. **Balance Preservation**: The sum of all account balances before and after a transaction must be equal
2. **Non-Negative Balances**: No account balance may become negative
3. **Proof Validity**: All state transitions must be accompanied by valid zero-knowledge proofs
4. **Double-Spend Prevention**: Each account nonce must be strictly increasing
5. **Account Validity**: All accounts must be in `Active` state

## Zero-Knowledge Proof Verification

The program verifies zero-knowledge proofs on-chain. The verification process:

1. **Deserialize Proof**: Parse the serialized proof from the proof account
2. **Extract Public Inputs**: Extract public inputs (account addresses, amounts, etc.)
3. **Verify Proof**: Call the proof verification function with the proof and public inputs
4. **Check Constraints**: Verify that the proof demonstrates all required properties

### Proof Verification Interface

```rust
pub fn verify_proof(
    proof: &[u8],
    public_inputs: &[u8],
) -> Result<(), ProgramError> {
    // Deserialize proof
    // Verify proof using cryptographic library
    // Return error if verification fails
}
```

### Public Inputs

Public inputs to zero-knowledge proofs include:
- Source account address
- Destination account address
- Transaction amount (may be hidden in some proofs)
- Account nonces
- Merkle roots (if using Merkle trees for account state)

## Error Handling

The program uses a custom error type:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NyxPayError {
    InvalidProof,
    InsufficientBalance,
    InvalidAccount,
    BalanceViolation,
    NegativeTransfer,
    InvalidStealthAddress,
    InvalidDisclosureProof,
    PropertyMismatch,
    AccountFrozen,
    AccountClosed,
    InvalidNonce,
}
```

All errors are returned as `ProgramError` and can be handled by clients.

## Security Considerations

1. **Proof Verification**: Proof verification must be performed before any state transitions
2. **Atomicity**: All state transitions are atomic (all succeed or all fail)
3. **Replay Prevention**: Nonces prevent transaction replay
4. **Balance Checks**: Balance checks must occur before state updates
5. **Account State**: Account state must be checked before operations

## Testing

The program includes comprehensive tests:

- Unit tests for instruction handlers
- Integration tests for state transitions
- Property-based tests for invariants
- Fuzz tests for error cases

Run tests with:
```bash
cargo test-sbf
```

## Deployment

The program is deployed to Solana mainnet, devnet, and testnet. Program IDs:

- Mainnet: `[To be deployed]`
- Devnet: `[To be deployed]`
- Testnet: `[To be deployed]`

## Interaction with ZK Layer

The program interfaces with the zero-knowledge proof system through:

1. **Proof Serialization**: Proofs are serialized by the wallet and included in transactions
2. **Proof Verification**: The program deserializes and verifies proofs on-chain
3. **Public Inputs**: Public inputs are extracted from account states and transaction data

See [../zk/README.md](../zk/README.md) for details on the zero-knowledge proof system.
