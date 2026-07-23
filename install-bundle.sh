#!/usr/bin/env sh
#
# cora code review — standalone installer
# Installs Cora (AI code review) in one command.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-code/main/install-bundle.sh | sh
#
# With version pinning:
#   CORA_VERSION=v0.5.1 curl -fsSL ... | sh

set -e

# Colors
if [ -t 1 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    DIM='\033[2m'
    NC='\033[0m'
else
    RED='' GREEN='' YELLOW='' CYAN='' BOLD='' DIM='' NC=''
fi

info()  { printf "${CYAN}[INFO]${NC} %s\n" "$1"; }
ok()    { printf "${GREEN}[OK]${NC} %s\n" "$1"; }
warn()  { printf "${YELLOW}[WARN]${NC} %s\n" "$1"; }
error() { printf "${RED}[ERROR]${NC} %s\n" "$1"; exit 1; }

BINARY_DIR="${HOME}/.local/bin"

# Ensure binary dir exists
mkdir -p "$BINARY_DIR"

# ─── Install Cora ───
install_cora() {
    info "Installing Cora — AI code review CLI..."

    if command -v cora >/dev/null 2>&1; then
        EXISTING=$(cora --version 2>/dev/null | head -1 || echo "unknown")
        info "Cora already installed: $EXISTING"
        if [ -z "${CORA_FORCE:-}" ]; then
            info "Skipping (set CORA_FORCE=1 to reinstall)"
            return 0
        fi
    fi

    curl -fsSL "https://raw.githubusercontent.com/codecoradev/cora-code/main/install.sh" | sh
    ok "Cora installed: $(cora --version 2>/dev/null | head -1)"
}

# ─── Verify ───
verify() {
    info "Verifying installation..."

    CORA_OK="no"

    if command -v cora >/dev/null 2>&1; then
        CORA_VER=$(cora --version 2>/dev/null | head -1)
        CORA_OK="yes"
        ok "Cora: $CORA_VER"
    else
        warn "Cora not found in PATH"
    fi

    echo ""
    printf "${BOLD}━━━ Setup Complete ━━━${NC}\n"
    echo ""

    if [ "$CORA_OK" = "yes" ]; then
        printf "${GREEN}Cora installed!${NC} You're ready for AI code review.\n\n"
        echo "  ${BOLD}Quick start:${NC}"
        echo "    cora auth login          # Set your API key"
        echo "    cora review --staged     # Review staged changes"
        echo "    cora commit              # Review + auto commit message"
    else
        error "Cora was not installed successfully."
    fi

    # PATH check
    case ":$PATH:" in
        *":$BINARY_DIR:"*) ;;
        *)
            echo ""
            warn "Add $BINARY_DIR to your PATH:"
            echo "    echo 'export PATH=\"$BINARY_DIR:\$PATH\"' >> ~/.bashrc"
            echo "    source ~/.bashrc"
            ;;
    esac
}

# ─── Main ───
echo ""
printf "${BOLD}${CYAN}╔══════════════════════════════════════╗${NC}\n"
printf "${BOLD}${CYAN}║  Cora — AI Code Review CLI             ║${NC}\n"
printf "${BOLD}${CYAN}║  Fast. Deterministic. Self-hosted.     ║${NC}\n"
printf "${BOLD}${CYAN}╚══════════════════════════════════════╝${NC}\n"
echo ""

install_cora
echo ""
verify
echo ""
