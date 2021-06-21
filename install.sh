#!/usr/bin/env sh

# Options
#
#   -V, --verbose
#     Enable verbose output for the installer
#
#   -f, -y, --force, --yes
#     Skip the confirmation prompt during installation

TERMSCP_VERSION="0.6.0"
GITHUB_URL="https://github.com/veeso/termscp/releases/download/v${TERMSCP_VERSION}"
DEB_URL="${GITHUB_URL}/termscp_${TERMSCP_VERSION}_amd64.deb"
FREEBSD_URL="${GITHUB_URL}/termscp-${TERMSCP_VERSION}.txz"
RPM_URL="${GITHUB_URL}/termscp-${TERMSCP_VERSION}-1.x86_64.rpm"

set -eu
printf "\n"

BOLD="$(tput bold 2>/dev/null || printf '')"
GREY="$(tput setaf 0 2>/dev/null || printf '')"
UNDERLINE="$(tput smul 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
BLUE="$(tput setaf 4 2>/dev/null || printf '')"
MAGENTA="$(tput setaf 5 2>/dev/null || printf '')"
NO_COLOR="$(tput sgr0 2>/dev/null || printf '')"

# Functions

info() {
    printf '%s\n' "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
    printf '%s\n' "${YELLOW}! $*${NO_COLOR}"
}

error() {
    printf '%s\n' "${RED}x $*${NO_COLOR}" >&2
}

completed() {
    printf '%s\n' "${GREEN}✓${NO_COLOR} $*"
}

has() {
    command -v "$1" 1>/dev/null 2>&1
}

get_tmpfile() {
    local suffix
    suffix="$1"
    if has mktemp; then
        printf "%s.%s" "$(mktemp)" "${suffix}"
    else
        # No really good options here--let's pick a default + hope
        printf "/tmp/termscp.%s" "${suffix}"
    fi
}

download() {
    output="$1"
    url="$2"
    
    if has curl; then
        cmd="curl --fail --silent --location --output $output $url"
    elif has wget; then
        cmd="wget --quiet --output-document=$output $url"
    elif has fetch; then
        cmd="fetch --quiet --output=$output $url"
    else
        error "No HTTP download program (curl, wget, fetch) found, exiting…"
        return 1
    fi
    $cmd && return 0 || rc=$?
    
    error "Command failed (exit code $rc): ${BLUE}${cmd}${NO_COLOR}"
    warn "If you believe this is a bug, please report immediately an issue to <https://github.com/veeso/termscp/issues/new>"
    return $rc
}

elevate_priv() {
    if ! has sudo; then
        error 'Could not find the command "sudo", needed to install termscp on your system.'
        info "If you are on Windows, please run your shell as an administrator, then"
        info "rerun this script. Otherwise, please run this script as root, or install"
        info "sudo."
        exit 1
    fi
    if ! sudo -v; then
        error "Superuser not granted, aborting installation"
        exit 1
    fi
}

test_writeable() {
  local path
  path="${1:-}/test.txt"
  if touch "${path}" 2>/dev/null; then
    rm "${path}"
    return 0
  else
    return 1
  fi
}

# Currently supporting:
#   - macos
#   - linux
#   - freebsd
detect_platform() {
    local platform
    platform="$(uname -s | tr '[:upper:]' '[:lower:]')"
    
    case "${platform}" in
        linux) platform="linux" ;;
        darwin) platform="macos" ;;
        freebsd) platform="freebsd" ;;
    esac
    
    printf '%s' "${platform}"
}

# Currently supporting:
#   - x86_64
detect_arch() {
    local arch
    arch="$(uname -m | tr '[:upper:]' '[:lower:]')"
    
    case "${arch}" in
        amd64) arch="x86_64" ;;
        armv*) arch="arm" ;;
        arm64) arch="aarch64" ;;
    esac
    
    # `uname -m` in some cases mis-reports 32-bit OS as 64-bit, so double check
    if [ "${arch}" = "x86_64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
        arch="i686"
    elif [ "${arch}" = "aarch64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
        arch="arm"
    fi
    
    if [ "${arch}" != "x86_64" ]; then
        error "Unsupported arch ${arch}"
        return 1
    fi
    
    printf '%s' "${arch}"
}

confirm() {
    if [ -z "${FORCE-}" ]; then
        printf "%s " "${MAGENTA}?${NO_COLOR} $* ${BOLD}[y/N]${NO_COLOR}"
        set +e
        read -r yn </dev/tty
        rc=$?
        set -e
        if [ $rc -ne 0 ]; then
            error "Error reading from prompt (please re-run with the '--yes' option)"
            exit 1
        fi
        if [ "$yn" != "y" ] && [ "$yn" != "yes" ]; then
            error 'Aborting (please answer "yes" to continue)'
            exit 1
        fi
    fi
}

# Installers

install_on_bsd() {
    info "Installing termscp via FreeBSD pkg"
    archive=$(get_tmpfile "txz")
    download "${archive}" "${FREEBSD_URL}"
    info "Downloaded FreeBSD package to ${archive}"
    if test_writeable "/usr/local/bin"; then
        sudo=""
        msg="Installing termscp, please wait…"
    else
        warn "Root permissions are required to install termscp…"
        elevate_priv
        sudo="sudo"
        msg="Installing termscp as root, please wait…"
    fi
    info "$msg"
    $sudo pkg install -y "${archive}"
}

install_on_linux() {
    local msg
    local sudo
    local archive
    if [ "${ARCH}" != "x86_64" ]; then
        try_with_cargo "we don't distribute packages for ${ARCH} at the moment"
    elif has yay; then
        info "Detected yay on your system"
        info "Installing termscp AUR package"
        yay -S termscp
    elif has pakku; then
        info "Detected pakku on your system"
        info "Installing termscp AUR package"
        pakku -S termscp
    elif has paru; then
        info "Detected paru on your system"
        info "Installing termscp AUR package"
        paru -S termscp
    elif has aurutils; then
        info "Detected aurutils on your system"
        info "Installing termscp AUR package"
        aurutils -S termscp
    elif has pamac; then
        info "Detected pamac on your system"
        info "Installing termscp AUR package"
        pamac -S termscp
    elif has pikaur; then
        info "Detected pikaur on your system"
        info "Installing termscp AUR package"
        pikaur -S termscp
    elif has dpkg; then
        info "Detected dpkg on your system"
        info "Installing termscp via Debian package"
        archive=$(get_tmpfile "deb")
        download "${archive}" "${DEB_URL}"
        info "Downloaded debian package to ${archive}"
        if test_writeable "/usr/bin"; then
            sudo=""
            msg="Installing termscp, please wait…"
        else
            warn "Root permissions are required to install termscp…"
            elevate_priv
            sudo="sudo"
            msg="Installing termscp as root, please wait…"
        fi
        info "$msg"
        $sudo dpkg -i "${archive}"
    elif has rpm; then
        info "Detected rpm on your system"
        info "Installing termscp via RPM package"
        archive=$(get_tempfile "rpm")
        download "${archive}" "${RPM_URL}"
        info "Downloaded rpm package to ${archive}"
        if test_writeable "/usr/bin"; then
            sudo=""
            msg="Installing termscp, please wait…"
        else
            warn "Root permissions are required to install termscp…"
            elevate_priv
            sudo="sudo"
            msg="Installing termscp as root, please wait…"
        fi
        info "$msg"
        $sudo rpm -U "${archive}"
    else
        try_with_cargo "No suitable installation method found for your Linux distribution; if you're running on Arch linux, please install an AUR package manager (such as yay). Currently only Arch, Debian based and Red Hat based distros are supported"
    fi
}

install_on_macos() {
    if has brew; then
        if has termscp; then
            info "Upgrading termscp..."
            # The OR is used since someone could have installed via cargo previously
            brew update && brew upgrade termscp || brew install veeso/termscp/termscp
        else
            info "Installing termscp..."
            brew install veeso/termscp/termscp
        fi
    else
        try_with_cargo "brew is missing on your system; please install it from <https://brew.sh/>"
    fi
}

try_with_cargo() {
    err="$1"
    if has cargo; then
        info "Installing termscp via Cargo..."
        cargo install termscp
    else
        error "$err"
        error "Alternatively you can opt for installing Cargo <https://www.rust-lang.org/tools/install>"
        return 1
    fi
}

# defaults
if [ -z "${PLATFORM-}" ]; then
    PLATFORM="$(detect_platform)"
fi

if [ -z "${BIN_DIR-}" ]; then
    BIN_DIR=/usr/local/bin
fi

if [ -z "${ARCH-}" ]; then
    ARCH="$(detect_arch)"
fi

if [ -z "${BASE_URL-}" ]; then
    BASE_URL="https://github.com/starship/starship/releases"
fi

# parse argv variables
while [ "$#" -gt 0 ]; do
    case "$1" in
        
        -V | --verbose)
            VERBOSE=1
            shift 1
        ;;
        -f | -y | --force | --yes)
            FORCE=1
            shift 1
        ;;
        -V=* | --verbose=*)
            VERBOSE="${1#*=}"
            shift 1
        ;;
        -f=* | -y=* | --force=* | --yes=*)
            FORCE="${1#*=}"
            shift 1
        ;;
        
        *)
            error "Unknown option: $1"
            exit 1
        ;;
    esac
done

printf "  %s\n" "${UNDERLINE}Termscp configuration${NO_COLOR}"
info "${BOLD}Platform${NO_COLOR}:      ${GREEN}${PLATFORM}${NO_COLOR}"
info "${BOLD}Arch${NO_COLOR}:          ${GREEN}${ARCH}${NO_COLOR}"

# non-empty VERBOSE enables verbose untarring
if [ -n "${VERBOSE-}" ]; then
    VERBOSE=v
    info "${BOLD}Verbose${NO_COLOR}: yes"
else
    VERBOSE=
fi

printf "\n"

confirm "Install ${GREEN}termscp ${TERMSCP_VERSION}${NO_COLOR}?"

# Installation based on arch
case $PLATFORM in
    "freebsd")
        install_on_bsd
    ;;
    "linux")
        install_on_linux
    ;;
    "macos")
        install_on_macos
    ;;
    *)
        error "${PLATFORM} is not supported by this installer"
        exit 1
    ;;
esac

completed "Congratulations! Termscp has successfully been installed on your system!"
info "If you're a new user, you might be interested in reading the user manual <https://veeso.github.io/termscp/#user-manual>"
info "While if you've just updated your termscp version, you can find the changelog at this link <https://veeso.github.io/termscp/#changelog>"
info "Remember that if you encounter any issue, you can report them on Github <https://github.com/veeso/termscp/issues/new>"
info "Feel free to open an issue also if you have an idea which could improve the project"
info "If you want to support the project, please, consider a little donation <https://www.buymeacoffee.com/veeso>"
info "I hope you'll enjoy using termscp :D"

exit 0
