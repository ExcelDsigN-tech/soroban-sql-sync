# 🤝 Contributing to Soroban-SQL-Sync

Thank you for your interest in contributing to **Soroban-SQL-Sync**! This document outlines the process for contributing to our project.

## 📋 Before You Start

- Ensure you have [Rust](https://www.rust-lang.org/) and [PostgreSQL](https://www.postgresql.org/) installed
- Have a GitHub account and [Git](https://git-scm.com/) configured locally
- Review [ROADMAP.md](./ROADMAP.md) to understand the project phases and priorities

## 🚀 Contribution Workflow

### Step 1: Fork the Project

Click the **Fork** button in the top-right corner of the repository to create your own copy.

```bash
# Clone your forked repository
git clone https://github.com/YOUR-USERNAME/soroban-sql-sync.git
cd soroban-sql-sync
```

### Step 2: Create Your Feature Branch

**Always create a new branch for your work.** This keeps your changes isolated and makes the review process smoother.

```bash
git checkout -b feature/YourFeatureName
```

**Branch naming conventions:**
- `feature/ComponentName` – New features or enhancements
- `fix/BugDescription` – Bug fixes
- `docs/DocumentationName` – Documentation updates
- `refactor/ComponentName` – Code refactoring or optimization

### Step 3: Make Your Changes

- Write clean, idiomatic Rust code
- Follow the existing code style and patterns in the repository
- Add tests for new functionality
- Update documentation as needed

### Step 4: Commit Your Changes

Write clear, descriptive commit messages:

```bash
git commit -m "Add YourFeature: brief description of changes"
```

**Good commit message format:**
- Start with a clear verb (Add, Fix, Update, Refactor, etc.)
- Be concise but descriptive
- Reference related issues if applicable: `Closes #123`

Examples:
```bash
git commit -m "Add event decoder for custom contract types"
git commit -m "Fix database migration ordering issue"
git commit -m "Update README with setup instructions"
```

### Step 5: Push to Your Branch

```bash
git push origin feature/YourFeatureName
```

### Step 6: Open a Pull Request

1. Navigate to the original repository on GitHub
2. Click **New Pull Request**
3. Select your branch to compare against `main`
4. Provide a clear title and description of your changes
5. Link to any related issues using `Closes #N`
6. Submit the PR for review

## ✅ PR Guidelines

When submitting a pull request:

- **Describe what you changed** – Explain the purpose and scope of your changes
- **Link to issues** – Use `Closes #N` format to auto-close related issues
- **Include proof** – Add screenshots or test output showing your changes work
- **Keep it focused** – Avoid combining unrelated changes in a single PR
- **Test locally** – Ensure the code builds and tests pass before pushing

Example PR description:
```
## Description
Implements XDR event decoding for custom contract events.

## Related Issue
Closes #42

## Changes
- Added decoder module in `src/ingest/decoder.rs`
- Implements XDR parsing for events
- Added unit tests for decoder functions

## Testing
```
[Screenshot of passing tests]
```
```

## 🔄 Code Review Process

Our maintainers will review your PR:
- We may request changes or ask clarifying questions
- Please respond promptly to feedback
- Approval requires at least one maintainer sign-off
- After approval, we'll merge your PR and credit your contribution

## 📝 Code Standards

### Rust Code
- Use `cargo fmt` for formatting: `cargo fmt`
- Run `cargo clippy` for linting: `cargo clippy`
- Write tests for new functionality
- Add documentation comments for public APIs

### Commit Guidelines
- One logical change per commit
- Rebase and squash commits if needed before merging
- Use meaningful commit messages

## 🐛 Reporting Issues

If you find a bug or have a feature request:
1. Check [existing issues](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues)
2. If not found, create a new issue with:
   - Clear title and description
   - Steps to reproduce (for bugs)
   - Expected vs. actual behavior
   - Environment details (OS, Rust version, PostgreSQL version)

## 📊 Contribution Areas

We welcome contributions in these areas:

- **Backend (Rust)** – Core indexing logic, API endpoints, database interactions
- **Database** – Schema design, migrations, query optimization
- **Blockchain Integration** – Soroban RPC integration, event decoding
- **Documentation** – README, guides, API documentation
- **Testing** – Unit tests, integration tests, test coverage
- **DevEx** – Build scripts, CLI tooling, developer experience

See [PHASE_1_LAUNCH.md](./PHASE_1_LAUNCH.md) and [ROADMAP.md](./ROADMAP.md) for current focus areas.

## ❓ Questions?

- Review existing [GitHub Issues](https://github.com/ExcelDsigN-tech/soroban-sql-sync/issues)
- Check the [Project README](./README.md)
- Ask in a new issue with the `question` label

---

**Thank you for contributing to Soroban-SQL-Sync! 🚀**
