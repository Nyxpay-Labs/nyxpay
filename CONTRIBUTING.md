# Contributing to NyxPay

Thank you for your interest in contributing to NyxPay. This document outlines the contribution process and guidelines.

## Code of Conduct

All contributors must adhere to the [Code of Conduct](./CODE_OF_CONDUCT.md). By participating, you agree to maintain a respectful and professional environment.

## Security-First Development

NyxPay is a security-critical protocol. All contributions must prioritize security and correctness over features or performance.

### Security Review Process

1. **Security-Critical Changes**: Any change that affects cryptographic operations, zero-knowledge proofs, or on-chain program logic must undergo security review
2. **External Review**: Security-critical changes require review by at least two independent reviewers with relevant expertise
3. **Formal Verification**: Critical components may require formal verification before acceptance

## Contribution Workflow

### 1. Issue Discussion

Before making significant changes, open an issue to discuss:
- The problem you're solving
- Your proposed approach
- Security implications
- Compatibility considerations

### 2. Fork and Branch

1. Fork the repository
2. Create a feature branch from `main`
3. Use descriptive branch names (e.g., `feature/stealth-address-optimization`, `fix/double-spend-prevention`)

### 3. Development

- Follow the coding standards and style guidelines
- Write comprehensive tests for all new code
- Update documentation for any API or protocol changes
- Ensure all tests pass locally

### 4. Commit Messages

Use clear, descriptive commit messages:

```
Short summary (50 chars or less)

More detailed explanation if needed. Wrap at 72 characters.
Explain what and why, not how.

- Bullet points are acceptable
- Reference issues: Fixes #123
```

### 5. Pull Request

1. Push your branch to your fork
2. Open a pull request against `main`
3. Fill out the pull request template completely
4. Request reviews from maintainers

### 6. Review Process

- All pull requests require at least two approvals
- Security-critical changes require additional security review
- Maintainers may request changes or clarification
- Once approved, a maintainer will merge the PR

## Coding Standards

### Rust (Solana Program)

- Follow Rust style guidelines (`rustfmt`)
- Use `clippy` for linting
- Document all public functions and types
- Use meaningful variable and function names
- Prefer explicit error handling over panics

### Documentation

- All public APIs must be documented
- Protocol changes must be documented in relevant README files
- Security implications must be documented in SECURITY.md if applicable

### Testing

- Aim for high test coverage (target: >90% for critical paths)
- Include unit tests, integration tests, and property-based tests where appropriate
- Test both success and failure cases
- Test edge cases and boundary conditions

## Security Vulnerabilities

### Responsible Disclosure

If you discover a security vulnerability:

1. **Do not** open a public issue
2. Email security@nyxpay.dev (or use the security contact method specified in the repository)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if available)

### Vulnerability Response

- The security team will acknowledge receipt within 48 hours
- Critical vulnerabilities will be addressed within 7 days
- Vulnerabilities will be disclosed responsibly after fixes are deployed
- Contributors who report vulnerabilities will be credited (if desired)

## Areas for Contribution

### High Priority

- Security audits and reviews
- Zero-knowledge proof optimizations
- Documentation improvements
- Test coverage improvements
- Performance optimizations

### Medium Priority

- Wallet feature enhancements
- Developer tooling
- Example implementations
- Tutorial content

### Low Priority

- UI/UX improvements
- Cosmetic changes
- Non-critical bug fixes

## Development Setup

See the relevant README files for setup instructions:

- [Solana Program](./programs/nyxpay_program/README.md)
- [Wallet](./wallet/README.md)
- [Zero-Knowledge System](./zk/README.md)

## Questions?

- Open an issue for general questions
- Contact maintainers for security-related questions
- Check existing documentation before asking

## License

By contributing to NyxPay, you agree that your contributions will be licensed under the MIT License.
