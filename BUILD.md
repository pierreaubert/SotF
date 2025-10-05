# AutoEQ App Build Guide

This guide explains how to build the AutoEQ desktop application (Tauri) for macOS, Linux, and Windows platforms.

## Quick Start

### Prerequisites

1. **Rust toolchain** (install via [rustup.rs](https://rustup.rs/))
2. **Node.js 18+**
3. **Platform-specific tools** (see below)

### Build Commands

```bash
# Install dependencies
npm install

# Build for current platform
npm run tauri build

# Development mode with hot-reload
npm run tauri dev
```

## Supported Platforms

- ✅ **macOS** - Intel and Apple Silicon
- ✅ **Linux** - x86_64
- ✅ **Windows** - x86_64

## Platform-Specific Setup

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Node.js (via Homebrew)
brew install node
```

### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config
```

### Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. Install [Node.js](https://nodejs.org/)
3. Install [Rust](https://rustup.rs/)

## Manual Build Instructions

### 1. Install Node Dependencies

```bash
npm install
```

### 2. Build for Current Platform

```bash
npm run tauri build
```

This will:
1. Build the TypeScript frontend using Vite
2. Compile the Rust backend
3. Create platform-specific bundles

### 3. Build for Specific Target

#### macOS

```bash
# Apple Silicon
npm run tauri build -- --target aarch64-apple-darwin

# Intel
npm run tauri build -- --target x86_64-apple-darwin

# Universal Binary (both architectures)
npm run tauri build -- --target universal-apple-darwin
```

#### Linux

```bash
# x86_64
npm run tauri build -- --target x86_64-unknown-linux-gnu

# ARM64 (requires cross-compilation)
rustup target add aarch64-unknown-linux-gnu
npm run tauri build -- --target aarch64-unknown-linux-gnu
```

#### Windows

```bash
# x86_64
npm run tauri build -- --target x86_64-pc-windows-msvc
```

## Cross-Compilation

### Linux from macOS

```bash
# Install cross-compilation tools
cargo install cross --git https://github.com/cross-rs/cross

# Add Linux target
rustup target add x86_64-unknown-linux-gnu

# Build using cross (requires Docker)
cd src-tauri
cross build --release --target x86_64-unknown-linux-gnu
```

## Output Structure

After building, you'll find bundles in:

```
src-tauri/target/release/bundle/
├── dmg/              # macOS disk images
│   └── AutoEQ_*.dmg
├── macos/            # macOS .app bundles
│   └── AutoEQ.app
├── deb/              # Debian packages (Linux)
│   └── autoeq_*.deb
├── appimage/         # AppImage (Linux)
│   └── autoeq_*.AppImage
└── msi/              # Windows installers
    └── AutoEQ_*.msi
```

## Development

### Hot-Reload Development

```bash
npm run tauri dev
```

This starts:
- Vite dev server on `http://localhost:5173`
- Tauri window with hot-reload enabled

Changes to TypeScript/HTML/CSS will hot-reload automatically.
Changes to Rust code require restarting the dev server.

### Testing

```bash
# Run all tests
npm run test

# Run unit tests only
npm run test:unit

# Run with coverage
npm run test:coverage
```

## Troubleshooting

### Build Fails on macOS

- Ensure Xcode Command Line Tools are installed: `xcode-select --install`
- Check that Rust is up to date: `rustup update`

### Build Fails on Linux

- Install all required dependencies (see Linux setup section)
- Ensure GTK3 development headers are installed

### Build Fails on Windows

- Ensure Visual Studio Build Tools are installed
- Try using the Visual Studio Developer Command Prompt

### "Cannot find module" Errors

```bash
# Clean and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Rust Compilation Errors

```bash
# Clean Rust build artifacts
cargo clean

# Update dependencies
cargo update
```

## GitHub Actions (Automated Builds)

The repository includes a GitHub Actions workflow that automatically builds the application for all platforms on:

- Push to main/master branch
- Pull requests
- Git tags (creates releases)

See `.github/workflows/build.yml` for details.

## Code Signing and Notarization

### macOS

For distribution, macOS apps need to be signed and notarized. See the [Tauri documentation](https://tauri.app/v1/guides/distribution/sign-macos) for details.

### Windows

For distribution, Windows apps should be signed. See the [Tauri documentation](https://tauri.app/v1/guides/distribution/sign-windows) for details.

## Distribution

### Creating Releases

1. Tag the release:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```

2. GitHub Actions will automatically build and create a release with artifacts.

### Manual Distribution

After building, the bundles in `src-tauri/target/release/bundle/` can be distributed directly or uploaded to release platforms.

## Next Steps

1. Set up code signing for macOS/Windows
2. Configure auto-updater (built into Tauri)
3. Set up continuous integration for automated testing
4. Create installer customizations (icons, splash screens, etc.)
