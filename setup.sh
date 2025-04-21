#!/bin/bash

set -euo pipefail

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

# Construct download filename for macOS
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
echo "Installing you to $INSTALL_DIR..."
sudo mv -f "$BIN_PATH" "$INSTALL_DIR/you"
sudo chmod +x "$INSTALL_DIR/you"

# Assist the user to setup required environment variables
read -p "Use default OpenAI endpoint? (y for yes, or paste your api endpoint here): " ENDPOINT
if [[ "$ENDPOINT" == "y" ]]; then
    ENDPOINT="https://api.openai.com"
fi

read -p "API key:" api_key
read -p "Model:" model

if [ -n "$ZSH_VERSION" ]; then
  PROFILE_FILE=~/.zshrc
elif [ -n "$BASH_VERSION" ]; then
  PROFILE_FILE=~/.bashrc
else
  PROFILE_FILE=~/.profile
fi

echo "export YOU_OPENAI_API_BASE=\"$endpoint\"" >> $PROFILE_FILE
echo "export YOU_OPENAI_API_KEY=\"$api_key\"" >> $PROFILE_FILE
echo "export YOU_OPENAI_MODEL=\"$model\"" >> $PROFILE_FILE
source $PROFILE_FILE
echo "Environment variable has been set. You may find them has been added here: $PROFILE_FILE"

echo "Successfully installed you $(you --version)"
