# Verification Flow

This document describes the end-to-end lifecycle of zero-knowledge proof generation, submission, and verification in NyxPay.

## Overview

The verification flow consists of:
1. Proof generation (wallet)
2. Proof serialization (wallet)
3. Transaction construction (wallet)
4. Transaction submission (wallet → Solana)
5. On-chain verification (Solana program)
6. State transition (Solana program)

## Proof Generation

### Input Collection

The wallet collects the following inputs:

**Private Inputs:**
- Source account balance `b_s`
- Destination account balance `b_d`
- Transfer amount `a`
- Sender's secret key `sk_s`
- Account nonces `n_s`, `n_d`

**Public Inputs:**
- Source account address `S`
- Destination account address `D`
- Account nonces `n_s`, `n_d`

### Circuit Execution

The wallet executes the zero-knowledge circuit with:
1. Private inputs (witness values)
2. Public inputs (statement values)
3. Circuit parameters (constants, curve parameters)

The circuit generates:
- A proof `π` that demonstrates the statement
- Public outputs (if any)

### Proof Validation

Before serialization, the wallet validates:
1. The proof is well-formed
2. The proof can be verified with the public inputs
3. The proof demonstrates all required properties

## Proof Serialization

### Format

The proof is serialized into a binary format that includes:
- Proof data (circuit-specific format)
- Public inputs
- Metadata (proof type, version, etc.)

### Structure

```
[proof_type: u8]
[version: u8]
[public_inputs_len: u32]
[public_inputs: Vec<u8>]
[proof_data_len: u32]
[proof_data: Vec<u8>]
```

### Size Constraints

The serialized proof must fit within Solana's transaction size limits. Current constraints:
- Maximum transaction size: 1232 bytes
- Proof overhead: ~100 bytes
- Maximum proof size: ~1100 bytes (approximate)

## Transaction Construction

### Account Setup

The transaction includes:
- Source account (writable)
- Destination account (writable)
- Proof account (readable, contains serialized proof)
- Program account (NyxPay program)
- System program (if needed)

### Instruction Data

The instruction data includes:
- Instruction discriminator
- Serialized proof
- Public inputs (may be embedded in proof or separate)

### Signatures

The transaction is signed by:
- Transaction authority (sender)
- Fee payer

## Transaction Submission

### Network Propagation

The transaction is submitted to the Solana network through:
1. RPC endpoint
2. Transaction propagation
3. Validator inclusion

### Confirmation

The transaction is confirmed when:
1. It is included in a block
2. The block is finalized
3. The program execution succeeds

## On-Chain Verification

### Proof Deserialization

The Solana program:
1. Extracts the proof type and version
2. Deserializes the proof data
3. Extracts the public inputs
4. Validates the proof format

### Public Input Validation

The program validates that:
1. Public inputs match on-chain account states
2. Account addresses are valid
3. Account nonces match current nonces
4. Accounts are in valid states (Active, not Frozen/Closed)

### Proof Verification

The program executes the proof verification algorithm:

```rust
fn verify_proof(
    proof: &[u8],
    public_inputs: &[u8],
) -> Result<bool, ProgramError> {
    // Deserialize proof
    let deserialized_proof = deserialize_proof(proof)?;
    
    // Extract public inputs
    let inputs = extract_public_inputs(public_inputs)?;
    
    // Verify proof
    let is_valid = zk_verifier::verify(&deserialized_proof, &inputs)?;
    
    Ok(is_valid)
}
```

### Constraint Checking

After proof verification, the program checks additional constraints:
1. Balance preservation (if not proven in ZK)
2. Account state validity
3. Nonce validity
4. Double-spend prevention

## State Transition

### Pre-Transition Validation

Before executing state transitions, the program verifies:
1. Proof is valid
2. All constraints are satisfied
3. Accounts are in valid states
4. Nonces are valid

### Atomic Execution

State transitions are executed atomically:
1. Debit amount from source account
2. Credit amount to destination account
3. Increment account nonces
4. Update account states

### Error Handling

If any step fails:
1. All state changes are rolled back
2. Transaction fails
3. Error is returned to the client

## Failure Modes

### Proof Generation Failures

**Invalid Private Inputs:**
- Insufficient balance
- Invalid secret key
- Invalid nonce

**Mitigation:** Wallet validates inputs before proof generation.

### Serialization Failures

**Proof Too Large:**
- Proof exceeds transaction size limits

**Mitigation:** Optimize proof size or split into multiple transactions.

### Verification Failures

**Invalid Proof:**
- Proof does not verify
- Public inputs do not match

**Mitigation:** Transaction is rejected, no state changes occur.

**Constraint Violation:**
- Balance preservation fails
- Nonce mismatch
- Account state invalid

**Mitigation:** Transaction is rejected, error is returned.

### Network Failures

**Transaction Not Included:**
- Network congestion
- Fee too low

**Mitigation:** Retry with higher fee or wait for network conditions to improve.

## Verification Performance

### Proof Generation Time

- Target: < 5 seconds for typical transactions
- Optimization: Parallel proof generation, circuit optimization

### Proof Verification Time

- Target: < 100ms for on-chain verification
- Constraint: Must complete within Solana's transaction time limits

### Proof Size

- Target: < 1000 bytes
- Optimization: Proof compression, circuit optimization

## Security Considerations

1. **Proof Replay**: Nonces prevent proof replay attacks
2. **Proof Forging**: Cryptographic security prevents proof forgery
3. **Timing Attacks**: Verification time should be constant to prevent timing attacks
4. **Side-Channel Attacks**: Implementation must resist side-channel attacks

## Error Codes

The verification flow may return the following errors:

- `InvalidProof`: Proof verification failed
- `InvalidPublicInputs`: Public inputs are invalid or do not match on-chain state
- `ProofTooLarge`: Proof exceeds size limits
- `ConstraintViolation`: Additional constraints are violated
- `AccountStateInvalid`: Account is not in a valid state for the operation
- `NonceMismatch`: Account nonce does not match expected value

## Testing

The verification flow is tested through:
1. Unit tests for proof generation and verification
2. Integration tests for end-to-end flow
3. Property-based tests for invariants
4. Fuzz tests for error cases
