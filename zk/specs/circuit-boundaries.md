# Circuit Boundaries

This document defines the explicit boundaries between in-circuit logic, off-circuit logic, and on-chain verification in NyxPay's zero-knowledge system.

## Overview

The zero-knowledge system is divided into three layers:

1. **In-Circuit Logic**: Computations performed within the zero-knowledge circuit
2. **Off-Circuit Logic**: Computations performed outside the circuit (wallet, client)
3. **On-Chain Verification**: Verification and state transitions performed on-chain

## In-Circuit Logic

### Purpose

In-circuit logic proves that certain computations were performed correctly without revealing the inputs or intermediate values.

### What Belongs In-Circuit

1. **Balance Computations**: Proving that `b_s' = b_s - a` and `b_d' = b_d + a`
2. **Balance Preservation**: Proving that `b_s + b_d = b_s' + b_d'`
3. **Non-Negative Checks**: Proving that `a >= 0` and `b_s' >= 0`
4. **Hash Computations**: Proving correct hash function evaluation (e.g., stealth address derivation)
5. **Signature Verification**: Proving that a signature is valid (if done in-circuit)
6. **Arithmetic Operations**: All arithmetic needed to establish proof statements

### Constraints

In-circuit logic must:
- Be expressible in the circuit's constraint system
- Have bounded computational complexity
- Not reveal private information

### Examples

**Balance Preservation:**
```
In-circuit: Prove that b_s + b_d = (b_s - a) + (b_d + a)
```

**Non-Negative Transfer:**
```
In-circuit: Prove that a >= 0 using range checks
```

**Stealth Address Derivation:**
```
In-circuit: Prove that stealth_addr = H(pk_r || r || context)
```

## Off-Circuit Logic

### Purpose

Off-circuit logic prepares inputs for the circuit, generates proofs, and handles operations that are not suitable for in-circuit computation.

### What Belongs Off-Circuit

1. **Input Preparation**: Collecting and formatting inputs for the circuit
2. **Proof Generation**: Executing the proof generation algorithm
3. **Transaction Construction**: Building Solana transactions
4. **Key Management**: Managing private keys and key material
5. **Account State Queries**: Querying on-chain account states
6. **Complex Computations**: Operations that are too expensive for in-circuit computation
7. **User Interaction**: UI, user input, error handling

### Rationale

These operations are performed off-circuit because:
- They don't need to be proven (they're trusted or verifiable through other means)
- They're too expensive for in-circuit computation
- They require external data or interaction

### Examples

**Account State Query:**
```
Off-circuit: Query on-chain account balance and nonce
```

**Transaction Signing:**
```
Off-circuit: Sign transaction with user's private key
```

**Proof Serialization:**
```
Off-circuit: Serialize proof for on-chain submission
```

## On-Chain Verification

### Purpose

On-chain verification ensures that proofs are valid and that state transitions preserve protocol invariants.

### What Belongs On-Chain

1. **Proof Verification**: Verifying zero-knowledge proofs
2. **Public Input Validation**: Validating that public inputs match on-chain state
3. **State Transition Execution**: Updating account balances and nonces
4. **Constraint Checking**: Checking additional constraints not proven in-circuit
5. **Account State Management**: Managing account states (Active, Frozen, Closed)
6. **Double-Spend Prevention**: Maintaining spent-input registries

### Rationale

These operations are performed on-chain because:
- They need to be enforced by the protocol
- They require access to on-chain state
- They must be atomic with state transitions

### Examples

**Proof Verification:**
```
On-chain: Verify zero-knowledge proof using public inputs
```

**Balance Update:**
```
On-chain: Atomically update account balances
```

**Nonce Validation:**
```
On-chain: Check that account nonce matches expected value
```

## Boundary Decisions

### Balance Preservation

**Decision**: Proven in-circuit, verified on-chain

**Rationale**: 
- In-circuit: Proves that the computation is correct without revealing balances
- On-chain: Verifies the proof and executes the state transition

**Security**: The proof ensures balance preservation even if on-chain verification has bugs (defense-in-depth).

### Non-Negative Transfers

**Decision**: Proven in-circuit, checked on-chain

**Rationale**:
- In-circuit: Proves that amounts are non-negative without revealing them
- On-chain: Additional check for defense-in-depth

**Security**: Multiple layers of validation prevent negative transfers.

### Stealth Address Derivation

**Decision**: Proven in-circuit, validated on-chain

**Rationale**:
- In-circuit: Proves correct derivation without revealing the recipient's public key
- On-chain: Validates that the stealth address matches the transaction destination

**Security**: The proof ensures stealth address correctness.

### Account State Queries

**Decision**: Performed off-circuit

**Rationale**:
- Account states are public on-chain
- No need to prove queries in-circuit
- Queries are too expensive for in-circuit computation

**Security**: Account states are verifiable on-chain, so queries don't need to be proven.

### Transaction Signing

**Decision**: Performed off-circuit

**Rationale**:
- Signing requires the user's private key, which cannot be used in-circuit
- Signatures are verified by the Solana runtime, not in-circuit
- Signing is a one-time operation, not a repeated computation

**Security**: Signatures are verified by Solana's transaction validation, providing security.

## Security Implications

### In-Circuit Logic Security

- **Correctness**: Circuit must correctly encode protocol invariants
- **Completeness**: All necessary constraints must be included
- **Soundness**: Invalid computations must be rejected

### Off-Circuit Logic Security

- **Input Validation**: Off-circuit logic must validate inputs before proof generation
- **Key Security**: Private keys must be stored and used securely
- **Proof Generation**: Proof generation must be performed correctly

### On-Chain Verification Security

- **Proof Verification**: Verification must be performed correctly
- **State Transitions**: State transitions must be atomic
- **Constraint Checking**: Additional constraints must be checked

## Optimization Considerations

### Circuit Size

- Minimize in-circuit logic to reduce proof generation time
- Move non-critical computations off-circuit
- Optimize constraint systems for efficiency

### Verification Cost

- Minimize on-chain verification costs
- Optimize proof size for transaction limits
- Batch verifications when possible

### Proof Generation Time

- Optimize circuit for fast proof generation
- Parallelize proof generation when possible
- Cache intermediate computations

## Extension Mechanism

New proof types and circuits can be added by:

1. **Defining Circuit Logic**: Specify what computations belong in-circuit
2. **Defining Public Inputs**: Specify what information is revealed
3. **Implementing Proof Generation**: Implement off-circuit proof generation
4. **Implementing Verification**: Implement on-chain verification
5. **Updating Protocol**: Update protocol to support new proof types

See [../future/placeholder.md](../future/placeholder.md) for the formal extension mechanism.

## Testing Boundaries

Boundaries are tested through:

1. **Circuit Tests**: Verify that circuits correctly encode constraints
2. **Integration Tests**: Verify that off-circuit and on-chain logic work together
3. **End-to-End Tests**: Verify the complete flow from proof generation to state transition
4. **Boundary Tests**: Verify that boundaries are correctly enforced

## Documentation

All boundary decisions must be documented with:
- Rationale for the boundary placement
- Security implications
- Performance considerations
- Extension points for future changes
