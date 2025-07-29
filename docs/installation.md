# Installation Guide

## Download Pre-built Binaries

The easiest way to install Teraclio is to download a pre-built binary from the [GitHub Releases](https://github.com/skitsanos/teraclio/releases) page.

### Available Platforms

- **Linux (glibc)**: `teraclio-linux-amd64.tar.gz`
- **Linux (musl)**: `teraclio-linux-musl-amd64.tar.gz` 
- **macOS (Intel)**: `teraclio-macos-amd64.tar.gz`
- **macOS (Apple Silicon)**: `teraclio-macos-arm64.tar.gz`
- **Windows**: `teraclio-windows-amd64.exe.zip`

### Installation Steps

#### Linux/macOS
```bash
# Download the appropriate archive for your platform
wget https://github.com/skitsanos/teraclio/releases/latest/download/teraclio-linux-amd64.tar.gz

# Extract the binary
tar -xzf teraclio-linux-amd64.tar.gz

# Make it executable (if needed)
chmod +x teraclio

# Move to a directory in your PATH
sudo mv teraclio /usr/local/bin/

# Verify installation
teraclio --version
```

#### Windows
1. Download `teraclio-windows-amd64.exe.zip` from the releases page
2. Extract the ZIP file to get `teraclio.exe`
3. Add the directory containing `teraclio.exe` to your PATH environment variable
4. Open Command Prompt or PowerShell and run `teraclio --version`

## Build from Source

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git

### Build Steps
```bash
# Clone the repository
git clone https://github.com/skitsanos/teraclio.git
cd teraclio

# Build in release mode
cargo build --release

# The binary will be in target/release/teraclio
./target/release/teraclio --version

# Optional: Install globally
cargo install --path .
```

## Verify Installation

After installation, verify Teraclio is working correctly:

```bash
# Check version
teraclio --version

# View help
teraclio --help

# Test with a simple template
echo '{"name": "World"}' > data.json
echo 'Hello {{ data.name }}!' > template.txt
teraclio --source data.json --template template.txt
```

Expected output: `Hello World!`