# Security Model

This document describes the threat model, security assumptions, and attack surface analysis for NyxPay.

## Threat Model

### Adversary Capabilities

We assume the following adversary capabilities:

1. **Network Adversary**: Can observe all on-chain transactions, including transaction data, account states, and timing information
2. **Passive Observer**: Can analyze transaction patterns, amounts, and metadata to infer relationships
3. **Active Attacker**: Can attempt to submit invalid transactions, replay transactions, or manipulate state
4. **Cryptographic Attacker**: Can attempt to break cryptographic primitives (hash functions, signature schemes, zero-knowledge proofs)
5. **Solana Validator**: We assume validators are honest but curious (they execute transactions correctly but may analyze them)

### Adversary Goals

An adversary may attempt to:
- Link transactions to user identities
- Determine payment amounts and recipients
- Correlate transactions across time
- Break privacy guarantees through metadata analysis
- Double-spend funds
- Create funds from nothing
- Deny service to legitimate users

## Trust Assumptions

### Cryptographic Assumptions

1. **Hash Function Security**: The hash function used for stealth address generation is collision-resistant and preimage-resistant
2. **Zero-Knowledge Proof Security**: The zero-knowledge proof system is sound (invalid proofs are rejected) and zero-knowledge (proofs reveal no information beyond validity)
3. **Signature Scheme Security**: Digital signatures are unforgeable under chosen-message attacks
4. **Random Number Generation**: Stealth address nonces are generated from a cryptographically secure random number generator

### System Assumptions

1. **Solana Runtime**: The Solana runtime correctly executes program instructions and maintains account state
2. **Program Correctness**: The NyxPay program correctly implements the protocol specification
3. **Wallet Security**: Users' private keys are stored securely and not compromised
4. **Network Honesty**: A majority of Solana validators are honest

## Attack Surface

### On-Chain Attacks

#### Double-Spend Attack

**Description**: An attacker attempts to spend the same input twice.

**Mitigation**: The NyxPay program maintains a spent-input registry and rejects transactions that reference already-spent inputs. Zero-knowledge proofs must demonstrate that inputs are unspent.

**Status**: Mitigated by on-chain state tracking and proof verification.

#### Balance Manipulation

**Description**: An attacker attempts to create funds from nothing or destroy funds.

**Mitigation**: Zero-knowledge proofs must demonstrate balance preservation. The program verifies that the sum of input balances equals the sum of output balances.

**Status**: Mitigated by proof verification.

#### Invalid Proof Submission

**Description**: An attacker submits a transaction with an invalid or malformed zero-knowledge proof.

**Mitigation**: The program performs rigorous proof verification. Invalid proofs are rejected and the transaction fails.

**Status**: Mitigated by proof verification logic.

#### Replay Attack

**Description**: An attacker replays a valid transaction to execute it multiple times.

**Mitigation**: Solana's transaction signature mechanism prevents replay attacks. Each transaction must have a unique signature.

**Status**: Mitigated by Solana's transaction model.

### Privacy Attacks

#### Transaction Linkability

**Description**: An adversary attempts to link multiple transactions to the same user.

**Mitigation**: Stealth addresses break linkability between transactions. Each payment uses a unique stealth address that cannot be linked to the recipient's public identity without the recipient's private key.

**Status**: Mitigated by stealth address design.

#### Metadata Analysis

**Description**: An adversary analyzes transaction metadata (timing, amounts, patterns) to infer relationships.

**Mitigation**: 
- Privacy scoring helps users understand metadata leakage
- Selective disclosure enables controlled information release
- Users can batch transactions to reduce timing correlation

**Limitations**: Complete metadata hiding is not possible on a public blockchain. NyxPay minimizes but does not eliminate metadata leakage.

**Status**: Partially mitigated. Complete mitigation is not possible without additional infrastructure.

#### Amount Inference

**Description**: An adversary attempts to determine payment amounts from transaction data.

**Mitigation**: Zero-knowledge proofs hide amounts within transactions. The on-chain state does not reveal individual transfer amounts.

**Status**: Mitigated by zero-knowledge proofs.

#### Timing Correlation

**Description**: An adversary correlates transactions based on timing patterns.

**Mitigation**: Users can introduce delays or batch transactions to reduce timing correlation. However, complete timing privacy is not possible on a public blockchain.

**Status**: Partially mitigated. Users must take additional precautions to minimize timing correlation.

### Cryptographic Attacks

#### Hash Function Collision

**Description**: An attacker finds a collision in the hash function used for stealth address generation.

**Mitigation**: NyxPay uses a cryptographically secure hash function (SHA-256) that is currently considered secure. If the hash function is broken, the protocol would need to be updated.

**Status**: Mitigated by use of secure hash function. Requires monitoring of cryptographic developments.

#### Zero-Knowledge Proof Forging

**Description**: An attacker forges a zero-knowledge proof for an invalid transaction.

**Mitigation**: The zero-knowledge proof system is designed to be sound. Invalid proofs are rejected with overwhelming probability.

**Status**: Mitigated by proof system design. Requires formal verification of proof circuits.

#### Signature Forgery

**Description**: An attacker forges a signature to authorize a transaction.

**Mitigation**: NyxPay uses Ed25519 signatures, which are currently considered secure. Signature verification is performed by the Solana runtime.

**Status**: Mitigated by use of secure signature scheme.

### Implementation Attacks

#### Program Bug Exploitation

**Description**: An attacker exploits a bug in the NyxPay program to violate protocol invariants.

**Mitigation**: 
- Comprehensive testing and code review
- Formal verification of critical components
- Security audits by independent auditors
- Defense-in-depth through multiple validation layers

**Status**: Mitigated through development practices. Requires ongoing vigilance.

#### Wallet Compromise

**Description**: An attacker compromises a user's wallet and steals private keys.

**Mitigation**: 
- Users must secure their private keys using best practices (hardware wallets, secure storage)
- NyxPay does not store private keys on-chain
- Wallet software should follow security best practices

**Status**: User responsibility. NyxPay cannot mitigate key compromise.

## Out-of-Scope Threats

The following threats are explicitly out of scope:

1. **Quantum Computing**: NyxPay does not provide post-quantum security guarantees. If quantum computers become capable of breaking current cryptographic primitives, the protocol would need to be updated.

2. **Network Partition**: NyxPay does not mitigate network partition attacks. If the Solana network is partitioned, privacy guarantees may be weakened.

3. **Validator Collusion**: If a majority of Solana validators collude to censor transactions or manipulate state, NyxPay cannot provide security guarantees.

4. **Side-Channel Attacks**: Attacks that exploit implementation details (timing, power consumption) are out of scope for the protocol specification, though implementations should mitigate them.

5. **Social Engineering**: Attacks that trick users into revealing private keys or authorizing malicious transactions are out of scope.

6. **Malicious Wallet Software**: If wallet software is malicious or compromised, security guarantees do not apply. Users must trust their wallet software.

## Security Properties

NyxPay provides the following security properties:

1. **Balance Preservation**: Funds cannot be created or destroyed through protocol operations
2. **Double-Spend Prevention**: Each input can only be spent once
3. **Transaction Privacy**: Transaction amounts and recipients are hidden from observers
4. **Unlinkability**: Transactions cannot be linked to user identities without private keys
5. **Selective Disclosure**: Users can prove specific transaction properties without revealing other information

## Audit Requirements

All security-critical components must undergo independent security audits:

1. **Solana Program**: The on-chain program must be audited for correctness and security vulnerabilities
2. **Zero-Knowledge Circuits**: Proof circuits must be audited for soundness and zero-knowledge properties
3. **Cryptographic Primitives**: All cryptographic operations must be reviewed for correctness
4. **Wallet Implementation**: Wallet software must be audited for key management and transaction construction

See [audits/README.md](./audits/README.md) for detailed audit requirements.

## Responsible Disclosure

Security vulnerabilities should be reported to the NyxPay security team through the process described in [CONTRIBUTING.md](./CONTRIBUTING.md). Critical vulnerabilities will be addressed promptly and disclosed responsibly.

## Security Updates

Security updates will be documented in [CHANGELOG.md](./CHANGELOG.md) and communicated through appropriate channels. Users should keep their wallet software and on-chain programs updated.
