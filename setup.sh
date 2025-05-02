#!/bin/bash

set -eo pipefail

# Determine OS and architecture
OS=$(uname -s 2>/dev/null || echo "Windows_NT")
OS=$(echo "$OS" | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m 2>/dev/null || echo "x86_64")

# Check if running in Windows
IS_WINDOWS=false
if [[ "$OS" == *"windows"* || "$OS" == *"mingw"* || "$OS" == *"msys"* || "$OS" == *"cygwin"* ]]; then
  IS_WINDOWS=true
  OS="windows"
fi

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
  windows*)
    FILENAME="you-Windows-x86_64.zip"
    ;;
  *)
    echo "Unsupported operating system: $(uname -s)"
    exit 1
    ;;
esac

# GitHub repository and latest release URL
REPO="AspadaX/you"
RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"

# Download URLs with error handling
echo "Fetching latest release information..."
RELEASE_INFO=$(curl -s -f $RELEASE_URL)
if [ $? -ne 0 ]; then
  echo "Error: Failed to fetch release information from GitHub. Check your internet connection."
  exit 1
fi

DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep -o "https://.*/$FILENAME" | head -1)
if [[ -z "$DOWNLOAD_URL" ]]; then
  echo "Error: Could not find download URL for $FILENAME in the latest release."
  echo "Available files:"
  echo "$RELEASE_INFO" | grep -o "https://.*\.(zip\|tar\.gz)" | sort
  exit 1
fi

SHA_URL="$DOWNLOAD_URL.sha256"

# Create temporary directory
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Download and verify checksum
echo "Downloading $FILENAME..."
curl -L -o "$TMPDIR/$FILENAME" "$DOWNLOAD_URL"
if [ $? -ne 0 ]; then
  echo "Error: Failed to download the binary. Check your internet connection."
  exit 1
fi

curl -L -o "$TMPDIR/$FILENAME.sha256" "$SHA_URL"
if [ $? -ne 0 ]; then
  echo "Warning: Failed to download the checksum file. Skipping verification."
else
  echo "Verifying checksum..."
  # Read expected hash from the downloaded file
  expected_hash=$(cat "$TMPDIR/$FILENAME.sha256" | awk '{print $1}')
  
  # Try multiple hash verification methods
  if command -v shasum >/dev/null 2>&1; then
    actual_hash=$(shasum -a 256 "$TMPDIR/$FILENAME" | awk '{print $1}')
  elif command -v sha256sum >/dev/null 2>&1; then
    actual_hash=$(sha256sum "$TMPDIR/$FILENAME" | awk '{print $1}')
  else
    echo "Warning: Could not find shasum or sha256sum. Skipping verification."
    actual_hash=$expected_hash  # Skip verification
  fi
  
  if [ "$actual_hash" != "$expected_hash" ]; then
    echo "Error: Checksum verification failed."
    echo "Expected: $expected_hash"
    echo "Actual: $actual_hash"
    exit 1
  else
    echo "Checksum verification successful."
  fi
fi

# Extract and install
if [[ "$OS" == "windows" ]]; then
  echo "Extracting Windows binary..."
  if command -v unzip >/dev/null 2>&1; then
    unzip -o "$TMPDIR/$FILENAME" -d "$TMPDIR"
  else
    echo "Error: unzip command not found. Please install unzip to continue."
    exit 1
  fi
  BIN_PATH=$(find "$TMPDIR" -name "you.exe" -type f -print -quit)
else
  echo "Extracting binary..."
  tar xzf "$TMPDIR/$FILENAME" -C "$TMPDIR"
  BIN_PATH=$(find "$TMPDIR" -name "you" -type f -print -quit)
fi

if [[ -z $BIN_PATH ]]; then
  # Try a more flexible search if the specific name wasn't found
  BIN_PATH=$(find "$TMPDIR" -type f -executable -print -quit)
  
  if [[ -z $BIN_PATH ]]; then
    echo "Error: Could not find binary in the package"
    echo "Files in extracted directory:"
    find "$TMPDIR" -type f | sort
    exit 1
  fi
fi

echo "Found binary at: $BIN_PATH"

# Determine the install directory based on OS
if $IS_WINDOWS; then
  INSTALL_DIR="$HOME/bin"
  mkdir -p "$INSTALL_DIR" 2>/dev/null
else
  INSTALL_DIR="/usr/local/bin"
fi

echo "Installing you to $INSTALL_DIR..."
if $IS_WINDOWS; then
  mv -f "$BIN_PATH" "$INSTALL_DIR/you.exe"
else
  sudo mv -f "$BIN_PATH" "$INSTALL_DIR/you"
  sudo chmod +x "$INSTALL_DIR/you"
fi

# Assist the user to setup required environment variables
ENDPOINT="https://api.openai.com"
read -p "Use default OpenAI endpoint? (y for yes, or paste your api endpoint here): " USER_INPUT
if [[ "$USER_INPUT" == "y" ]]; then
    ENDPOINT="https://api.openai.com"
else
    ENDPOINT="$USER_INPUT"
fi

read -p "API key: " api_key
read -p "Model: " model

# Determine the user's shell and appropriate profile file
if $IS_WINDOWS; then
  # For Windows, we'll create a PowerShell profile if it doesn't exist
  POWERSHELL_PROFILE="$HOME/Documents/WindowsPowerShell/Microsoft.PowerShell_profile.ps1"
  PROFILE_DIR=$(dirname "$POWERSHELL_PROFILE")
  
  mkdir -p "$PROFILE_DIR" 2>/dev/null
  
  echo "[Environment]::SetEnvironmentVariable('YOU_OPENAI_API_BASE', '$ENDPOINT', 'User')" >> "$POWERSHELL_PROFILE"
  echo "[Environment]::SetEnvironmentVariable('YOU_OPENAI_API_KEY', '$api_key', 'User')" >> "$POWERSHELL_PROFILE"
  echo "[Environment]::SetEnvironmentVariable('YOU_OPENAI_MODEL', '$model', 'User')" >> "$POWERSHELL_PROFILE"
  
  # Set for current session too
  echo "\`$env:YOU_OPENAI_API_BASE = '$ENDPOINT'" >> "$POWERSHELL_PROFILE"
  echo "\`$env:YOU_OPENAI_API_KEY = '$api_key'" >> "$POWERSHELL_PROFILE"
  echo "\`$env:YOU_OPENAI_MODEL = '$model'" >> "$POWERSHELL_PROFILE"
  
  PROFILE_FILE="$POWERSHELL_PROFILE"
else
  # For Unix-like systems - try multiple detection methods
  CURRENT_SHELL=""
  
  # Method 1: Check $SHELL variable
  if [[ -n "$SHELL" ]]; then
    CURRENT_SHELL=$(basename "$SHELL")
  else
    # Method 2: Check process name
    CURRENT_SHELL=$(ps -p $$ -o comm= 2>/dev/null | sed 's/^-//')
  fi
  
  # Method 3: Fall back to checking for known shell files
  if [[ -z "$CURRENT_SHELL" || "$CURRENT_SHELL" == "sh" ]]; then
    if [[ -f "$HOME/.zshrc" ]]; then
      CURRENT_SHELL="zsh"
    elif [[ -f "$HOME/.bashrc" || -f "$HOME/.bash_profile" ]]; then
      CURRENT_SHELL="bash"
    elif [[ -f "$HOME/.config/fish/config.fish" ]]; then
      CURRENT_SHELL="fish"
    else
      CURRENT_SHELL="bash"  # Default to bash if all detection methods fail
    fi
  fi
  
  echo "Detected shell: $CURRENT_SHELL"
  
  case "$CURRENT_SHELL" in
    zsh)
      PROFILE_FILE=~/.zshrc
      ;;
    bash)
      # For macOS, use .bash_profile
      if [[ "$OS" == "macOS" ]]; then
        if [[ -f "$HOME/.bash_profile" ]]; then
          PROFILE_FILE=~/.bash_profile
        else
          # Some macOS users might only have .bashrc
          PROFILE_FILE=~/.bashrc
        fi
      else
        PROFILE_FILE=~/.bashrc
      fi
      ;;
    fish)
      PROFILE_FILE=~/.config/fish/config.fish
      mkdir -p "$(dirname "$PROFILE_FILE")" 2>/dev/null
      ;;
    *)
      PROFILE_FILE=~/.profile
      ;;
  esac

  # Make sure the file exists
  touch "$PROFILE_FILE"

  # For fish shell, use a different syntax
  if [[ "$CURRENT_SHELL" == "fish" ]]; then
    echo "set -x YOU_OPENAI_API_BASE \"$ENDPOINT\"" >> "$PROFILE_FILE"
    echo "set -x YOU_OPENAI_API_KEY \"$api_key\"" >> "$PROFILE_FILE"
    echo "set -x YOU_OPENAI_MODEL \"$model\"" >> "$PROFILE_FILE"
  else
    echo "export YOU_OPENAI_API_BASE=\"$ENDPOINT\"" >> "$PROFILE_FILE"
    echo "export YOU_OPENAI_API_KEY=\"$api_key\"" >> "$PROFILE_FILE"
    echo "export YOU_OPENAI_MODEL=\"$model\"" >> "$PROFILE_FILE"
  fi
fi

echo "Environment variables have been set in: $PROFILE_FILE"
if $IS_WINDOWS; then
  echo "To use them in your current session, run: "
  echo "  . $POWERSHELL_PROFILE  # In PowerShell"
else
  echo "To use them in your current session, run:"
  echo "  source \"$PROFILE_FILE\""
fi

echo "Successfully installed you. Run 'you --version' to verify the installation."
