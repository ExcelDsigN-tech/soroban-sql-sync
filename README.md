# ⚡ Soroban-SQL-Sync

**A high-performance bridge between Stellar/Soroban and Relational Databases.**

## 🌟 Project Overview

**Soroban-SQL-Sync** is a modular, performance-first infrastructure tool designed to solve the data availability gap in the Stellar ecosystem. While smart contracts excel at execution, querying historical event data directly from an RPC node is inefficient for production-grade dApps.

This project provides a robust solution for developers who need a structured, SQL-ready mirror of their smart contract's state and history, allowing for complex queries, aggregations, and real-time data visualization.

## 🛠️ Technical Architecture

Built with a senior-level backend stack, this indexer is optimized for high-throughput and data integrity:

  * **Language:** Rust 🦀
  * **Concurrency:** Powered by the **Tokio async runtime** for non-blocking I/O.
  * **Persistence:** Leveraging **sqlx** for type-safe, compile-time checked PostgreSQL interactions.
  * **Protocol:** Directly interfaces with the Stellar Horizon and Soroban RPC nodes.

## 🚀 Key Features

  - **Live Event Indexing:** Catch and decode custom contract events (e.g., from SoroSusu or Stellar-Fluid).
  - **Relational Storage:** Map raw XDR data into clean PostgreSQL tables.
  - **Reorg Resilience:** Handles ledger rollbacks to maintain data consistency.
  - **Developer CLI:** Manage sync offsets, database migrations, and health monitoring.

## 📦 Getting Started

### Prerequisites

  - Rust (latest stable)
  - PostgreSQL 14+
  - A Stellar/Soroban RPC endpoint

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/ExcelDsigN-tech/soroban-sql-sync.git
    cd soroban-sql-sync
    ```
2.  **Configure Environment:**
    Create a `.env` file (this is ignored by git for security):
    ```text
    DATABASE_URL=postgres://user:password@localhost/soroban_db
    RPC_URL=https://soroban-testnet.stellar.org:443
    NETWORK_PASSPHRASE=Test SDF Network ; September 2015
    ```
3.  **Build the project:**
    ```bash
    cargo build --release
    ```

## ⚖️ License

This project is licensed under the **MIT License** - see the [LICENSE](./LICENSE) file for details.

## 🤝 Contribution & Wave 4

This project is part of **Stellar Wave 4**. We welcome contributions from the community. Please check the **Issues** tab for current milestones and tasks.

-----
