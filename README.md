# Rust Blockchain Implementation

## Overview

A simple blockchain implementation in Rust, demonstrating core blockchain concepts including block creation, hashing, and chain validation.

## Features

- Create blockchain from scratch
- Add blocks with custom data
- Generate genesis block
- Validate blockchain integrity
- Print blockchain details

## Prerequisites

- Rust programming language
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/darrendc26/blockchain-implementation-rust.git
cd blockchain-implementation-rust
```

2. Build the project:
```bash
cargo build
```

3. Run the blockchain:
```bash
cargo run
```

## How It Works

### Block Structure
- Contains index, timestamp, data, previous hash, and current hash
- Uses SHA-256 for hash generation

### Blockchain Validation
- Verifies integrity by checking hash connections between blocks

## Example Usage

```rust
let mut blockchain = Blockchain::new();
blockchain.add_genesis_block();
blockchain.add_block(String::from("First Transaction"));
blockchain.print_chain();
```

## Limitations

- In-memory blockchain
- No persistent storage
- Simple hashing strategy

## Potential Improvements

- Implement Proof of Work
- Add transaction support
- Create persistent storage
- Develop consensus mechanism

## License

MIT License