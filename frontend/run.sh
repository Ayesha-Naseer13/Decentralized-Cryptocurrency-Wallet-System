#!/bin/bash

# CryptoWallet Frontend Runner

echo "CryptoWallet Frontend - Starting..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Node.js not found. Install Node.js 16+ first."
    exit 1
fi

# Install dependencies
echo "Installing dependencies..."
npm install

# Start development server
echo "Starting frontend development server..."
npm run dev
