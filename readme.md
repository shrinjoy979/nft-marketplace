# NFT Marketplace (Solana + Rust)

A decentralized NFT marketplace built on the Solana blockchain using Rust.  
The program allows users to list NFTs for sale, purchase listed NFTs, and manage marketplace transactions securely on-chain.

---

## Features

- List NFTs for sale
- Buy NFTs using SOL
- Secure on-chain transactions
- Ownership transfer after purchase
- Marketplace listing management

---

## Built With

- Rust
- Solana Program Library
- Solana CLI
- Cargo
- SPL Token / Token Metadata (for NFTs)

---

## Installation & Setup

### 1. Install Solana CLI

Follow the official guide:

https://docs.solana.com/cli/install-solana-cli-tools

Verify installation:

```bash
solana --version
```

---

### 2. Clone the repository

```bash
git clone https://github.com/your-username/nft-marketplace.git
cd nft-marketplace
```

---

### 3. Build the program

```bash
cargo build-bpf
```

---

### 4. Deploy the program

```bash
solana program deploy target/deploy/nft_marketplace.so
```

---

## How It Works

1. A user lists an NFT for sale on the marketplace.
2. The NFT is locked in a marketplace escrow account.
3. Another user can purchase the NFT by paying the specified price.
4. The NFT is transferred to the buyer.
5. The seller receives the payment.

---

## Project Structure

```
nft-marketplace/
│
├── src/
│   ├── lib.rs
│   ├── instruction.rs
│   ├── processor.rs
│   └── state.rs
│
├── Cargo.toml
└── README.md
```

---

## Concepts Used

- Solana Accounts
- Program Instructions
- NFT Ownership Transfer
- Escrow Logic
- Cross Program Invocation (CPI)
- SPL Token Program Interaction

---

## Future Improvements

- Add royalty support
- Add marketplace fees
- Support multiple NFT collections
- Build a frontend UI
- Add tests and security checks

---

## Contributing

Pull requests are welcome.  
Feel free to fork the project and improve it.
