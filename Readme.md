# YieldMind

> A Solana on-chain treasury and yield-strategy program built with the Anchor framework.

YieldMind is a capstone project that implements a programmable **treasury management system** on the Solana blockchain. It allows a user (the "authority") to create a personal treasury vault, deposit and withdraw SPL tokens, define a yield strategy with a configurable risk profile, and trigger rebalances — all enforced by on-chain logic and Program Derived Addresses (PDAs).

This README explains what the project does, how it is structured, and how to set it up, build it, and test it from scratch.

---

## Table of Contents

1. [Overview](#overview)
2. [Key Features](#key-features)
3. [Tech Stack](#tech-stack)
4. [Project Structure](#project-structure)
5. [On-Chain Data Accounts](#on-chain-data-accounts)
6. [Instructions Reference](#instructions-reference)
7. [Prerequisites](#prerequisites)
8. [Step-by-Step Setup](#step-by-step-setup)
9. [Building the Program](#building-the-program)
10. [Running Tests](#running-tests)
11. [Deploying the Program](#deploying-the-program)
12. [Error Codes](#error-codes)
13. [Roadmap](#roadmap)
14. [Contributing](#contributing)
15. [License](#license)

---

## Overview

YieldMind models a simplified **yield-optimizing treasury** as a Solana smart contract (called a "program" in Solana terminology). Each user can:

- Spin up their own **Treasury** account, tied to their wallet via a PDA.
- **Deposit** SPL tokens into the treasury's vault.
- **Withdraw** tokens back out, with on-chain checks to prevent unauthorized access or over-withdrawal.
- Attach a **Strategy** to the treasury, defining a strategy type, a risk score, and an allocation percentage.
- **Rebalance** the treasury, which increments a rebalance counter and timestamps the action (the current version is an MVP placeholder — it does not yet route funds to external protocols).

All state changes are validated on-chain using Anchor's account constraints (`has_one`, PDA `seeds`/`bump`, checked arithmetic) so funds can only move with the correct authority's signature.

## Key Features

| Feature | Description |
|---|---|
| **Treasury Initialization** | Creates a unique, per-wallet treasury PDA that tracks total assets, deposits, withdrawals, and rebalance history. |
| **Token Deposits** | Transfers SPL tokens from a user's wallet into the treasury's vault using a CPI (Cross-Program Invocation) to the Token Program. |
| **Token Withdrawals** | Lets only the treasury authority withdraw funds, signed by the treasury PDA itself. |
| **Strategy Creation** | Defines a yield strategy (type, risk score 0–255, allocation percentage 0–100) and links it to the treasury. |
| **Rebalancing** | Updates the treasury's rebalance count and timestamp — the hook point for future automated yield routing. |
| **Safety Checks** | Checked (overflow-safe) arithmetic, authority validation, and allocation bounds checking throughout. |

## Tech Stack

- **[Rust](https://www.rust-lang.org/)** — on-chain program logic
- **[Anchor Framework](https://www.anchor-lang.com/) (v0.30.1)** — Solana program scaffolding, account validation, and IDL generation
- **[anchor-spl](https://docs.rs/anchor-spl)** — SPL Token CPI helpers
- **[Solana](https://solana.com/)** — underlying blockchain / runtime
- **TypeScript** — deployment migration script
- **Rust integration tests** — using `anchor-client`, run via `cargo test`
- **Mocha / Chai / ts-mocha** — JS/TS testing tooling (configured, ready for TS-based tests)
- **Prettier** — code formatting for JS/TS files

## Project Structure

```
YieldMind---Capstone-Project/
├── Anchor.toml                  # Anchor workspace & cluster configuration
├── Cargo.toml                   # Rust workspace manifest
├── package.json                 # Node dependencies & lint scripts
├── tsconfig.json                # TypeScript configuration
├── migrations/
│   └── deploy.ts                # Anchor deploy/migration script
├── programs/
│   └── yieldmind/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # Program entrypoint — declares all instructions
│           ├── constants.rs      # PDA seed constants
│           ├── error.rs          # Custom on-chain error codes
│           ├── instructions/
│           │   ├── initialize.rs # initialize_treasury instruction
│           │   ├── deposit.rs    # deposit instruction
│           │   ├── withdraw.rs   # withdraw instruction
│           │   ├── strategy.rs   # create_strategy instruction
│           │   └── rebalance.rs  # rebalance instruction
│           └── state/
│               ├── treasury.rs   # Treasury account schema
│               └── strategy.rs   # Strategy account schema
└── tests/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── test_initialize.rs    # Rust integration test (anchor-client)
```

## On-Chain Data Accounts

### `Treasury`

Stores the state of a single user's treasury vault.

| Field | Type | Description |
|---|---|---|
| `authority` | `Pubkey` | Wallet that owns and controls this treasury |
| `treasury_bump` | `u8` | PDA bump seed |
| `total_assets` | `u64` | Current token balance tracked by the treasury |
| `total_deposits` | `u64` | Lifetime total deposited |
| `total_withdrawals` | `u64` | Lifetime total withdrawn |
| `rebalance_count` | `u64` | Number of times the treasury has been rebalanced |
| `strategy_pubkey` | `Pubkey` | Linked strategy account (if any) |
| `created_at` | `i64` | Unix timestamp of creation / last rebalance |

**PDA seeds:** `["treasury", authority.key()]`

### `Strategy`

Stores the yield strategy configuration linked to a treasury.

| Field | Type | Description |
|---|---|---|
| `authority` | `Pubkey` | Wallet that owns this strategy |
| `treasury` | `Pubkey` | The treasury this strategy belongs to |
| `strategy_type` | `u8` | Identifier for the strategy type |
| `risk_score` | `u8` | Risk rating (0–255) |
| `allocation_percentage` | `u8` | Percentage of treasury assets allocated (0–100) |
| `active` | `bool` | Whether the strategy is currently active |
| `bump` | `u8` | PDA bump seed |

**PDA seeds:** `["strategy", treasury.key()]`

## Instructions Reference

| Instruction | Signature | Purpose |
|---|---|---|
| `initialize_treasury` | `initialize_treasury(ctx)` | Creates a new Treasury PDA for the signer. |
| `deposit` | `deposit(ctx, amount: u64)` | Transfers `amount` tokens from the user into the treasury vault. |
| `withdraw` | `withdraw(ctx, amount: u64)` | Transfers `amount` tokens from the treasury vault to a destination, signed by the treasury PDA. |
| `create_strategy` | `create_strategy(ctx, strategy_type: u8, risk_score: u8, allocation_percentage: u8)` | Creates and links a Strategy account to a treasury. |
| `rebalance` | `rebalance(ctx)` | Increments the treasury's rebalance counter and updates its timestamp. |

## Prerequisites

Before you begin, install the following on your machine:

1. **Rust** (stable toolchain) — [rustup.rs](https://rustup.rs/)
2. **Solana CLI** — [Solana installation guide](https://docs.solana.com/cli/install-solana-cli-tools)
3. **Anchor CLI** (v0.30.1) — install via [AVM (Anchor Version Manager)](https://www.anchor-lang.com/docs/installation)
4. **Node.js** (v16+ recommended) and **Yarn** or **npm**
5. A local **Solana wallet keypair** (generated via `solana-keygen new`)

Verify your installations:

```bash
rustc --version
solana --version
anchor --version
node --version
```

## Step-by-Step Setup

**1. Clone the repository**

```bash
git clone https://github.com/janmejaisingh4/YieldMind---Capstone-Project.git
cd YieldMind---Capstone-Project
```

**2. Install JavaScript/TypeScript dependencies**

```bash
yarn install
# or
npm install
```

**3. Generate a local Solana wallet (if you don't already have one)**

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

**4. Configure Solana CLI for local development**

```bash
solana config set --url localhost
```

**5. Confirm `Anchor.toml` points to the right cluster**

By default, `Anchor.toml` is set to `Localnet`. Leave it as-is for local development, or update the `[provider]` section to `Devnet`/`Mainnet` when you're ready to deploy elsewhere.

## Building the Program

**1. Start a local validator** (in a separate terminal)

```bash
solana-test-validator
```

**2. Build the Anchor program**

```bash
anchor build
```

This compiles the Rust program in `programs/yieldmind` and generates the IDL (Interface Definition Language) file used by client applications.

**3. Sync the program ID**

After the first build, Anchor generates a new program keypair. Sync the declared ID in `lib.rs` and `Anchor.toml` with the generated keypair:

```bash
anchor keys sync
```

## Running Tests

The repository includes a Rust-based integration test (`tests/src/test_initialize.rs`) that connects to a local validator using `anchor-client`.

**1. Make sure a local validator is running:**

```bash
solana-test-validator
```

**2. Set the `ANCHOR_WALLET` environment variable:**

```bash
export ANCHOR_WALLET=~/.config/solana/id.json
```

**3. Run the tests:**

```bash
cargo test
```

This is also wired up as the Anchor test script in `Anchor.toml`, so you can alternatively run:

```bash
anchor test
```

> **Note:** `anchor test` will spin up and tear down its own local validator automatically unless you configure it to use an already-running one.

## Deploying the Program

**1. Deploy to your configured cluster:**

```bash
anchor deploy
```

**2. (Optional) Run the migration script** to execute any custom post-deploy setup defined in `migrations/deploy.ts`:

```bash
anchor migrate
```

**3. Verify the deployment:**

```bash
solana program show <PROGRAM_ID>
```

Replace `<PROGRAM_ID>` with the value declared via `declare_id!` in `programs/yieldmind/src/lib.rs`.

## Error Codes

| Error | Trigger Condition |
|---|---|
| `Unauthorized` | Caller is not the treasury/strategy authority. |
| `TreasuryAlreadyInitialized` | Attempting to re-initialize an existing treasury. |
| `InvalidStrategy` | Reserved for invalid strategy configuration. |
| `InvalidAllocation` | Allocation percentage exceeds 100, or deposit amount is zero. |
| `InsufficientFunds` | Withdrawal amount exceeds the treasury's tracked assets. |
| `Overflow` | A checked arithmetic operation would overflow/underflow a `u64`. |

## Roadmap

This project is an MVP built for capstone evaluation. Planned future enhancements include:

- [ ] Integrate real external yield sources (lending protocols, liquidity pools) into `rebalance`
- [ ] Add automated/off-chain triggers (cron or keeper bots) for periodic rebalancing
- [ ] Support multiple concurrent strategies per treasury
- [ ] Add TypeScript-based end-to-end tests alongside the existing Rust tests
- [ ] Add an on-chain fee mechanism and treasury performance analytics

## Contributing

Contributions, issues, and feature requests are welcome.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m "Add your feature"`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request

Please run `yarn lint` before submitting changes to keep formatting consistent.

---

**Author:** [janmejaisingh4](https://github.com/janmejaisingh4)