---
title: Installation
---

# Installation

## Quick Install (Recommended)

The fastest way to get cora — single command, no Rust required:

```bash
$ curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
```

Installs to `~/.local/bin`. Add to PATH if needed:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

Pin a specific version:

```bash
$ CORA_VERSION=v0.6.1 curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
```

## Install via Cargo

If you have Rust 1.85+ installed:

```bash
$ cargo install cora-cli
```

This compiles cora from source and installs it to Cargo's binary directory (typically `~/.cargo/bin/`).

## Download Binary

Pre-built binaries are available from the [GitHub Releases](https://github.com/codecoradev/cora-cli/releases) page.

Supported platforms:

- Linux x86_64 (glibc)
- Linux arm64 (aarch64)
- macOS arm64 (Apple Silicon)
- Windows x86_64

```bash
# Download and extract
$ curl -sL https://github.com/codecoradev/cora-cli/releases/latest/download/cora-linux-x86_64.tar.gz | tar xz
$ mv cora ~/.local/bin/cora
```

## Build from Source

If you prefer to build from the latest source:

```bash
$ git clone https://github.com/codecoradev/cora-cli.git
$ cd cora-cli
$ cargo build --release
# Binary at target/release/cora
```

## Shell Completions

cora provides shell completions for bash, zsh, and fish:

```bash
# Bash
$ cora completion bash > ~/.cora/completion.bash
$ echo 'source ~/.cora/completion.bash' >> ~/.bashrc

# Zsh
$ cora completion zsh > ~/.cora/completion.zsh
$ echo 'source ~/.cora/completion.zsh' >> ~/.zshrc

# Fish
$ cora completion fish > ~/.config/fish/completions/cora.fish
```

## Verify Installation

Confirm cora is installed correctly:

```bash
$ cora --version
cora 0.6.1

$ cora auth status
Provider: openai
API key: configured
```

### Check for stale copies on PATH

cora is distributed through multiple channels (installer script, `cargo`, pre-built binaries). If you have more than one installed, `which cora` resolves to whichever appears first in `$PATH` — which may silently be a stale version.

```bash
# List every `cora` on your PATH (one entry = healthy)
$ which -a cora
/Users/you/.local/bin/cora

# Should match the latest release
$ cora --version
cora 0.6.1
```

If `which -a cora` shows more than one path (e.g. `~/.local/bin/cora` and `~/.cargo/bin/cora`), remove the one you don't want or reorder your `PATH`. See [Issue #314](https://github.com/codecoradev/cora-cli/issues/314) for background.

## macOS: `Killed: 9` on launch?

Prebuilt macOS binaries (`aarch64-apple-darwin`) are not Apple-notarized. When downloaded directly (via browser, `curl`, or `gh release download`), macOS attaches `com.apple.quarantine` / `com.apple.provenance` extended attributes and kills the binary on first launch with `Killed: 9` and **no error message**.

The `install.sh` installer strips these attributes automatically. If you downloaded the binary manually, strip them yourself:

```bash
$ xattr -dr com.apple.quarantine /path/to/cora
$ xattr -dr com.apple.provenance /path/to/cora
```

Or install via `cargo` / Homebrew to sidestep Gatekeeper entirely.

## Updating

To update cora to the latest version:

| Method | Command |
|--------|---------|
| Via Cargo | `cargo install cora-cli --force` |
| Via Binary | Download the latest release from GitHub and replace the existing binary |
