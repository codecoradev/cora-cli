#!/usr/bin/env sh
#
# cora code review — standalone installer
# Installs Cora (AI code review) in one command.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-code/main/install-bundle.sh | sh
#
set -eu

# ── Config ──────────────────────────────────────────────────────────
REPO_OWNER="codecoradev"
REPO_NAME="cora-code"
BINARY_NAME="cora"
INSTALL_DIR="$HOME/.local/bin"

# Detect platform
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux*)  OS="linux" ;;
        Darwin*) OS="macos" ;;
        *)       echo "Unsupported OS: $OS"; exit 1 ;;
    esac

    case "$ARCH" in
        x86_64|amd64) ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *)             echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac

    PLATFORM="${OS}-${ARCH}"
}

# Get latest release tag from GitHub
get_latest_version() {
    VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" \
        | grep '"tag_name"' \
        | head -1 \
        | sed -E 's/.*"([^"]+)".*/\1/')"

    if [ -z "$VERSION" ]; then
        # Fallback: try CORA_VERSION env var
        VERSION="${CORA_VERSION:-}"
        if [ -z "$VERSION" ]; then
            echo "Error: Could not determine latest version."
            echo "Set CORA_VERSION env var to install a specific version:"
            echo "  CORA_VERSION=v0.6.1 $0"
            exit 1
        fi
    fi
    echo "$VERSION"
}

# Download and install
install_binary() {
    VERSION="$1"

    echo "cora installer"
    echo "  Version:   ${VERSION}"
    echo "  Platform:  ${PLATFORM}"
    echo "  Install to: ${INSTALL_DIR}/${BINARY_NAME}"
    echo ""

    # Create install directory
    mkdir -p "$INSTALL_DIR"

    # Download URL
    URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${VERSION}/${BINARY_NAME}-${PLATFORM}"

    echo "Downloading ${BINARY_NAME} ${VERSION}..."
    curl -fsSL "$URL" -o "${INSTALL_DIR}/${BINARY_NAME}"

    # Make executable
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    # Strip quarantine attributes on macOS
    if [ "$(uname -s)" = "Darwin" ]; then
        xattr -dr com.apple.quarantine "${INSTALL_DIR}/${BINARY_NAME}" 2>/dev/null || true
        xattr -dr com.apple.provenance "${INSTALL_DIR}/${BINARY_NAME}" 2>/dev/null || true
    fi

    echo ""
    echo "✅ ${BINARY_NAME} ${VERSION} installed to ${INSTALL_DIR}/${BINARY_NAME}"
    echo ""
    echo "Verify:"
    echo "  ${INSTALL_DIR}/${BINARY_NAME} --version"
    echo ""
}

# Verify installation
verify_install() {
    if ! command -v "$BINARY_NAME" >/dev/null 2>&1; then
        echo "⚠️  ${BINARY_NAME} installed but not on PATH."
        echo "   Add this to your shell profile:"
        echo ""
        echo "     export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
    else
        VERSION="$("${BINARY_NAME}" --version 2>/dev/null || echo "unknown")"
        echo "✅ $(which "$BINARY_NAME") — ${VERSION}"
    fi
}

# ── Main ───────────────────────────────────────────────────────────
main() {
    detect_platform
    VERSION="$(get_latest_version)"
    install_binary "$VERSION"
    verify_install
}

main "$@"
