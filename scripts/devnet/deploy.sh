#!/bin/bash
# Deploy NyxPay program to Solana devnet

set -e

PROGRAM_NAME="nyxpay_program"
NETWORK="devnet"

echo "Building NyxPay program for $NETWORK..."

# Build the program
cd ../../programs/$PROGRAM_NAME
cargo build-sbf

echo "Deploying to $NETWORK..."

# Deploy to devnet
solana program deploy \
    --url $NETWORK \
    target/deploy/$PROGRAM_NAME.so

echo "Deployment complete. Program ID:"
solana address -k target/deploy/$PROGRAM_NAME-keypair.json
