# 🌉 Soroban-SQL-Sync: A High-Performance Bridge Between Stellar/Soroban and Relational Databases

`STELLAR` `SOROBAN` `RUST` `POSTGRESQL` `MIT`

**Soroban-SQL-Sync** is a modular, performance-first infrastructure tool designed to solve the data availability gap in the Stellar ecosystem. By indexing smart contract events and storing them in a relational database, developers get instant access to historical data, complex queries, and real-time analytics—without hitting RPC rate limits or waiting for slow sequential calls.

## 🛠️ Why Soroban-SQL-Sync?

While smart contracts excel at execution, querying historical event data directly from an RPC node is inefficient for production-grade dApps. This project provides a robust solution for developers who need a structured, SQL-ready mirror of their smart contract's state and history, enabling:

- **Complex Queries & Aggregations** – Full SQL power for analytics and reporting
- **Real-Time Data Visualization** – Dashboards and monitoring without RPC delays
- **Production-Grade Data Availability** – Type-safe, consistent data with reorg resilience

## 🏗️ Technical Architecture

Built with a senior-level backend stack optimized for high-throughput and data integrity:

- **Language:** Rust 🦀
- **Concurrency:** **Tokio async runtime** for non-blocking, high-performance I/O
- **Persistence:** **sqlx** for type-safe, compile-time checked PostgreSQL interactions
- **Protocol:** Direct integration with Stellar Horizon and Soroban RPC nodes

## ✨ Core Features

- **Live Event Indexing** – Catch and decode custom contract events in real-time
- **Relational Storage** – Map raw XDR data into clean, queryable PostgreSQL tables
- **Reorg Resilience** – Gracefully handle ledger rollbacks and maintain data consistency
- **Developer CLI** – Manage sync offsets, run migrations, and monitor health
- **Type-Safe** – Compile-time checked database queries prevent runtime errors

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
1. **Fork the Project** – Create your own copy of the repository
2. **Create your Feature Branch** – Use `git checkout -b feature/YourFeatureName` to work on a new feature
3. **Commit your Changes** – Use `git commit -m "Add YourFeature"`
4. **Push to the Branch** – Use `git push origin feature/YourFeatureName`
5. **Open a Pull Request** – Submit your PR with a clear description

For detailed contribution guidelines, see [CONTRIBUTING.md](./CONTRIBUTING.md).

**Additional Guidelines:**
- Check [GitHub Issues](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues) for current milestones
- Link PRs to issues with "Closes #N" in commit messages
- Include proof screenshots (build output, test results, migrations)
- Follow Phase 1 → 2 → 3 progression for maximum impact

-----
