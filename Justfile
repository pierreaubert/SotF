# --------------------------------------------------------- -*- just -*-
# How to install Just?
#	  cargo install just
# ----------------------------------------------------------------------

# List all available commands
default:
	@just --list

# ----------------------------------------------------------------------
# PROD
# ----------------------------------------------------------------------

alias build := prod

prod:
	cargo build --release
	cd src-ui && npm run tauri build

# Build audio capture app for production
prod-capture:
	cargo build --release
	cd src-audio-capture && npm run tauri build

# Build GPUI app for production
prod-gpui:
	cd src-gpui && cargo build --release

# ----------------------------------------------------------------------
# DEV
# ----------------------------------------------------------------------

dev:
	cargo build
	cd src-ui && npm run tauri dev

dev-capture:
	cargo build
	cd src-audio-capture && npm run tauri dev

dev-gpui:
	cd src-gpui && cargo run

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
	cargo test

test-ts:
	cd src-ui && npm run test

test-gpui:
	cd src-gpui && cargo test

# ----------------------------------------------------------------------
# FORMAT
# ----------------------------------------------------------------------

fmt: fmt-rust fmt-ts-ui fmt-ts-capture

fmt-rust:
	cargo fmt --all

fmt-ts-capture:
	cd src-audio-capture && npm run fmt

fmt-ts-ui:
	cd src-ui && npm run fmt

# ----------------------------------------------------------------------
# CLEAN
# ----------------------------------------------------------------------

clean:
	cargo clean
	rm -rf src-*/dist
	rm -rf src-*/node_modules
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

install-ubuntu-common:
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
			 libasound2-dev

install-ubuntu-x86-driver :
		sudo apt install -y \
			 chromium-browser \
			 chromium-chromedriver

install-ubuntu-arm64-driver :
		sudo apt install -y firefox
		# where is the geckodriver ?

install-ubuntu-node:
		# node
		sudo npm cache clean -f
		sudo npm install -f n
		sudo n stable

install-ubuntu-x86: install-ubuntu-common install-ubuntu-x86-driver install-ubuntu-node

install-ubuntu-arm64: install-ubuntu-common install-ubuntu-arm64-driver install-ubuntu-node


# ----------------------------------------------------------------------
# POST
# ----------------------------------------------------------------------

post-install-npm-ui:
	cd src-ui && npm install .

post-install-npm-capture:
	cd src-audio-capture && npm install . && cd ..

post-install-npm: post-install-npm-ui post-install-npm-capture

post-install-rust:
	rustup default stable
	cargo install just
	cargo check

post-install: post-install-rust post-install-npm

# ----------------------------------------------------------------------
# SIGNING
# ----------------------------------------------------------------------



