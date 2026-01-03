# Transaction Builder

The transaction builder provides deterministic transaction construction and signing for NyxPay transactions.

## Overview

The transaction builder ensures that transactions are constructed deterministically and securely. It handles:

- Input selection
- Stealth address generation
- Proof generation
- Transaction assembly
- Signing

## Deterministic Construction

### Input Selection

Input selection is deterministic:

1. **Account Selection**: Selects accounts based on deterministic criteria (balance, nonce, etc.)
2. **Amount Selection**: Selects amounts deterministically
3. **Ordering**: Orders inputs deterministically

### Proof Generation

Proof generation is deterministic given inputs:

1. **Input Formatting**: Formats inputs deterministically
2. **Circuit Execution**: Executes circuit deterministically
3. **Proof Generation**: Generates proof deterministically

### Transaction Assembly

Transaction assembly is deterministic:

1. **Instruction Construction**: Constructs instructions deterministically
2. **Account Ordering**: Orders accounts deterministically
3. **Data Serialization**: Serializes data deterministically

## Transaction Structure

### Accounts

Transactions include the following accounts:

- **Source Account**: Account sending funds (writable)
- **Destination Account**: Account receiving funds (writable)
- **Proof Account**: Account containing zero-knowledge proof (readable)
- **Program Account**: NyxPay program account
- **System Program**: System program (if needed)
- **Fee Payer**: Account paying transaction fees (signer)

### Instructions

Transactions include NyxPay instructions:

- **Transfer**: Transfer instruction with proof
- **InitializeAccount**: Account initialization (if needed)
- **CreateStealthAddress**: Stealth address creation (if needed)

### Signatures

Transactions are signed by:

- **Transaction Authority**: User's keypair (signer)
- **Fee Payer**: Fee payer's keypair (signer)

## Signing

### Signature Scheme

Transactions use Ed25519 signatures:

1. **Message Construction**: Constructs message from transaction data
2. **Signing**: Signs message with private key
3. **Verification**: Verifies signature (before submission)

### Key Management

Signing keys are managed securely:

1. **Key Storage**: Keys are stored securely (hardware wallet, secure storage)
2. **Key Usage**: Keys are used only for signing
3. **Key Protection**: Keys are protected from exposure

## Validation

### Pre-Submission Validation

Transactions are validated before submission:

1. **Proof Validity**: Verifies proof is valid
2. **Account State**: Verifies account states are valid
3. **Balance Sufficiency**: Verifies sufficient balance
4. **Nonce Validity**: Verifies nonces are valid
5. **Signature Validity**: Verifies signatures are valid

### Error Handling

Validation errors are handled:

1. **Error Detection**: Detects validation errors
2. **Error Reporting**: Reports errors to user
3. **Error Recovery**: Provides recovery options

## Transaction Submission

### Network Interaction

Transactions are submitted to the Solana network:

1. **RPC Connection**: Connects to Solana RPC endpoint
2. **Transaction Submission**: Submits transaction
3. **Confirmation**: Waits for confirmation
4. **Error Handling**: Handles submission errors

### Retry Logic

Failed transactions are retried:

1. **Error Detection**: Detects submission failures
2. **Retry Strategy**: Implements retry strategy
3. **Fee Adjustment**: Adjusts fees if needed
4. **Timeout Handling**: Handles timeouts

## Security Considerations

### Deterministic Construction

Deterministic construction ensures:

1. **Reproducibility**: Transactions can be reproduced
2. **Auditability**: Transactions can be audited
3. **Debugging**: Transactions can be debugged

### Signing Security

Signing is performed securely:

1. **Key Protection**: Keys are protected from exposure
2. **Signature Verification**: Signatures are verified before submission
3. **Replay Prevention**: Nonces prevent replay attacks

### Validation Security

Validation ensures:

1. **Correctness**: Transactions are correct before submission
2. **Security**: Security properties are maintained
3. **Error Prevention**: Errors are prevented before submission

## Testing

The transaction builder includes comprehensive tests:

- Unit tests for construction logic
- Integration tests for signing and submission
- Property-based tests for determinism
- Security tests for key management

Run tests with:
```bash
cargo test
```

## Usage

### Creating a Transaction

```rust
use nyxpay_wallet::tx_builder::TransactionBuilder;

let builder = TransactionBuilder::new(keypair);
let transaction = builder.build_transfer(
    source_account,
    destination,
    amount,
)?;
```

### Signing a Transaction

```rust
let signed_transaction = builder.sign_transaction(transaction)?;
```

### Submitting a Transaction

```rust
let signature = builder.submit_transaction(signed_transaction)?;
```
