#!/bin/bash

# CryptoWallet Backend Runner

echo "CryptoWallet Backend - Starting..."

# Check if MongoDB is running
if ! command -v mongod &> /dev/null; then
    echo "MongoDB not found. Install MongoDB first."
    exit 1
fi

# Start MongoDB if not running
if ! pgrep -x "mongod" > /dev/null; then
    echo "Starting MongoDB..."
    mongod --dbpath ./data &
    sleep 2
fi

# Build and run Rust backend
echo "Building Rust backend..."
cargo build --release

echo "Starting backend server..."
cargo run --release
