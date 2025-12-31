# Extension Registry

This document defines the formal mechanism for extending NyxPay's zero-knowledge system with new proof types, circuits, and privacy primitives without breaking existing functionality.

## Extension Principles

1. **Backward Compatibility**: Extensions must not break existing proofs or verification
2. **Explicit Registration**: New proof types must be explicitly registered
3. **Versioning**: Extensions are versioned to enable migration
4. **Audit Requirements**: Extensions must undergo security review

## Proof Type Registration

### Registration Process

1. **Specification**: Define the proof statement, public inputs, and private inputs
2. **Circuit Implementation**: Implement the zero-knowledge circuit
3. **Verification Implementation**: Implement on-chain verification
4. **Security Review**: Undergo security audit
5. **Registration**: Register the proof type with a unique identifier

### Proof Type Identifier

Each proof type has a unique identifier:

```rust
pub enum ProofType {
    Transfer = 0,
    Disclosure = 1,
    StealthAddress = 2,
    // New proof types are assigned sequential identifiers
    CustomType = N,
}
```

### Registration Format

```rust
pub struct ProofTypeRegistration {
    pub proof_type: ProofType,
    pub version: u8,
    pub circuit_id: [u8; 32],
    pub public_input_schema: PublicInputSchema,
    pub verification_function: VerificationFunction,
}
```

## Circuit Extension

### Adding New Circuits

1. **Define Circuit Logic**: Specify in-circuit computations and constraints
2. **Define Boundaries**: Specify what belongs in-circuit vs. off-circuit
3. **Implement Circuit**: Implement the circuit in the proof system
4. **Test Circuit**: Verify correctness and security properties
5. **Register Circuit**: Register with circuit identifier

### Circuit Versioning

Circuits are versioned to enable:
- Migration to improved circuits
- Fixing bugs without breaking existing proofs
- Optimizing circuits without changing proof semantics

### Circuit Registry

```rust
pub struct CircuitRegistry {
    pub circuit_id: [u8; 32],
    pub version: u8,
    pub circuit_hash: [u8; 32],
    pub trusted_setup: Option<TrustedSetup>,
    pub verification_key: VerificationKey,
}
```

## Privacy Primitive Extension

### Adding New Privacy Primitives

1. **Define Privacy Property**: Specify what privacy property is provided
2. **Define Threat Model**: Specify what attacks are mitigated
3. **Implement Primitive**: Implement the cryptographic primitive
4. **Integrate with Protocol**: Integrate with existing protocol components
5. **Security Review**: Undergo security audit

### Privacy Primitive Types

- **Address Privacy**: New stealth address schemes
- **Amount Privacy**: New amount hiding techniques
- **Linkability Privacy**: New unlinkability mechanisms
- **Metadata Privacy**: New metadata hiding techniques

## Protocol Integration

### On-Chain Integration

New proof types are integrated on-chain through:

1. **Instruction Extension**: Add new instructions to the program
2. **Verification Integration**: Integrate verification into the program
3. **State Extension**: Extend account state if needed
4. **Migration Support**: Support migration from old to new proof types

### Wallet Integration

New proof types are integrated in wallets through:

1. **Proof Generation**: Implement proof generation
2. **Transaction Construction**: Update transaction construction
3. **UI Updates**: Update user interface if needed
4. **Migration Tools**: Provide tools for migrating to new proof types

## Versioning Strategy

### Semantic Versioning

Extensions follow semantic versioning:
- **Major**: Breaking changes (incompatible proof formats)
- **Minor**: New features (new proof types, backward compatible)
- **Patch**: Bug fixes and optimizations

### Migration Path

1. **Deprecation Notice**: Announce deprecation of old proof types
2. **Migration Period**: Provide migration tools and documentation
3. **Sunset Date**: Set date for removing old proof types
4. **Removal**: Remove old proof types after sunset

## Security Review Process

### Review Requirements

All extensions must undergo:
1. **Cryptographic Review**: Review of cryptographic primitives
2. **Circuit Review**: Review of zero-knowledge circuits
3. **Implementation Review**: Review of code implementation
4. **Integration Review**: Review of protocol integration

### Review Criteria

- Correctness: Does the extension correctly implement its specification?
- Security: Does the extension provide the claimed security properties?
- Performance: Does the extension meet performance requirements?
- Compatibility: Does the extension maintain backward compatibility?

## Example: Adding a Range Proof

### Specification

**Proof Statement**: Prove that a value is within a range `[a, b]` without revealing the value.

**Public Inputs**: Range bounds `a`, `b`

**Private Inputs**: Value `v`

### Circuit Implementation

```rust
// Pseudo-code for range proof circuit
fn range_proof_circuit(
    value: PrivateInput,
    lower_bound: PublicInput,
    upper_bound: PublicInput,
) -> Proof {
    // Prove that value >= lower_bound
    prove_greater_than_or_equal(value, lower_bound);
    
    // Prove that value <= upper_bound
    prove_less_than_or_equal(value, upper_bound);
    
    // Generate proof
    generate_proof()
}
```

### Registration

```rust
ProofTypeRegistration {
    proof_type: ProofType::RangeProof,
    version: 1,
    circuit_id: [0x12, ...], // Unique identifier
    public_input_schema: PublicInputSchema {
        fields: vec![
            Field::U64("lower_bound"),
            Field::U64("upper_bound"),
        ],
    },
    verification_function: verify_range_proof,
}
```

### Integration

1. Add `RangeProof` instruction to the program
2. Implement `verify_range_proof` function
3. Update wallet to generate range proofs
4. Update documentation

## Extension Registry Format

The extension registry is stored as:

```toml
[extensions.range_proof]
proof_type = "RangeProof"
version = 1
circuit_id = "0x12..."
public_inputs = ["lower_bound", "upper_bound"]
private_inputs = ["value"]
status = "active"
audited = true
audit_report = "https://..."
```

## Maintenance

### Registry Updates

The extension registry is updated through:
1. Pull requests with extension proposals
2. Security review and approval
3. Registration in the registry
4. Documentation updates

### Registry Access

The registry is:
- Publicly readable
- Updated through governance process
- Versioned with the protocol

## Future Considerations

Potential future extensions:
- Post-quantum cryptography
- Cross-chain privacy
- Advanced disclosure mechanisms
- Performance optimizations
- New privacy primitives

All extensions must follow the registration and review process defined in this document.
