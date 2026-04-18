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

### Installation & Local Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/ExcelDsigN-tech/soroban-sql-sync.git
    cd soroban-sql-sync
    ```

2.  **Configure Environment:**
    ```bash
    cp .env.example .env
    # Edit .env with your PostgreSQL and Stellar RPC credentials
    ```

3.  **Install Dependencies:**
    ```bash
    cargo check
    ```

4.  **Set Up Database:**
    ```bash
    # Install sqlx-cli if needed
    cargo install sqlx-cli --no-default-features --features postgres
    
    # Create database
    createdb soroban_sql_sync
    
    # Run migrations
    sqlx migrate run
    ```

5.  **Build and Run:**
    ```bash
    cargo build --release
    cargo run
    ```

## 📋 Project Roadmap

See [ROADMAP.md](./ROADMAP.md) for detailed phase breakdown and [GitHub Issues](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues) for current work.

### Phase 1: Foundation (In Progress)
- Core project architecture and dependency setup
- PostgreSQL schema design and migrations
- Smart contract event schema specification

See [PHASE_1_LAUNCH.md](./PHASE_1_LAUNCH.md) for deployment instructions.

## ⚖️ License

This project is licensed under the **MIT License** - see the [LICENSE](./LICENSE) file for details.

## 🤝 Contribution & Stellar Wave

This project is part of **Stellar Wave** initiative. We welcome contributions from the community.

**How to Contribute:**
- Check [GitHub Issues](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues) for current milestones
- Link PRs to issues with "Closes #N" in commit messages
- Include proof screenshots (build output, test results, migrations)
- Follow Phase 1 → 2 → 3 progression for maximum impact

-----
