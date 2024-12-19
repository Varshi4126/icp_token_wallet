# ICP Token Wallet

A secure token wallet implementation for the Internet Computer Protocol (ICP) blockchain, built with Rust. This wallet implements IRCRC2 token functionality including sending, receiving, and balance checking capabilities.

## Features

- Send and receive IRCRC2 tokens
- Real-time balance checking
- Secure transaction handling
- Comprehensive test suite
- Thread-safe state management

## Prerequisites

- Rust (1.56 or later)
- DFX SDK (latest version)
- Internet Computer CLI
- Ubuntu or compatible Linux distribution

## Project Structure

```
icp_token_wallet/
├── src/
│   ├── lib.rs                 # Main contract implementation
│   └── icp_token_wallet.did   # Candid interface definition
├── frontend/
│   └── assets/               # Frontend assets
├── Cargo.toml                # Rust dependencies
├── dfx.json                  # DFX configuration
└── README.md                 # This file
```

## Installation

1. Install the Internet Computer SDK:
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

2. Install the Rust target for WebAssembly:
```bash
rustup target add wasm32-unknown-unknown
```

3. Clone and build the project:
```bash
git clone <repository-url>
cd icp_token_wallet
cargo build
```

## Deployment

1. Start the local Internet Computer network:
```bash
dfx stop
dfx start --clean --background
```

2. Deploy the canister:
```bash
dfx deploy
```

## Usage

### Get Your Account ID
```bash
dfx identity get-principal
dfx ledger account-id
```

### Check Balance
```bash
dfx canister call icp_token_wallet get_balance '(blob "YOUR_ACCOUNT_ID")'
```

### Send Tokens
```bash
dfx canister call icp_token_wallet transfer '(record { 
    to = blob "RECIPIENT_ACCOUNT_ID"; 
    amount = record { e8s = 100000000 } 
})'
```

### Receive Tokens
```bash
dfx canister call icp_token_wallet receive_tokens '(record { e8s = 100000000 })'
```

## Testing

Run the test suite:
```bash
cargo test
```

## Common Issues and Solutions

1. If dfx shows "already running":
```bash
dfx stop
dfx start --clean --background
```

2. If deployment fails:
```bash
dfx stop
dfx start --clean --background
cargo build
dfx deploy
```

3. If "no wasm module" error appears:
```bash
dfx stop
dfx start --clean --background
cargo build
dfx deploy
```

## Security Features

- Balance validation before transfers
- Secure state management
- Protected against integer overflow
- Thread-safe operations
- Proper error handling

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'Add YourFeature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a Pull Request
