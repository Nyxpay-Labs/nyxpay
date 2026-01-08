#!/bin/bash
# Setup local Solana validator for NyxPay development

set -e

echo "Setting up local Solana validator..."

# Start local validator in background
solana-test-validator \
    --reset \
    --quiet \
    &

VALIDATOR_PID=$!

echo "Waiting for validator to start..."
sleep 5

# Airdrop SOL to default keypair
echo "Airdropping SOL to default keypair..."
solana airdrop 10

echo "Local validator is running (PID: $VALIDATOR_PID)"
echo "To stop: kill $VALIDATOR_PID"
