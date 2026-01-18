# NyxPay

**NyxPay** is a privacy-focused smart wallet and payment protocol on Solana that enables private transactions through stealth addresses, zero-knowledge proofs, and selective disclosure mechanisms.

## Overview

NyxPay provides cryptographic privacy guarantees for Solana transactions while maintaining compatibility with the Solana runtime. The protocol operates through three integrated components:

1. **Wallet Layer**: Client-side wallet that generates stealth addresses, constructs private transactions, and produces zero-knowledge proofs
2. **Zero-Knowledge Layer**: Cryptographic proofs that demonstrate transaction validity without revealing sensitive information
3. **On-Chain Program**: Solana program that verifies proofs and executes state transitions

## Privacy Model

NyxPay implements a privacy model based on stealth addresses and zero-knowledge proofs. Unlike mixing protocols that rely on anonymity sets, NyxPay provides cryptographic privacy guarantees at the transaction level.

### Stealth Addresses

Each payment recipient generates a stealth address from their public key and a random nonce. The stealth address appears on-chain as the transaction recipient, but only the recipient can link it to their identity using their private key. This breaks the linkability between a recipient's public identity and their on-chain payment addresses.

### Zero-Knowledge Proofs

NyxPay uses zero-knowledge proofs to demonstrate:
- The sender owns sufficient funds to complete the transaction
- The transaction preserves total balance (no funds are created or destroyed)
- All transfers are non-negative
- The stealth address is correctly derived
- The transaction does not constitute a double-spend

These proofs are verified on-chain before state transitions occur.

### Selective Disclosure

Users can generate disclosure proofs that reveal specific transaction properties (e.g., payment amount, timestamp) to third parties without revealing other sensitive information. This enables compliance and auditing while preserving privacy.

### Privacy Scoring

Each wallet maintains a privacy score that quantifies the privacy properties of its transaction history. The score accounts for:
- Linkability between transactions
- Metadata leakage (timing, amounts, patterns)
- Stealth address usage
- Zero-knowledge proof coverage

Higher scores indicate stronger privacy guarantees. The score is computed locally and never stored on-chain.

## Architecture

```
┌───────────────────────────────────────────────────────────┐
│                        NyxPay Wallet                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Stealth     │  │  ZK Proof    │  │  Transaction │     │
│  │  Address     │  │  Generation  │  │  Builder     │     │
│  │  Generator   │  │              │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└───────────────────────────────────────────────────────────┘
                            │
                            │ Transaction + ZK Proof
                            ▼
┌───────────────────────────────────────────────────────────┐
│                    Solana Validator                       │
│  ┌────────────────────────────────────────────────────┐   │
│  │            NyxPay Program                          │   │
│  │  ┌──────────────┐  ┌──────────────┐                │   │
│  │  │  ZK Proof    │  │  State       │                │   │
│  │  │  Verifier    │  │  Transition  │                │   │
│  │  └──────────────┘  └──────────────┘                │   │
│  └────────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────────┘
```

### Wallet Layer

The wallet layer is responsible for:
- Generating and managing stealth addresses
- Constructing private transactions
- Generating zero-knowledge proofs for transaction validity
- Computing privacy scores
- Managing key material securely

### Zero-Knowledge Layer

The ZK layer defines:
- Proof statements and their cryptographic properties
- Circuit boundaries and verification logic
- Proof serialization and deserialization
- Verification failure modes

### On-Chain Program

The Solana program:
- Verifies zero-knowledge proofs
- Validates account states
- Executes state transitions atomically
- Enforces protocol invariants

## Stealth Address Lifecycle

1. **Recipient Setup**: Recipient generates a stealth address keypair `(s_priv, s_pub)` and publishes `s_pub` as their payment address.

2. **Payment Initiation**: Sender generates a random nonce `r` and computes the stealth address:
   ```
   stealth_address = H(s_pub || r || context)
   ```

3. **Transaction Construction**: Sender constructs a transaction that sends funds to `stealth_address` and includes a zero-knowledge proof demonstrating:
   - Ownership of source funds
   - Correct stealth address derivation
   - Balance preservation

4. **On-Chain Verification**: The NyxPay program verifies the ZK proof and executes the state transition.

5. **Recipient Discovery**: Recipient scans the blockchain for transactions to addresses matching their stealth address pattern and uses their private key to claim funds.

## Privacy Score

The privacy score is a quantitative measure of a wallet's privacy properties. It is computed as:

```
score = w_link * linkability_score + w_meta * metadata_score + w_zk * zk_score
```

Where:
- `linkability_score`: Measures the degree to which transactions can be linked together
- `metadata_score`: Measures metadata leakage (timing, amounts, patterns)
- `zk_score`: Measures the proportion of transactions protected by zero-knowledge proofs
- `w_link`, `w_meta`, `w_zk`: Weighting factors

The score ranges from 0 (no privacy) to 100 (maximum privacy). It is computed locally and never transmitted to the network.

## Differences from Mixers and Tumblers

NyxPay differs from mixing and tumbling protocols in several fundamental ways:

1. **Cryptographic vs. Anonymity Sets**: Mixers rely on anonymity sets (pools of users) to provide privacy. NyxPay uses cryptographic techniques (stealth addresses and ZK proofs) that provide privacy guarantees independent of other users.

2. **No Pooling**: NyxPay does not require users to pool funds or wait for other users. Privacy is achieved through cryptography, not through coordination.

3. **Deterministic Privacy**: The privacy guarantees of NyxPay are deterministic and do not depend on network conditions, user behavior, or timing.

4. **Selective Disclosure**: NyxPay supports selective disclosure proofs that enable compliance without sacrificing privacy for other transactions.

## Design Goals

1. **Cryptographic Privacy**: Provide strong privacy guarantees through cryptography, not anonymity sets
2. **Solana Compatibility**: Operate within Solana's runtime constraints and account model
3. **Auditability**: Enable selective disclosure for compliance and auditing
4. **Performance**: Minimize on-chain verification costs and transaction latency
5. **Security**: Maintain security even if cryptographic assumptions are weakened

## Non-Goals

1. **Anonymity Sets**: NyxPay does not implement mixing or tumbling mechanisms
2. **Cross-Chain Privacy**: Privacy guarantees apply only within the Solana network
3. **Metadata Hiding**: While NyxPay minimizes metadata leakage, it does not hide all transaction metadata (e.g., transaction size, timing)
4. **Quantum Resistance**: Current implementation does not provide post-quantum security guarantees

## Protocol Invariants

The following invariants are enforced by the on-chain program:

1. **Balance Preservation**: The sum of all account balances before and after a transaction must be equal
2. **Non-Negative Balances**: No account balance may become negative
3. **Proof Validity**: All state transitions must be accompanied by valid zero-knowledge proofs
4. **Double-Spend Prevention**: Each input can only be spent once

## Security Considerations

See [SECURITY.md](./SECURITY.md) for a detailed threat model, security analysis, and trust assumptions.

## Documentation

- [Security Model](./SECURITY.md)
- [Solana Program](./programs/nyxpay_program/README.md)
- [Zero-Knowledge System](./zk/README.md)
- [Wallet Architecture](./wallet/README.md)
- [Audit Guidelines](./audits/README.md)

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on contributing to NyxPay.
