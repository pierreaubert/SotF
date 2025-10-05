<!-- markdownlint-disable-file MD013 -->

# AutoEQ App : Desktop Application

## Introduction

This is the desktop application (Tauri-based) for AutoEQ. It provides a graphical user interface for optimizing speaker and headphone equalization based on measurements.

The application uses Tauri (Rust backend + TypeScript frontend) to provide a cross-platform desktop experience.

## Prerequisites

### Rust

Install [rustup](https://rustup.rs/) first.

If you already have cargo / rustup:

```shell
cargo install just
```

### Node.js

Install Node.js 18 or higher.

### Platform-specific Requirements

#### macOS

Install [brew](https://brew.sh/) first, then:

```shell
brew install npm
```

#### Linux

```shell
sudo apt-get update
sudo apt-get install -y build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

#### Windows

Install the [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

## Development Setup

### Install Dependencies

```shell
npm install
```

### Run in Development Mode

```shell
just dev
# or
npm run tauri dev
```

This will start the Vite development server and launch the Tauri application in development mode with hot-reload enabled.

## Building for Production

### Build the Application

```shell
just prod
# or
npm run tauri build
```

This will create platform-specific bundles in `src-tauri/target/release/bundle/`.

### Output Locations

- **macOS**: `.dmg` and `.app` files
- **Linux**: `.deb`, `.AppImage` files
- **Windows**: `.msi` and `.exe` files

## Using Just

```shell
just
```

will give you the list of possible commands:

- **Development build**: `just dev`
- **Production build**: `just prod`
- **Cross-compile for Linux (from macOS)**: `just cross-linux-x86`

## Project Structure

```
autoeq-app/
├── src-tauri/           # Rust backend (Tauri)
│   ├── src/            # Rust source code
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── src-ui/             # TypeScript frontend
│   ├── src/           # TypeScript source code
│   ├── assets/        # Static assets
│   └── index.html     # Entry HTML file
├── package.json       # Node.js dependencies
├── vite.config.ts     # Vite configuration
└── tsconfig.json      # TypeScript configuration
```

## Testing

```shell
npm run test
```

## Troubleshooting

### Build Errors

If you encounter build errors, try:

```shell
# Clean build artifacts
cargo clean
rm -rf node_modules
rm -rf dist

# Reinstall dependencies
npm install
```

### macOS Signing Issues

For macOS builds, you may need to configure code signing. See the [Tauri documentation](https://tauri.app/v1/guides/distribution/sign-macos) for details.

## Related Projects

- [AutoEQ CLI](https://github.com/pierreaubert/autoeq) - Command-line tools and libraries

## License

GPL-3.0-or-later
