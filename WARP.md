# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Overview

AutoEQ App is a Tauri-based desktop application for optimizing speaker and headphone equalization based on measurements. It provides a graphical user interface for the AutoEQ library.

**Core Purpose**: GUI wrapper around the [AutoEQ library](https://github.com/pierreaubert/autoeq) for parametric EQ optimization using differential evolution and metaheuristics algorithms.

## Architecture

### Technology Stack

**Backend (Rust)**: 
- Tauri 2.x framework for native desktop integration
- `autoeq` library (git dependency) - core optimization algorithms
- Tauri commands expose Rust functions to frontend via IPC
- Single-instance plugin prevents multiple app instances
- API integration with spinorama.org for speaker measurements

**Frontend (TypeScript)**:
- Vite for build tooling and dev server (port 5173)
- Vanilla TypeScript with modular architecture (no framework)
- Plotly.js for visualization (frequency response, CEA2034, progress graphs)
- Web Audio API for real-time audio playback with EQ preview

### Project Structure

```
autoeq-app/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Main Tauri commands and optimization logic
│   │   ├── main.rs         # Entry point
│   │   ├── tests.rs        # Rust tests
│   │   └── test_mocks.rs   # Test utilities
│   ├── Cargo.toml          # Rust dependencies (uses workspace from autoeq repo)
│   └── tauri.conf.json     # Tauri configuration (app metadata, window config)
├── src-ui/                 # TypeScript frontend
│   └── src/
│       ├── main.ts         # Application entry point
│       ├── modules/        # Core UI logic
│       │   ├── api-manager.ts         # Tauri IPC communication
│       │   ├── ui-manager.ts          # DOM manipulation & form handling
│       │   ├── optimization-manager.ts # Optimization workflow
│       │   ├── plot/                  # Plotly visualization modules
│       │   ├── audio/                 # Web Audio API integration
│       │   └── templates.ts           # Dynamic HTML generation
│       ├── types/          # TypeScript type definitions
│       └── tests/          # Vitest unit & E2E tests
├── package.json            # Node.js dependencies
├── vite.config.ts          # Vite build configuration
├── Justfile                # Task runner recipes
└── .github/workflows/      # CI/CD for multi-platform builds
```

### Key Architectural Patterns

**Frontend Modular Design**:
- `UIManager`: Handles all DOM interactions, form state, progress updates
- `PlotManager`: Manages all Plotly visualizations (filter response, CEA2034, progress)
- `OptimizationManager`: Orchestrates optimization workflow, manages state
- `APIManager`: Tauri command invocation and response handling
- `AudioPlayer`: Real-time audio processing with parametric EQ filters

**Backend-Frontend Communication**:
- Frontend calls Rust via `invoke()` (Tauri IPC)
- Backend emits progress events via `app_handle.emit()` for real-time updates
- Cancellation handled via shared `CancellationState` with atomic operations

**Data Flow**:
1. User configures optimization parameters in UI
2. UIManager validates and packages params
3. APIManager invokes Rust `run_optimization` command
4. Rust backend runs optimization algorithm with progress callbacks
5. Progress events streamed to frontend via Tauri events
6. PlotManager updates visualizations in real-time
7. Optimization completes, filter parameters returned to frontend
8. AudioPlayer applies filters for preview

## Common Commands

### Development

```bash
# Start development server with hot-reload (recommended)
just dev
# or
npm run tauri dev

# Run in development mode (starts Vite on localhost:5173 + Tauri window)
npm run dev  # Vite only (for UI development without Tauri)
```

### Building

```bash
# Production build for current platform
just prod
# or
npm run tauri build

# Build for specific targets (macOS example)
npm run tauri build -- --target aarch64-apple-darwin  # Apple Silicon
npm run tauri build -- --target x86_64-apple-darwin   # Intel
npm run tauri build -- --target universal-apple-darwin # Universal

# Cross-compile for Linux from macOS (requires Docker)
just cross-linux-x86
```

**Build artifacts**: `src-tauri/target/release/bundle/` (.dmg, .deb, .AppImage, .msi)

### Testing

```bash
# Run all tests (Rust + TypeScript)
just test

# TypeScript tests only
npm run test              # All tests with Vitest
npm run test:unit         # Unit tests only
npm run test:e2e          # End-to-end tests
npm run test:e2e:force    # Force E2E (sets FORCE_E2E=true)
npm run test:watch        # Watch mode
npm run test:coverage     # Generate coverage report

# Rust tests only
just test-rust
# or
cargo test --workspace --release
```

**Test Framework**: Vitest with jsdom environment for TypeScript, cargo test for Rust

### Linting & Formatting

```bash
# Format all code
just fmt                 # Both Rust and TypeScript
just fmt-rust           # Rust only (cargo fmt)
just fmt-ts             # TypeScript only (Prettier)

# TypeScript linting
npx eslint src-ui/      # ESLint check
npx prettier -w src-ui  # Prettier format
```

### Dependency Management

```bash
# Update all dependencies
just update                 # Both Rust and TypeScript
just update-rust           # rustup + cargo update
just update-ts             # Tauri CLI + npm dependencies

# Check for outdated npm packages
npm run upgrade            # Uses npm-check-updates
```

### Cleaning

```bash
just clean    # Remove build artifacts, node_modules, temp files
cargo clean   # Rust build artifacts only
```

### Audio Player Build

```bash
# Build audio player separately (if needed)
npm run build:audio-player
```

## Version Management

**IMPORTANT**: Before committing changes, update version numbers:

1. **Increment patch version** in `src-tauri/Cargo.toml` (e.g., `0.3.53` → `0.3.54`)
2. **Update version** in `tauri.conf.json` (e.g., `0.4.35` → `0.4.36`)
3. **Update version** in `package.json` (e.g., `0.2.6` → `0.2.7`)

*Note: The user rule specifies incrementing the version in Cargo.toml files. While there's only one Cargo.toml in src-tauri/, the tauri.conf.json and package.json versions should also be kept in sync.*

## Prerequisites

### Required

- **Rust toolchain**: Install via [rustup.rs](https://rustup.rs/)
- **Node.js 18+**: For npm and Vite
- **just**: Command runner (`cargo install just`)

### Platform-Specific

**macOS**:
```bash
xcode-select --install  # Command Line Tools
brew install npm        # Node.js via Homebrew
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get install -y build-essential curl wget file \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows**:
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
- Install Node.js and Rust from official sites

### Cross-Compilation (Optional)

For building Linux binaries from macOS:
```bash
cargo install cross --git https://github.com/cross-rs/cross
rustup target add x86_64-unknown-linux-gnu
```

## Rust Workspace Dependencies

The `src-tauri/Cargo.toml` uses `workspace = true` for dependencies, which are defined in the **parent AutoEQ library** at `/Users/pierrre/src.local/autoeq/Cargo.toml`. 

**Key workspace dependencies**:
- `tauri`: Desktop framework
- `autoeq`: Core optimization library (git dependency)
- `ndarray`: Numerical arrays with BLAS
- `serde`, `serde_json`: Serialization
- `tokio`: Async runtime
- `reqwest`: HTTP client for spinorama.org API
- `plotly`: Visualization library

**Important**: The autoeq library must be available locally or accessible via git for builds to succeed.

## Common Issues

### Build Errors

```bash
# Clean everything and reinstall
cargo clean
rm -rf node_modules dist
npm install
```

### macOS Signing

For distribution, macOS apps require code signing and notarization. See [Tauri macOS signing docs](https://tauri.app/v1/guides/distribution/sign-macos).

### Rust Compilation Errors

If seeing workspace dependency errors, ensure the AutoEQ library is properly set up:
- Check that `/Users/pierrre/src.local/autoeq` exists
- Verify `Cargo.toml` in autoeq defines the workspace correctly
- Run `cargo update` to refresh dependencies

## CI/CD

GitHub Actions workflow (`.github/workflows/build.yml`) automatically builds for all platforms:

**Triggers**:
- Push to main/master branch
- Pull requests
- Git tags starting with `v*` (creates releases)

**Platforms**: macOS (Intel + ARM), Linux (x86_64), Windows (x86_64)

**Release Process**:
```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
# GitHub Actions will build and create release with artifacts
```

## Development Workflow

1. **Start dev server**: `just dev`
2. **Make changes** to TypeScript (hot-reload) or Rust (requires restart)
3. **Run tests**: `just test` or `npm run test:watch`
4. **Format code**: `just fmt`
5. **Update versions** in Cargo.toml, tauri.conf.json, package.json
6. **Test build**: `just prod`
7. **Commit and push**

## Important Notes

- **Hot-reload**: TypeScript changes reload automatically; Rust changes require restarting `just dev`
- **Port 5173**: Vite dev server runs here; ensure it's not in use
- **Single instance**: App prevents multiple windows (via tauri-plugin-single-instance)
- **API endpoint**: Connects to `https://api.spinorama.org` for speaker data
- **Audio resources**: Demo audio files bundled from `src-ui/public/demo-audio/*.wav`
