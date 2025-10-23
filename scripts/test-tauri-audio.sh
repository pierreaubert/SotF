#!/bin/bash

# Tauri Audio Integration Test Helper
# This script helps start the dev environment for testing

set -e

echo "🎵 Tauri Audio Integration Test Helper"
echo "========================================"
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v camilladsp &> /dev/null; then
    echo "❌ CamillaDSP not found. Please install it first."
    exit 1
fi

echo "✓ CamillaDSP found at: $(which camilladsp)"
echo "✓ Version: $(camilladsp --version)"
echo ""

# Check demo files
if [ ! -f "public/demo-audio/piano.wav" ]; then
    echo "❌ Demo audio files not found in public/demo-audio/"
    exit 1
fi

echo "✓ Demo audio files found"
echo ""

# Kill any existing CamillaDSP processes
if pgrep -x "camilladsp" > /dev/null; then
    echo "⚠️  Existing CamillaDSP process found. Killing it..."
    pkill -9 camilladsp || true
    sleep 1
fi

# Check if port 1234 is in use
if lsof -i :1234 > /dev/null 2>&1; then
    echo "⚠️  Port 1234 is in use. Trying to free it..."
    lsof -ti :1234 | xargs kill -9 || true
    sleep 1
fi

echo "✓ Port 1234 is available"
echo ""

# Compile the backend
echo "📦 Compiling Rust backend..."
cargo build --release --bin audio_test > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Rust backend compiled successfully"
else
    echo "❌ Rust compilation failed"
    exit 1
fi
echo ""

echo "✅ All prerequisites met!"
echo ""
echo "📖 Next steps:"
echo "   1. Run: just dev (or npm run tauri dev)"
echo "   2. Open DevTools in the Tauri window (Cmd+Option+I on macOS)"
echo "   3. Follow the test guide: docs/TAURI_AUDIO_INTEGRATION_TEST.md"
echo ""
echo "Quick test script for console:"
echo "----------------------------------------"
cat << 'EOF'
async function quickTest() {
  console.log('🎵 Testing audio...');
  await window.__TAURI__.invoke('audio_start_playback', {
    filePath: '/Users/pierre/src/autoEQ-app/public/demo-audio/piano.wav',
    outputDevice: null,
    sampleRate: 48000,
    channels: 2,
    filters: []
  });
  console.log('✓ Playing... (run: await window.__TAURI__.invoke("audio_stop_playback") to stop)');
}
quickTest();
EOF
echo "----------------------------------------"
echo ""
echo "Press Enter to start Tauri dev server..."
read

# Start Tauri dev
exec cd src-ui && npm run tauri dev
