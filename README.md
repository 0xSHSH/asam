# Autonomous Smart Account Manager (ASAM)

[![Rust](https://img.shields.io/badge/Rust-1.75+-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**ASAM** is a Rust-based tool designed to monitor and manage Ethereum smart accounts autonomously. It ensures account balances remain above critical thresholds, optimizes cross-chain fund routing, and provides detailed logging and error handling for enhanced observability.

## Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Architecture](#architecture)
4. [Setup Instructions](#setup-instructions)
5. [Usage](#usage)
6. [Error Handling and Logging](#error-handling-and-logging)
7. [Future Improvements](#future-improvements)
8. [Contributing](#contributing)
9. [License](#license)

## Overview

ASAM connects to the Ethereum network via Alchemy and monitors a specified Ethereum address for balance thresholds. If the balance falls below a critical threshold, it logs detailed errors and retries after a configurable interval. Additionally, ASAM supports cross-chain fund routing with robust validation, liquidity checks, and bridge transaction simulations.

## Features

1. **Enhanced Error Handling and Logging**:
   - Detailed error messages with context
   - Comprehensive logging system
   - Transaction simulation and validation

2. **Robust DeFi Protocol Integration**:
   - Flexible protocol data parsing
   - Multiple APY calculation methods
   - Intelligent pool selection algorithm

3. **Advanced Cross-Chain Support**:
   - Support for Ethereum, Arbitrum, Optimism, Polygon, and Fantom
   - Liquidity validation across chains
   - Transaction simulation before execution

4. **Safe Balance Management**:
   - Critical balance threshold monitoring
   - Gas estimation and validation
   - Transaction batching and optimization

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- An Ethereum RPC endpoint (e.g., Infura)
- A valid Ethereum account address

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/asam.git
   cd asam
   ```

2. Create a `.env` file:
   ```env
   ETH_RPC_URL=https://mainnet.infura.io/v3/your-project-id
   ACCOUNT_ADDRESS=0x0000000000000000000000000000000000000000
   RUST_LOG=info
   API_TIMEOUT_SECS=10
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run the tests:
   ```bash
   cargo test
   ```

## Usage

Run the application:
```bash
cargo run
```

The application will:
- Monitor account balances
- Identify optimal DeFi opportunities
- Execute cross-chain transfers when beneficial
- Provide detailed logging of all operations

## Project Structure

```
asam/
├── src/
│   ├── agents/
│   │   ├── cross_chain_router.rs  # Cross-chain transfer logic
│   │   ├── defi_optimizer.rs      # DeFi protocol integration
│   │   ├── safe_manager.rs        # Account management
│   │   └── mod.rs                 # Module declarations
│   └── main.rs                    # Application entry point
├── Cargo.toml                     # Project configuration
├── .env.example                   # Environment variables template
└── README.md                      # Project documentation
```

## Configuration

The application can be configured through environment variables:

- `ETH_RPC_URL`: Ethereum RPC endpoint URL (required)
- `ACCOUNT_ADDRESS`: Account address to monitor (required)
- `RUST_LOG`: Log level (optional, defaults to "info")
- `API_TIMEOUT_SECS`: API request timeout (optional, defaults to 10)

## Testing

The project includes comprehensive tests for various edge cases:

1. **Chain Validation Tests**:
   ```bash
   cargo test cross_chain_router
   ```

2. **DeFi Protocol Tests**:
   ```bash
   cargo test defi_optimizer
   ```

3. **Balance Management Tests**:
   ```bash
   cargo test safe_manager
   ```

## Error Handling

ASAM provides detailed error handling for:
- Invalid chain configurations
- Insufficient liquidity
- API failures
- Transaction simulation errors
- Balance threshold violations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Architecture

The architecture of ASAM is modular and follows a layered design to ensure scalability, maintainability, and extensibility:

```plaintext
+-----------------------------+
|        Main Application     |
|                             |
|  +-----------------------+  |
|  |      Initialization    |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |
|  |    Monitoring Loop     |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |
|  | Cross-Chain Routing    |  |
|  +-----------------------+  |
+-----------------------------+
              |
              v
+-----------------------------+
|         Agents Layer        |
|                             |
|  +-----------------------+  |
|  |      SafeManager       |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |
|  |    DefiOptimizer       |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |
|  |  CrossChainRouter      |  |
|  +-----------------------+  |
+-----------------------------+
              |
              v
+-----------------------------+
|        Utilities Layer      |
|                             |
|  +-----------------------+  |
|  |       Logging          |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |
|  |    Error Handling      |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |
|  |    Configuration       |  |
|  +-----------------------+  |
+-----------------------------+
```

1. **Safe Manager**
   - Handles account balance monitoring
   - Manages transaction execution
   - Implements safety checks and thresholds

2. **DeFi Optimizer**
   - Analyzes DeFi protocols
   - Calculates optimal yields
   - Manages protocol interactions

3. **Cross-Chain Router**
   - Coordinates cross-chain transfers
   - Validates liquidity across chains
   - Ensures transaction safety

## Setup Instructions

1. **Prerequisites**
   - Rust 1.75 or higher
   - Cargo package manager
   - Ethereum RPC endpoint
   - Valid Ethereum account

2. **Installation**
   ```bash
   git clone https://github.com/yourusername/asam.git
   cd asam
   cargo build
   ```

3. **Configuration**
   Create a `.env` file with:
   ```env
   ETH_RPC_URL=your-rpc-url
   ACCOUNT_ADDRESS=your-account-address
   RUST_LOG=info
   ```

## Usage

1. **Starting the Service**
   ```bash
   cargo run
   ```

2. **Monitoring**
   - View logs in real-time
   - Check balance status
   - Monitor cross-chain operations

3. **Configuration Options**
   - Adjust balance thresholds
   - Modify DeFi parameters
   - Configure chain settings

## Error Handling and Logging

- Comprehensive error tracking
- Detailed transaction logs
- Balance monitoring alerts
- Cross-chain validation reports

## Future Improvements

1. **Enhanced Features**
   - Multi-signature support
   - Additional chain integrations
   - Advanced DeFi strategies

2. **Technical Improvements**
   - Performance optimizations
   - Extended test coverage
   - API enhancements

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement changes
4. Add tests
5. Submit a pull request

## License

MIT License. See [LICENSE](LICENSE) for details.
