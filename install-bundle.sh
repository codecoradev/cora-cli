#!/usr/bin/env sh
#
# cora + uteke bundle installer
# Installs both Cora (AI code review) and Uteke (AI memory) in one command.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-code/main/install-bundle.sh | sh
#
# With version pinning:
#   CORA_VERSION=v0.5.1 UTEKE_VERSION=v0.1.0 curl -fsSL ... | sh

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

# ─── Install Uteke ───
install_uteke() {
    info "Installing Uteke — AI memory engine..."

    if command -v uteke >/dev/null 2>&1; then
        EXISTING=$(uteke --version 2>/dev/null | head -1 || echo "unknown")
        info "Uteke already installed: $EXISTING"
        if [ -z "${UTEKE_FORCE:-}" ]; then
            info "Skipping (set UTEKE_FORCE=1 to reinstall)"
            return 0
        fi
    fi

    curl -fsSL "https://raw.githubusercontent.com/codecoradev/uteke/main/install.sh" | sh
    ok "Uteke installed: $(uteke --version 2>/dev/null | head -1)"
}

# ─── Verify ───
verify() {
    info "Verifying installation..."

    CORA_OK="no"
    UTEKE_OK="no"

    if command -v cora >/dev/null 2>&1; then
        CORA_VER=$(cora --version 2>/dev/null | head -1)
        CORA_OK="yes"
        ok "Cora: $CORA_VER"
    else
        warn "Cora not found in PATH"
    fi

    if command -v uteke >/dev/null 2>&1; then
        UTEKE_VER=$(uteke --version 2>/dev/null | head -1)
        UTEKE_OK="yes"
        ok "Uteke: $UTEKE_VER"
    else
        warn "Uteke not found in PATH"
    fi

    echo ""
    printf "${BOLD}━━━ Setup Complete ━━━${NC}\n"
    echo ""

    if [ "$CORA_OK" = "yes" ] && [ "$UTEKE_OK" = "yes" ]; then
        printf "${GREEN}Both tools installed!${NC} You now have AI code review with memory.\n\n"
        echo "  ${BOLD}Quick start:${NC}"
        echo "    cora auth login          # Set your API key"
        echo "    cora review --staged     # Review staged changes"
        echo "    cora review --memory     # Review with project memory"
        echo "    cora commit              # Review + auto commit message"
        echo ""
        echo "  ${BOLD}Memory workflow:${NC}"
        echo "    cora review --memory --learn   # Review + save to memory"
        echo "    uteke stats                     # Check memory stats"
        echo ""
        printf "  ${DIM}Your code review gets smarter every sprint.${NC}\n"
    elif [ "$CORA_OK" = "yes" ]; then
        printf "${YELLOW}Cora installed.${NC} Install Uteke separately for memory features:\n"
        echo "    curl -fsSL https://raw.githubusercontent.com/codecoradev/uteke/main/install.sh | sh"
    elif [ "$UTEKE_OK" = "yes" ]; then
        printf "${YELLOW}Uteke installed.${NC} Install Cora separately for code review:\n"
        echo "    curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-code/main/install.sh | sh"
    else
        error "Neither tool was installed successfully."
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
printf "${BOLD}${CYAN}╔══════════════════════════════════════════════╗${NC}\n"
printf "${BOLD}${CYAN}║  CodeCoraDev Bundle — Cora + Uteke          ║${NC}\n"
printf "${BOLD}${CYAN}║  AI code review that remembers and learns   ║${NC}\n"
printf "${BOLD}${CYAN}╚══════════════════════════════════════════════╝${NC}\n"
echo ""

install_cora
echo ""
install_uteke
echo ""
verify
echo ""
