# Proof Statements

This document defines all formal proof statements used in NyxPay. Each statement is described in precise cryptographic terms.

## Notation

- `H`: Cryptographic hash function
- `G`: Generator of the elliptic curve group
- `sk`: Secret key
- `pk`: Public key
- `a`: Transfer amount
- `b`: Account balance
- `addr`: Account address
- `nonce`: Account nonce
- `stealth_addr`: Stealth address
- `r`: Random nonce for stealth address generation

## Transfer Proof Statement

### Statement

A transfer proof demonstrates that a transaction satisfies:

1. **Balance Preservation**: The sum of input balances equals the sum of output balances
2. **Non-Negative Transfers**: All transfer amounts are non-negative
3. **Ownership**: The sender owns the source account
4. **Nonce Validity**: The account nonce is valid and prevents double-spending

### Formal Definition

Given:
- Source account `S` with balance `b_s`, nonce `n_s`, authority `pk_s`
- Destination account `D` with balance `b_d`, nonce `n_d`
- Transfer amount `a`
- Sender's secret key `sk_s`

The proof demonstrates:

```
∃ (b_s, b_d, a, sk_s, n_s, n_d) such that:
  1. b_s >= a
  2. b_s' = b_s - a
  3. b_d' = b_d + a
  4. b_s + b_d = b_s' + b_d'  (balance preservation)
  5. a >= 0  (non-negative transfer)
  6. Verify(pk_s, signature, (S, D, a, n_s))  (ownership)
  7. n_s' = n_s + 1  (nonce increment)
```

Where:
- `b_s'` and `b_d'` are the balances after the transfer
- `Verify` is the signature verification function
- The proof reveals only: `S`, `D`, `n_s`, `n_d` (public inputs)
- The proof hides: `b_s`, `b_d`, `a`, `sk_s` (private inputs)

### Public Inputs

- Source account address `S`
- Destination account address `D`
- Source account nonce `n_s`
- Destination account nonce `n_d`

### Private Inputs

- Source account balance `b_s`
- Destination account balance `b_d`
- Transfer amount `a`
- Sender's secret key `sk_s`

### Verification

The verifier checks:
1. The proof is valid (circuit verification passes)
2. The public inputs match the on-chain account states
3. The nonce `n_s` matches the source account's current nonce
4. The signature is valid for the transaction

## Stealth Address Proof Statement

### Statement

A stealth address proof demonstrates that a stealth address is correctly derived from a recipient's public key and a random nonce.

### Formal Definition

Given:
- Recipient's public key `pk_r`
- Random nonce `r`
- Stealth address `stealth_addr`

The proof demonstrates:

```
∃ (pk_r, r) such that:
  stealth_addr = H(pk_r || r || context)
```

Where:
- `H` is a cryptographic hash function
- `context` is a protocol-specific context string
- The proof reveals only: `stealth_addr` (public input)
- The proof hides: `pk_r`, `r` (private inputs)

### Public Inputs

- Stealth address `stealth_addr`

### Private Inputs

- Recipient's public key `pk_r`
- Random nonce `r`

### Verification

The verifier checks:
1. The proof is valid (circuit verification passes)
2. The stealth address matches the destination in the transaction

## Balance Preservation Proof Statement

### Statement

A balance preservation proof demonstrates that a transaction does not create or destroy funds.

### Formal Definition

Given:
- Input accounts with balances `b_1, b_2, ..., b_n`
- Output accounts with balances `b_1', b_2', ..., b_m'`

The proof demonstrates:

```
∑_{i=1}^n b_i = ∑_{j=1}^m b_j'
```

Where:
- The proof reveals only: the number of inputs and outputs (public inputs)
- The proof hides: individual balances (private inputs)

### Public Inputs

- Number of input accounts `n`
- Number of output accounts `m`

### Private Inputs

- Input balances `b_1, b_2, ..., b_n`
- Output balances `b_1', b_2', ..., b_m'`

### Verification

The verifier checks:
1. The proof is valid (circuit verification passes)
2. The sums are equal (enforced by the circuit)

## Non-Negative Transfer Proof Statement

### Statement

A non-negative transfer proof demonstrates that all transfer amounts in a transaction are non-negative.

### Formal Definition

Given:
- Transfer amounts `a_1, a_2, ..., a_k`

The proof demonstrates:

```
∀ i ∈ {1, 2, ..., k}: a_i >= 0
```

Where:
- The proof reveals only: the number of transfers `k` (public input)
- The proof hides: individual amounts `a_i` (private inputs)

### Public Inputs

- Number of transfers `k`

### Private Inputs

- Transfer amounts `a_1, a_2, ..., a_k`

### Verification

The verifier checks:
1. The proof is valid (circuit verification passes)
2. All amounts are non-negative (enforced by the circuit)

## Double-Spend Prevention Proof Statement

### Statement

A double-spend prevention proof demonstrates that an input has not been spent before.

### Formal Definition

Given:
- Account address `addr`
- Account nonce `n`
- Spent nonces registry `R`

The proof demonstrates:

```
n ∉ R
```

Where:
- `R` is the set of all spent nonces
- The proof reveals only: `addr`, `n` (public inputs)
- The proof may hide: parts of `R` depending on implementation

### Public Inputs

- Account address `addr`
- Account nonce `n`

### Private Inputs

- Spent nonces registry `R` (may be public or private depending on implementation)

### Verification

The verifier checks:
1. The proof is valid (circuit verification passes)
2. The nonce `n` is not in the spent nonces registry
3. The account's current nonce matches `n`

## Disclosure Proof Statement

### Statement

A disclosure proof enables selective disclosure of transaction properties.

### Formal Definition

Given:
- Transaction `tx` with properties `P = {p_1, p_2, ..., p_n}`
- Disclosed properties `D ⊆ P`

The proof demonstrates:

```
∃ tx such that:
  For all p_i ∈ D: p_i has the claimed value
  For all p_j ∈ P \ D: p_j is not revealed
```

Where:
- The proof reveals: properties in `D` (public inputs)
- The proof hides: properties in `P \ D` (private inputs)

### Public Inputs

- Disclosed properties `D` and their values

### Private Inputs

- Full transaction `tx`
- Undisclosed properties `P \ D`

### Verification

The verifier checks:
1. The proof is valid (circuit verification passes)
2. The disclosed properties match the claimed values
3. The transaction exists on-chain (if applicable)

## Composite Proof Statement

### Statement

A composite proof combines multiple proof statements into a single proof.

### Formal Definition

A composite proof demonstrates all of:
- Balance preservation
- Non-negative transfers
- Ownership
- Nonce validity
- Stealth address correctness (if applicable)

The composite proof is the conjunction of all individual proof statements.

### Verification

The verifier checks that all component proofs are valid.

## Security Properties

All proof statements must satisfy:

1. **Soundness**: Invalid statements are rejected with overwhelming probability
2. **Zero-Knowledge**: Valid proofs reveal no information beyond the statement's validity
3. **Completeness**: Valid statements can be proven with overwhelming probability

## Implementation Considerations

1. **Proof Size**: Proofs must be small enough to fit in Solana transactions
2. **Verification Time**: Verification must complete within Solana's transaction time limits
3. **Circuit Size**: Circuits must be optimized for proof generation time
4. **Trusted Setup**: If using SNARKs, the trusted setup must be performed securely
