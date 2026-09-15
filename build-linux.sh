#!/usr/bin/env bash
# Build the Linux playtest kit for Dying Earth.
#
# RUN THIS ON UBUNTU -- a real machine, a VM, or WSL. It cannot be run on the Windows host the game
# is developed on, and the reason is worth writing down so nobody spends an afternoon on it:
#
#   Bevy is taken with its default features, so on Linux it links against ALSA (audio) and udev
#   (input devices) through `alsa-sys` and `libudev-sys`. Both are C libraries found by pkg-config
#   at build time. Cross-compiling from Windows would need a full Linux sysroot carrying those
#   headers and shared objects, which neither `rustup target add x86_64-unknown-linux-gnu` nor a
#   Zig linker supplies. The 0.07.6 Linux build says "Built on Ubuntu 22.04" on its tin because that
#   is what it was: a native build.
#
# Usage, from the repository root:
#
#     bash build-linux.sh 0.08.1
#
# It leaves dist/dying-earth-<version>-linux/ ready to zip.

set -euo pipefail

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "usage: bash build-linux.sh <version>   e.g. 0.08.1" >&2
    exit 2
fi

ROOT="$(cd "$(dirname "$0")" && pwd)"
OUT="$ROOT/dist/dying-earth-$VERSION-linux"
cd "$ROOT"

echo "== 1. Build dependencies =="
# The packages the Bevy build needs on a clean Ubuntu. libasound2-dev and libudev-dev are the two
# that actually decide whether this works; the rest are the Vulkan and windowing stack.
sudo apt-get update
sudo apt-get install -y \
    build-essential pkg-config \
    libasound2-dev libudev-dev \
    libwayland-dev libxkbcommon-dev \
    libx11-dev libxcursor-dev libxrandr-dev libxi-dev \
    libvulkan1 mesa-vulkan-drivers

if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo is not on PATH. Install rustup from https://rustup.rs and re-run." >&2
    exit 1
fi

echo "== 2. Build, release =="
cargo build --release --locked

BIN="$ROOT/target/release/dying-earth"
# Exit status is not evidence: assert the artifact exists and is not empty before packaging it.
[ -s "$BIN" ] || { echo "no binary at $BIN" >&2; exit 1; }
echo "binary: $(stat -c '%s bytes' "$BIN")"

echo "== 3. Assemble the kit =="
rm -rf "$OUT"
mkdir -p "$OUT"
cp "$BIN" "$OUT/dying-earth"
chmod +x "$OUT/dying-earth"
cp -r "$ROOT/assets" "$OUT/assets"
# The two notes that ship beside it. README.txt is the playtest note, shared with the Windows kit.
cp "$ROOT/dist/dying-earth-$VERSION/README.txt" "$OUT/README.txt" 2>/dev/null \
    || echo "NOTE: no dist/dying-earth-$VERSION/README.txt yet; copy the playtest note in by hand."
cp "$ROOT/docs/playtest/RUN-ON-LINUX.txt" "$OUT/RUN-ON-LINUX.txt"

echo "== 4. Check it actually runs, headlessly =="
# NEVER run the game bare: with no shot: argument it starts a game and opens a window. The shot mode
# places the window off-screen, writes its captures and exits, which is also the only proof the
# binary works on this machine.
TMP="$(mktemp -d)"
( cd "$OUT" && ./dying-earth "shot:$TMP/check" window:1280x800 turns:0 seed:1 >/dev/null 2>&1 ) || true
if ls "$TMP"/check-*.png >/dev/null 2>&1; then
    echo "ran headlessly and wrote $(ls "$TMP"/check-*.png | wc -l) captures"
else
    echo "WARNING: the binary wrote no captures. It may still work on a desktop session -- a headless"
    echo "         build machine with no GPU cannot render -- but nothing here has proved it does."
fi
rm -rf "$TMP"

echo
echo "Done: $OUT"
echo "Zip it with:   cd dist && zip -r dying-earth-$VERSION-linux.zip dying-earth-$VERSION-linux"
