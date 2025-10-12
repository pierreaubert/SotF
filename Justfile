# --------------------------------------------------------- -*- just -*-
# How to install Just?
#     cargo install just
# ----------------------------------------------------------------------

# List all available commands
default:
	@just --list

# ----------------------------------------------------------------------
# PROD
# ----------------------------------------------------------------------

prod:
	npm run tauri build

# Check code signing setup for macOS distribution
check-signing:
	./scripts/check-signing.sh

# ----------------------------------------------------------------------
# DEV
# ----------------------------------------------------------------------

dev:
	npm run tauri dev

# ----------------------------------------------------------------------
# UPDATE
# ----------------------------------------------------------------------

update: update-rust update-ts

update-rust:
	rustup update
	cargo update

update-ts:
	npm run tauri update
	npm run upgrade

# ----------------------------------------------------------------------
# TEST
# ----------------------------------------------------------------------

test: test-rust test-ts

test-rust:
	cd src-tauri && cargo test

test-ts:
	npm run test

# ----------------------------------------------------------------------
# FORMAT
# ----------------------------------------------------------------------

fmt: fmt-rust fmt-ts

fmt-rust:
	cd src-tauri && cargo fmt --all

fmt-ts:
	npm run fmt

# ----------------------------------------------------------------------
# CLEAN
# ----------------------------------------------------------------------

clean:
	cargo clean
	rm -rf dist
	rm -rf node_modules
	find . -name '*~' -exec rm {} \; -print
	find . -name 'Cargo.lock' -exec rm {} \; -print
	find . -name 'package-lock.json' -exec rm {} \; -print

# ----------------------------------------------------------------------
# CROSS
# ----------------------------------------------------------------------

cross: cross-linux-x86

cross-linux-x86:
	echo "This can take minutes!"
	cd src-tauri && cross build --release --target x86_64-unknown-linux-gnu

cross-win-x86-gnu:
	echo "This is not working well yet from macOS!"
	cd src-tauri && cross build --release --target x86_64-pc-windows-gnu

# ----------------------------------------------------------------------
# INSTALL
# ----------------------------------------------------------------------

install-cross:
	cargo install cross --git https://github.com/cross-rs/cross

install-macos:
	# need rustup first
	# need xcode
	xcode-select --install
	# need brew
	brew install npm
	npm install .
	# For Tauri
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin

install-ubuntu-arm:
        sudo apt install -y \
             curl \
             build-essential gcc g++ \
             pkg-config \
             libssl-dev \
             ca-certificates \
             cmake \
             ninja-build \
             perl \
             rustup \
             just \
             libglib2.0-dev \
             libgtk-3-dev \
             libwebkit2gtk-4.1-dev \
             libayatana-appindicator3-dev \
             librsvg2-dev \
             patchelf \
             libopenblas-dev \
             gfortran \
             chromium-browser \
             chromium-chromedriver
        # rust
        rustup default stable
        # node
        sudo npm cache clean -f
        sudo npm install -f n
        sudo n stable
        /usr/local/bin/npm install .

