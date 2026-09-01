# Build Instructions - TimeFlow

## Prerequisites

- **Rust**: 1.70+ with `cargo` (install via [rustup](https://rustup.rs/))
- **Node.js**: 18+ with npm or pnpm
- **Tauri CLI**: 2.x (`cargo install tauri-cli`)
- **macOS**: 12.0+ (Monterey or later)
- **Xcode Command Line Tools**: `xcode-select --install`

## Environment Setup

### 1. Install Rust

```bash
# Install rustup if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### 2. Install Node.js Dependencies

```bash
# Using pnpm (recommended)
pnpm install

# Or using npm
npm install
```

### 3. Install Tauri CLI

```bash
cargo install tauri-cli
```

## Build Steps

### Development Build

```bash
# Start development server with hot reload
pnpm tauri dev

# Or using npm
npm run tauri dev
```

This will:

1. Start the Vite dev server for the frontend
2. Compile the Rust backend
3. Launch the application window

### Production Build

```bash
# Build for production
pnpm tauri build

# Or using npm
npm run tauri build
```

**Build Artifacts Location:**

- macOS App Bundle: `src-tauri/target/release/bundle/macos/TimeFlow.app`
- DMG Installer: `src-tauri/target/release/bundle/dmg/TimeFlow_*.dmg`

## Verify Build Success

### Expected Output (Development)

```text
   Compiling timeflow v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
        Info Watching for changes...
```

### Expected Output (Production)

```text
   Compiling timeflow v0.1.0
    Finished release [optimized] target(s)
    Bundling TimeFlow.app
    Finished 1 bundle at:
        src-tauri/target/release/bundle/macos/TimeFlow.app
```

## Troubleshooting

### Build Fails with "xcrun: error"

**Cause**: Xcode Command Line Tools not installed or outdated

**Solution**:

```bash
xcode-select --install
# Or reset if already installed
sudo xcode-select --reset
```

### Build Fails with Rust Compilation Errors

**Cause**: Missing or incompatible Rust version

**Solution**:

```bash
# Update Rust to latest stable
rustup update stable
rustup default stable
```

### Build Fails with Node Module Errors

**Cause**: Corrupted or missing node_modules

**Solution**:

```bash
# Remove and reinstall
rm -rf node_modules
pnpm install
```

### Build Fails with "tauri" Command Not Found

**Cause**: Tauri CLI not installed or not in PATH

**Solution**:

```bash
cargo install tauri-cli
# Ensure ~/.cargo/bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build Hangs on "Bundling"

**Cause**: Code signing issues on macOS

**Solution**:

```bash
# For development, disable code signing
export APPLE_SIGNING_IDENTITY="-"
pnpm tauri build
```

## Clean Build

If experiencing persistent issues:

```bash
# Clean Rust build artifacts
cargo clean --manifest-path src-tauri/Cargo.toml

# Clean Node modules
rm -rf node_modules

# Reinstall and rebuild
pnpm install
pnpm tauri build
```
