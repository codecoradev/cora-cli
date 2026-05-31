# Contributing to cora-cli

First off — thank you for considering contributing to **cora-cli**! 🎉
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

- **Rust 1.85+** (stable toolchain recommended)
- **Git** for version control
- An **LLM API key** (OpenAI, Anthropic, etc.) for testing review features

### Setup

```bash
# Clone the repository
git clone https://github.com/ajianaz/cora-cli.git
cd cora-cli

# Build in debug mode
cargo build

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings
```

### Project Structure

```
cora-cli/
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── commands/                # CLI subcommand handlers
│   │   ├── mod.rs
│   │   ├── review.rs            # cora review
│   │   ├── scan.rs              # cora scan
│   │   ├── upload.rs            # cora upload-sarif
│   │   ├── auth.rs              # cora auth
│   │   ├── hook_cmd.rs          # cora hook
│   │   ├── init.rs              # cora init
│   │   ├── completion.rs        # cora completion
│   │   └── providers.rs         # cora providers
│   ├── config/                  # Configuration loading & schema
│   │   ├── mod.rs
│   │   ├── schema.rs            # Config struct definitions
│   │   ├── loader.rs            # Config file discovery & loading
│   │   └── providers.rs         # Provider auto-detection
│   ├── engine/                  # Core review/scanning engine
│   │   ├── mod.rs
│   │   ├── llm.rs               # LLM API client
│   │   ├── review.rs            # Diff review logic
│   │   ├── scanner.rs           # Project scanning logic
│   │   └── types.rs             # Shared types (Severity, Findings, etc.)
│   ├── formatters/              # Output formatting
│   │   ├── mod.rs
│   │   ├── pretty.rs            # Pretty-printed terminal output
│   │   ├── compact.rs           # Compact single-line output
│   │   ├── json_fmt.rs          # JSON output
│   │   └── sarif.rs             # SARIF output
│   ├── git/                     # Git operations
│   │   ├── mod.rs
│   │   ├── diff.rs              # Diff generation
│   │   └── files.rs             # File discovery
│   └── hook/                    # Git hook management
│       ├── mod.rs
│       ├── install.rs           # Hook install/uninstall
│       └── template.rs          # Hook script template
├── tests/                       # Integration tests
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
7. **CI will automatically run cora on your PR** — all findings must be addressed or the PR will be blocked

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
    fn test_scan_single_file() {
        let result = scan_files(vec!["src/main.rs".into()]);
        assert!(!result.is_empty());
    }
}
```

## Reporting Bugs

Please open a [GitHub Issue](https://github.com/ajianaz/cora-cli/issues/new) with:

- **Description** — What happened vs. what you expected
- **Steps to reproduce** — Minimal reproduction steps
- **Environment** — OS, Rust version, cora-cli version (`cora --version`)
- **Logs** — Output with `cora --verbose review`

## Feature Requests

We love hearing your ideas! Open an issue with:

- **Problem** — What problem does this solve?
- **Proposed solution** — How should it work?
- **Alternatives considered** — Other approaches you thought about

## Questions?

Feel free to open an issue tagged with `question` or reach out on GitHub Discussions.

---

Thank you for helping make cora-cli better! 🦀
