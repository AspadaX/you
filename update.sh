#!/bin/bash

set -eo pipefail

echo "Updating you CLI tool..."

# Check if you is installed
if ! command -v you &> /dev/null; then
    echo "Error: you is not installed. Please run setup.sh instead."
    exit 1
fi

CURRENT_VERSION=$(you --version 2>/dev/null || echo "unknown")
echo "Current version: $CURRENT_VERSION"

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture to GitHub release format
case $ARCH in
  x86_64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  armv7l) ARCH="arm" ;;
  powerpc) ARCH="powerpc" ;;
  s390x) ARCH="s390x" ;;
  i686) ARCH="i686" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Construct download filename based on OS
case $OS in
  linux*)
    LIBC="gnu"
    if ldd --version 2>&1 | grep -iq musl || [[ -n $(find /lib -name 'ld-musl-*' -print -quit) ]]; then
      LIBC="musl"
    fi
    FILENAME="you-Linux-${LIBC}-${ARCH}.tar.gz"
    ;;
  darwin*)
    if [[ "$ARCH" == "arm64" ]]; then
      FILENAME="you-macOS-arm64.tar.gz"
    else
      FILENAME="you-macOS-x86_64.tar.gz"
    fi
    OS="macOS"
    ;;
  freebsd*)
    FILENAME="you-FreeBSD-x86_64.tar.gz"
    ;;
  *)
    echo "Unsupported operating system: $(uname -s)"
    exit 1
    ;;
esac

# GitHub repository and latest release URL
REPO="AspadaX/you"
RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"

# Get latest version information
echo "Checking for latest version..."
LATEST_VERSION=$(curl -s $RELEASE_URL | grep -o '"tag_name": *"[^"]*"' | sed 's/"tag_name": *"//;s/"//')
echo "Latest version: $LATEST_VERSION"

# Compare versions and exit if already up to date
if [[ "$CURRENT_VERSION" == "$LATEST_VERSION" ]]; then
  echo "You are already running the latest version ($LATEST_VERSION)."
  exit 0
fi

# Download URLs
DOWNLOAD_URL=$(curl -s $RELEASE_URL | grep -o "https://.*/$FILENAME" | head -1)
SHA_URL="$DOWNLOAD_URL.sha256"

# Create temporary directory
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Download and verify checksum
echo "Downloading $FILENAME..."
curl -L -o "$TMPDIR/$FILENAME" "$DOWNLOAD_URL"
curl -L -o "$TMPDIR/$FILENAME.sha256" "$SHA_URL"

echo "Verifying checksum..."
(cd "$TMPDIR" && shasum -a 256 -c "$FILENAME.sha256")

# Extract and install
tar xzf "$TMPDIR/$FILENAME" -C "$TMPDIR"
BIN_PATH=$(find "$TMPDIR" -name you -type f -print -quit)

if [[ -z $BIN_PATH ]]; then
  echo "Error: Could not find you binary in the package"
  exit 1
fi

INSTALL_DIR="/usr/local/bin"
echo "Updating you in $INSTALL_DIR..."
sudo mv -f "$BIN_PATH" "$INSTALL_DIR/you"
sudo chmod +x "$INSTALL_DIR/you"

# Verify installation
NEW_VERSION=$(you --version)
echo "Successfully updated you to version $NEW_VERSION"
