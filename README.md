# Token Wallet for ICP Blockchain

This project implements a Rust-based token wallet for the ICP blockchain, supporting basic functionalities such as sending and receiving IRCRC2 tokens. It demonstrates your proficiency with Rust, blockchain principles, and the Internet Computer Protocol (ICP) smart contract development.

## Table of Contents
- [Project Overview](#project-overview)
- [Setup Instructions](#setup-instructions)
- [Interacting with the Wallet](#interacting-with-the-wallet)
- [Running Unit Tests](#running-unit-tests)
- [Code Structure](#code-structure)
- [Security Features](#security-features)
- [Testing the Smart Contract](#testing-the-smart-contract)
- [Contributing](#contributing)
- [License](#license)

## Project Overview
The Token Wallet allows users to interact with the ICP blockchain, supporting these core features:

- **Send Tokens:** Users can send tokens to other addresses.
- **Receive Tokens:** Users can receive tokens and update their balance.
- **Get Balance:** Users can view the current token balance of their wallet.

The wallet's core logic is written in Rust and deployed as a smart contract on a local ICP testnet.

## Setup Instructions

### Prerequisites
Before starting, ensure you have the following installed:

- **Rust:** Install Rust if you haven't already.
- **dfx:** The Internet Computer SDK. You can install it by running the following command:

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
--------------------------------------------------------------------------------------------
Steps to Set Up and Run the Project :
# Clone the repository and navigate to the project folder
git clone <repository-url>
cd token_wallet

# Start the local ICP testnet
dfx start

# Deploy canisters to the local testnet
dfx deploy

# Check the status of the deployed canisters
dfx canister status token_wallet_backend

Interacting with the Wallet :
# Send tokens to a specified Principal ID
dfx canister call token_wallet_backend send_tokens '(<principal>, 10)'

# Receive tokens and update the balance
dfx canister call token_wallet_backend receive_tokens '(10)'

# Get the current balance of the wallet
dfx canister call token_wallet_backend get_balance '()'
---------------------------------------------------------------------------------------
Running Unit Tests:
# Run the unit tests for the smart contract
cargo test
Unit Test Examples:
-Test sending tokens with insufficient balance.
-Test sending zero tokens.
-Test receiving zero tokens.
 
 ---------------------------------------------------------------------------------------

Code Structure :
The project follows a clean code structure to separate logic and facilitate easy testing:
src/token_wallet_backend
lib.rs: Contains the smart contract logic, including wallet functions (send_tokens, receive_tokens, get_balance) and the initialization code.
tests.rs: Contains unit tests for the smart contract, verifying core functions like sending, receiving tokens, and checking the balance.