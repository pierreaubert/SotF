# AutoEQ GPUI Examples

This directory contains standalone examples demonstrating individual components and features of the AutoEQ GPUI application.

## Available Examples

### Audio Interface Component

Demonstrates the audio device selection component, which displays available input and output devices on the system.

**Run:**
```bash
cargo run --example audio_interface
```

**Features:**
- Lists all audio input and output devices
- Shows device details (sample rate, channel count, bit depth)
- Refresh button to re-scan for devices
- Automatically selects default devices
- Click device to select it
- Routing matrix icon to configure channel routing

### Routing Matrix Component

Demonstrates the channel routing matrix component for configuring audio channel routing.

**Run:**
```bash
cargo run --example routing_matrix
```

**Features:**
- Interactive grid for routing logical channels to physical outputs
- Support for 2, 4, and 8 channel configurations
- Stereo (2ch): Left, Right
- Quadraphonic (4ch): Left, Right, Center, Subwoofer
- 7.1 Surround (8ch): Left, Right, Center, Sub, SR, SL, RR, RL
- Automatic channel swapping to maintain valid routing
- Modal overlay with semi-transparent backdrop
- Click cells to change routing
- Close button or click outside to dismiss

## Running Examples

All examples can be run using:
```bash
cargo run --example <example_name>
```

For better performance, run with release mode:
```bash
cargo run --release --example <example_name>
```

## Example Structure

Each example is a minimal GPUI application that:
1. Initializes the GPUI framework with `app_setup::init_app(cx)`
2. Sets up menu bar with Quit option
3. Configures platform-specific keyboard shortcuts (Cmd-Q on macOS, Ctrl-Q elsewhere)
4. Creates a window with appropriate size and title
5. Demonstrates one or more components
6. Provides clear visual context and instructions

## Development

When developing new components, create a corresponding example to:
- Test the component in isolation
- Document its usage
- Demonstrate its features
- Serve as a starting point for integration

## App Setup

All examples and the main application use a common setup module (`src/app_setup.rs`) that provides:

**Menu Bar:**
- AutoEQ menu with Quit option
- File menu with Quit option

**Keyboard Shortcuts:**
- **macOS:** `Cmd-Q` to quit
- **Linux/Windows:** `Ctrl-Q` to quit

**Usage in your code:**
```rust
use autoeq_gpui::app_setup;

Application::new().run(|cx: &mut App| {
    // Initialize menu bar and keyboard shortcuts
    app_setup::init_app(cx);
    
    // ... rest of your app setup
});
```

## Design System

All GPUI components use a unified design system (`src/design.rs`) that matches the TypeScript/Tauri UI:

**Colors** (Light Mode):
- Primary: #007bff (blue)
- Background: #f8f9fa (light gray), #ffffff (white)
- Text: #212529 (dark), #6c757d (secondary)
- Borders: #dee2e6
- Success/Warning/Danger/Info colors

**Spacing**:
- XS: 4px, SM: 8px, MD: 12px, LG: 16px, XL: 24px

**Typography**:
- Base size: 13px
- System font stack
- Consistent font sizes (XS to 2XL)

**Components**:
- Section headers with accent background
- Primary/secondary/text buttons
- Input fields with proper styling
- Consistent border radius (6px) and shadows

## Notes

- Examples require the same dependencies as the main application
- Logging is enabled by default (set `RUST_LOG=debug` for verbose output)
- Audio device enumeration requires system permissions on some platforms
- All components follow the design system for consistent look and feel
