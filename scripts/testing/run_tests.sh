#!/bin/bash
# Run NyxPay test suite

set -e

echo "Running NyxPay test suite..."

# Run program tests
echo "Running program tests..."
cd ../../programs/nyxpay_program
cargo test-sbf

# Run wallet tests (if implemented)
if [ -d "../../wallet/src" ]; then
    echo "Running wallet tests..."
    cd ../../wallet
    cargo test
fi

echo "All tests passed!"
