# AutoEQ GPUI Frontend

Native Rust GUI frontend for AutoEQ using [GPUI](https://github.com/zed-industries/gpui) and [gpui-component](https://github.com/longbridge/gpui-component).

## Prerequisites

### macOS Metal Toolchain

GPUI requires the Metal toolchain for rendering on macOS:

```bash
xcodebuild -downloadComponent MetalToolchain
```

### Other Dependencies

- Rust 1.70+ (install via [rustup.rs](https://rustup.rs/))
- Xcode Command Line Tools: `xcode-select --install`

## Project Structure

```
src-gpui/
├── src/
│   ├── main.rs                    # Application entry point with home screen
│   ├── components/                # UI components
│   │   ├── audio_interface.rs     # Audio device configuration
│   │   ├── eq_design.rs           # EQ optimization controls
│   │   ├── eq_customization.rs    # Spectrum analyzer & EQ customization
│   │   ├── audio_player.rs        # Audio playback with EQ preview
│   │   ├── audio_capture.rs       # Room measurement capture
│   │   └── workflow_navigator.rs  # Responsive workflow navigation
│   ├── workflows/                 # Workflow implementations
│   │   ├── headphone.rs           # Headphone EQ workflow (5 steps)
│   │   ├── speaker.rs             # Speaker EQ workflow (5 steps)
│   │   └── room.rs                # Room correction workflow (5 steps)
│   └── backend/                   # Backend adapter (autoeq library integration)
│       └── mod.rs
├── tests/                         # Component tests
└── examples/                      # Standalone examples
```

## Building

### Development Build

```bash
cargo build
```

### Run Application

```bash
cargo run
```

### Run Tests

```bash
cargo test
```

### Run Examples

```bash
# Audio interface component demo
cargo run --example audio_interface_demo

# Workflow demo
cargo run --example workflow_demo
```

## Architecture

### Home Screen

The main application presents three large icons for workflow selection:
1. **Headphone** - Curve-based headphone EQ optimization
2. **Speaker** - Speaker measurement-based EQ (via spinorama.org API)
3. **Room** - Room correction via audio capture

### Workflows

Each workflow follows a 5-step process:

#### Headphone Workflow
1. Select curve & target
2. EQ design (run optimization)
3. Display frequency response
4. Audio player (test EQ)
5. Save EQ settings

#### Speaker Workflow  
1. Select speaker (brand, model, measurement)
2. EQ design (run optimization)
3. Display CEA2034 curves
4. Audio player (test EQ)
5. Save EQ settings

#### Room Workflow
1. Audio capture (room measurement)
2. EQ design (run optimization)
3. Display measured vs corrected curves
4. Audio player (test correction)
5. Save EQ settings

### Responsive Navigation

The workflow navigator adapts to screen size:

**Desktop (>700px):** 
```
1 -- 2 -- 3 -- 4 -- 5
```

**Mobile (<700px):**
```
[Prev] [Next]
```

## Components

### AudioInterfaceComponent
- Device selection (input/output)
- Channel configuration (1-8 channels)
- Sample rate selection (44.1k, 48k, 96k, 192k)
- Bit depth selection (16, 24, 32 bit)
- Channel routing matrix

### EQDesignComponent
- Optimization parameter controls
- "Run Optimization" button with progress
- "Reset" button
- Real-time progress display

### EQCustomizationComponent
- Spectrum analyzer visualization
- Frequency response curve display
- Interactive EQ band adjustment
- Filter parameter display (freq, gain, Q)

### AudioPlayerComponent
- Play/pause/stop controls
- Volume slider
- File selection & drag-drop
- Waveform visualization
- Real-time EQ preview

### AudioCaptureComponent
- Input device selection
- Record/stop controls
- Level meters
- Sweep signal generation (for room measurement)
- Capture progress & duration

## Backend Integration

The GPUI frontend reuses the core `autoeq` library for optimization but avoids the Tauri-specific `autoeq_backend` to prevent dependency conflicts.

**Key integrations:**
- `autoeq` library for EQ optimization algorithms
- `cpal` for audio device enumeration and I/O
- `reqwest` for spinorama.org API calls
- `plotly` for curve visualization data

## Development Status

### ✅ Completed
- Project structure and workspace configuration
- Component skeletons with basic UI
- Workflow routing and navigation
- Responsive workflow navigator
- Home screen with workflow selection
- **Real optimization backend** - Full AutoEQ library integration
  - Support for multiple algorithms (COBYLA, ISRES, DE, etc.)
  - Progress callbacks and cancellation support
- **Export functionality** - Generate configs for:
  - CamillaDSP
  - Parametric EQ
  - REW (Room EQ Wizard)
- **Filter display component** - Table view with copy/export
- **Frequency plot component** - Visualization with computed bounds
- **Spinorama API client** - Fetch speaker data from spinorama.org

### ⚠️ In Progress
- Interactive workflow state management
- File picker integration for curve selection
- Canvas-based frequency response rendering
- Complete speaker workflow with spinorama integration
- Complete room workflow with audio capture
- Real-time audio playback with EQ preview

### 📋 TODO
- Audio capture with sweep signal generation
- CEA2034 curve visualization  
- Interactive EQ band adjustment
- Spectrum analyzer with FFT
- Performance optimization
- CI/CD integration
- Comprehensive test suite

## Relationship to Existing Codebase

This GPUI frontend is a **parallel implementation** alongside the existing Tauri+TypeScript UI:

- **src-ui/** - Existing Tauri + TypeScript frontend (unchanged)
- **src-gpui/** - New GPUI + Rust frontend (this project)
- **src-backend/** - Shared backend logic (used by Tauri, not directly by GPUI)
- **src-audio-capture/** - Separate Tauri audio capture app

The GPUI frontend is isolated in its own workspace to avoid dependency conflicts between GPUI and Tauri.

## License

Same as parent AutoEQ project.
