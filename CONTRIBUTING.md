# Contributing to cora-code

First off — thank you for considering contributing to **cora-code**! 🎉
This guide will help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

## Code of Conduct

Be respectful, constructive, and inclusive. We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Getting Started

### Prerequisites

- **Rust** 1.85+ (stable toolchain recommended)
- **Git** for version control
- An **LLM API key** (OpenAI, Anthropic, etc.) for testing review features

### Setup

```bash
# Clone the repository
git clone https://github.com/codecoradev/cora-code.git
cd cora-code

# Build in debug mode
cargo build

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings
```

### Project Structure

```
cora-code/
├── src/
│   ├── main.rs              # CLI entry point + clap args
│   ├── cli.rs               # Argument parsing (clap)
│   ├── commands/            # CLI subcommands
│   │   ├── review.rs       # cora review
│   │   ├── scan.rs         # cora scan
│   │   ├── config_cmd.rs   # cora config show/set
│   │   ├── auth.rs         # cora auth login/logout
│   │   └── init.rs         # cora init
│   ├── config/
│   │   ├── loader.rs       # Config resolution chain (global → project → env → CLI)
│   │   ├── schema.rs       # YAML config structs + merge logic
│   │   └── providers.rs    # Provider presets (OpenAI, Anthropic, etc.)
│   ├── engine/
│   │   ├── review.rs       # LLM review + SARIF generation
│   │   ├── scanner.rs      # File scanning and diff generation
│   │   ├── types.rs        # Issue, Severity, ScanResponse types
│   │   └── llm.rs          # LLM API abstraction
│   └── formatter/
│       └── mod.rs          # Output formatting (pretty, compact, json, sarif)
├── .github/
│   └── workflows/          # CI, release, deploy workflows
├── tests/                   # Integration tests
├── Cargo.toml
└── README.md
```

## Development Workflow

1. **Fork** the repository on GitHub
2. **Create a branch** for your change:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** and commit with meaningful messages:
   ```bash
   git commit -m "feat: add support for SARIF output format"
   ```
4. **Push** to your fork and open a **Pull Request**

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

| Prefix     | Purpose                          |
|------------|----------------------------------|
| `feat:`    | New feature                      |
| `fix:`     | Bug fix                          |
| `docs:`    | Documentation changes            |
| `test:`    | Adding or updating tests         |
| `refactor:`| Code changes without feature/fix |
| `chore:`   | Maintenance tasks                |
| `ci:`      | CI/CD changes                    |

## Pull Request Process

1. Update documentation if your change affects user-facing behavior
2. Add tests for any new functionality
3. Ensure `cargo test` passes
4. Ensure `cargo clippy -- -D warnings` is clean
5. Ensure `cargo fmt` has been applied
6. Keep PRs focused — one logical change per PR

## Coding Standards

- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Run `cargo clippy -- -D warnings` — no warnings allowed
- **Errors**: Use `anyhow` for application errors, define custom error types for library code
- **Testing**: Write unit tests for core logic, integration tests for CLI commands
- **Documentation**: Add doc comments to all public items (`///`)

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_from_str() {
        use crate::engine::types::Severity;
        assert_eq!(Severity::from_str_lossy("critical"), Severity::Critical);
        assert_eq!(Severity::from_str_lossy("info"), Severity::Info);
    }
}
```

## Reporting Bugs

Please open a [GitHub Issue](https://github.com/codecoradev/cora-code/issues/new) with:

- **Description** — What happened vs. what you expected
- **Steps to reproduce** — Minimal reproduction steps
- **Environment** — OS, Rust version, cora-code version
- **Logs** — Output with `RUST_LOG=debug cora review`

## Feature Requests

We love hearing your ideas! Open an issue with:

- **Problem** — What problem does this solve?
- **Proposed solution** — How should it work?
- **Alternatives considered** — Other approaches you thought about

## Questions?

Feel free to open an issue tagged with `question` or reach out on GitHub Discussions.

---

Thank you for helping make cora-code better! 🦀
