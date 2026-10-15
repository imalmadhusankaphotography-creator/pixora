#!/usr/bin/env bash
# Cross-build PIXORA for Windows x64 from Linux, with no Windows SDK: cargo-zigbuild links with
# zig's mingw-w64, -Zbuild-std builds Rust's std for the target from rust-src, and `zig rc` makes
# the icon/version resources (apps/*/build.rs link the .res files).
#
# Needs: rustup's rust-src (or the std sources linked into the sysroot), `pip install ziglang`,
# `cargo install cargo-zigbuild`.
#
#   packaging/pixora/build-windows.sh            # -> target/pixora-dist/PIXORA-<ver>-windows-x64.zip
#
# Env: CGU (codegen units, default 16), LTO (default off), JOBS (cargo -j), SKIP_BUILD=1 (package only).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
TARGET=x86_64-pc-windows-gnu
VERSION="$(awk '/^\[workspace.package\]/{p=1;next} /^\[/{p=0} p&&/^version/{gsub(/[" ]/,"",$0);sub("version=","");print;exit}' Cargo.toml)"
WORK="$ROOT/target/pixora-res"
DIST="$ROOT/target/pixora-dist"
mkdir -p "$WORK" "$DIST"
IFS=. read -r MA MI PA <<<"${VERSION%%-*}"

rc() { # rc <name> <description> <original filename>
  cat >"$WORK/$1.rc" <<EOF
#include <winver.h>
1 ICON "$ROOT/assets/app-icon/photocraft.ico"
VS_VERSION_INFO VERSIONINFO
FILEVERSION $MA,$MI,$PA,0
PRODUCTVERSION $MA,$MI,$PA,0
FILEOS VOS_NT_WINDOWS32
FILETYPE VFT_APP
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904b0"
    BEGIN
      VALUE "ProductName", "PIXORA"
      VALUE "FileDescription", "$2"
      VALUE "CompanyName", "Imal Madhusanka"
      VALUE "LegalCopyright", "Copyright (c) 2026 Imal Madhusanka (PIXORA) and the PhotoCraft authors. MIT OR Apache-2.0."
      VALUE "OriginalFilename", "$3"
      VALUE "FileVersion", "$VERSION"
      VALUE "ProductVersion", "$VERSION"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x409, 1200
  END
END
EOF
  python3 -m ziglang rc /fo "$WORK/$1.res" -- "$WORK/$1.rc"
}
rc pixora "PIXORA photo editor" PIXORA.exe
rc pixora-cli "PIXORA command-line interface" pixora-cli.exe

export PIXORA_WIN_RES="$WORK/pixora.res" PIXORA_CLI_WIN_RES="$WORK/pixora-cli.res"
export CRAFT_FONTS_DIR="$ROOT/packaging/pixora" CRAFT_FONTS_REQUIRED=1
export PHOTOCRAFT_BUILD_DATE="${PHOTOCRAFT_BUILD_DATE:-$(date -u +%F)}"
export PHOTOCRAFT_BUILD_SHA="${PHOTOCRAFT_BUILD_SHA:-$(git -C "$ROOT" rev-parse HEAD 2>/dev/null || true)}"
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS="${CGU:-16}" CARGO_PROFILE_RELEASE_LTO="${LTO:-false}"
export RUSTC_BOOTSTRAP=1
[ -n "${SKIP_BUILD:-}" ] || cargo zigbuild --release -Zbuild-std --target "$TARGET" ${JOBS:+-j "$JOBS"} -p photocraft -p photocraft-cli \
  --features photocraft/heif,photocraft-cli/heif

BIN="$ROOT/target/$TARGET/release"
STAGE="$DIST/PIXORA"
rm -rf "$STAGE"
mkdir -p "$STAGE"
cp "$BIN/photocraft.exe" "$STAGE/PIXORA.exe"
cp "$BIN/photocraft-cli.exe" "$STAGE/pixora-cli.exe"
cp "$ROOT/LICENSE-MIT" "$ROOT/LICENSE-APACHE" "$ROOT/NOTICE" "$ROOT/ATTRIBUTION.md" "$STAGE/"
cp "$ROOT/packaging/pixora/fonts/OFL-NotoSinhala.txt" "$STAGE/"
cp "$ROOT/packaging/pixora/README-PIXORA.txt" "$STAGE/README.txt"
cp "$ROOT/packaging/pixora/Create Desktop Shortcut.bat" "$STAGE/"
(cd "$DIST" && rm -f "PIXORA-$VERSION-windows-x64.zip" && zip -qr9 "PIXORA-$VERSION-windows-x64.zip" PIXORA)
echo "$DIST/PIXORA-$VERSION-windows-x64.zip"
