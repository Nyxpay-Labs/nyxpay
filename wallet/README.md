# NyxPay Wallet

The NyxPay wallet is a client-side application that enables users to interact with the NyxPay protocol. It provides functionality for generating stealth addresses, constructing private transactions, generating zero-knowledge proofs, and managing privacy scores.

## Architecture

The wallet is structured as a modular system with the following components:

- **Core Wallet Logic** (`src/`): Core wallet functionality, account management, and protocol interaction
- **Privacy Module** (`privacy/`): Privacy guarantees, metadata leakage analysis, and mitigations
- **Transaction Builder** (`tx-builder/`): Deterministic transaction construction and signing
- **User Interface** (`ui/`): Interaction model and security constraints

## Core Functionality

### Account Management

The wallet manages:
- User accounts and keypairs
- Account balances and nonces
- Stealth addresses
- Privacy scores

### Stealth Address Generation

The wallet generates stealth addresses for receiving payments:

1. **Recipient Setup**: User generates a stealth address keypair
2. **Address Publication**: User publishes the stealth address public key
3. **Payment Receipt**: Wallet scans for payments to stealth addresses
4. **Fund Claiming**: Wallet uses private key to claim funds

### Transaction Construction

The wallet constructs private transactions:

1. **Input Selection**: Selects source accounts and amounts
2. **Stealth Address Generation**: Generates stealth addresses for recipients
3. **Proof Generation**: Generates zero-knowledge proofs
4. **Transaction Assembly**: Assembles transaction with proof
5. **Signing**: Signs transaction with user's private key
6. **Submission**: Submits transaction to Solana network

### Zero-Knowledge Proof Generation

The wallet generates zero-knowledge proofs for transactions:

1. **Input Collection**: Collects private inputs (balances, amounts, keys)
2. **Circuit Execution**: Executes zero-knowledge circuit
3. **Proof Generation**: Generates proof using proof system
4. **Serialization**: Serializes proof for on-chain submission

### Privacy Score Computation

The wallet computes privacy scores for user accounts:

1. **Transaction Analysis**: Analyzes transaction history
2. **Linkability Assessment**: Measures transaction linkability
3. **Metadata Analysis**: Analyzes metadata leakage
4. **Score Calculation**: Computes composite privacy score

## Privacy Module

The privacy module (`privacy/`) provides:

### Privacy Guarantees

- **Transaction Privacy**: Amounts and recipients are hidden
- **Unlinkability**: Transactions cannot be linked to user identities
- **Selective Disclosure**: Users can prove specific properties

### Metadata Leakage Analysis

The module analyzes:
- **Timing Patterns**: Transaction timing and correlation
- **Amount Patterns**: Amount distributions and patterns
- **Address Patterns**: Address reuse and linkability
- **Network Patterns**: Network-level metadata

### Mitigations

The module provides mitigations for:
- **Timing Correlation**: Batching and delays
- **Amount Inference**: Amount obfuscation techniques
- **Address Linkability**: Stealth address best practices
- **Network Analysis**: Network-level privacy techniques

## Transaction Builder

The transaction builder (`tx-builder/`) provides:

### Deterministic Construction

Transactions are constructed deterministically:
- Input selection is deterministic
- Proof generation is deterministic (given inputs)
- Transaction assembly is deterministic

### Signing

Transactions are signed using:
- User's private key
- Ed25519 signature scheme
- Solana transaction format

### Validation

Transactions are validated before submission:
- Proof validity
- Account state validity
- Balance sufficiency
- Nonce validity

## User Interface

The user interface (`ui/`) provides:

### Interaction Model

- **Account Overview**: Display account balances and privacy scores
- **Transaction Creation**: Create and send private transactions
- **Transaction History**: View transaction history (with privacy considerations)
- **Settings**: Configure privacy preferences

### Security Constraints

- **Key Management**: Private keys are never exposed to UI
- **Proof Generation**: Proofs are generated securely
- **Transaction Signing**: Signing is performed securely
- **Error Handling**: Errors are handled securely without leaking information

## Security Considerations

### Key Management

- Private keys are stored securely (hardware wallet, secure storage)
- Keys are never transmitted over the network
- Key derivation follows best practices

### Proof Generation

- Proof generation is performed locally
- Private inputs are never exposed
- Proofs are validated before submission

### Transaction Security

- Transactions are validated before submission
- Signatures are verified
- Nonces prevent replay attacks

## Usage

### Initialization

```rust
use nyxpay_wallet::Wallet;

let wallet = Wallet::new(keypair)?;
```

### Creating a Transaction

```rust
let transaction = wallet.create_transfer(
    destination,
    amount,
    source_account,
)?;
```

### Generating a Proof

```rust
let proof = wallet.generate_proof(
    transaction,
    private_inputs,
)?;
```

### Computing Privacy Score

```rust
let score = wallet.compute_privacy_score(account)?;
```

## Testing

The wallet includes comprehensive tests:

- Unit tests for core functionality
- Integration tests for protocol interaction
- Privacy tests for privacy guarantees
- Security tests for key management

Run tests with:
```bash
cargo test
```

## Documentation

- [Privacy Module](./privacy/README.md)
- [Transaction Builder](./tx-builder/README.md)
- [User Interface](./ui/README.md)
