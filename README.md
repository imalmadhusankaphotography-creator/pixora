<p align="center"><img src="assets/app-icon/hicolor/128x128/apps/ai.storyteller.photocraft.png" width="96" alt="PIXORA"></p>

<h1 align="center">PIXORA</h1>

<p align="center"><b>A Photoshop-style photo editor for Windows, by Imal Madhusanka.</b><br>
Layers, masks, adjustment layers, layer styles, type (Sinhala too), vectors, brushes and real PSD files,
in a fast native app. Scriptable through a local control channel.</p>

PIXORA is built on [PhotoCraft](https://github.com/storytold/photocraft), the open-source, clean-room
Photoshop reimplementation in Rust by the ArtCraft team and the PhotoCraft contributors. PIXORA keeps its
engine and interface and adds its own name and icon, the PIXORA Link, file access for local automation anywhere on the
computer, a `doc.render` preview for assistants, bundled Noto Sinhala fonts and a Windows build made from
Linux. It is not made, sponsored or endorsed by the ArtCraft Team.

## What's in it

- **67 tools**: marquees, lassos, Magic Wand, Quick and Object Selection, Select Subject, Crop and Perspective
  Crop, Spot Healing, Healing, Patch, Content-Aware Move, Remove, Clone, brushes with a real brush engine,
  Gradient, Dodge/Burn/Sponge, Pen and shape tools, Type (horizontal and vertical).
- **Layers done properly**: groups, clipping masks, pixel and vector masks, fill and adjustment layers, smart
  objects with smart filters, 27 blend modes, layer styles (Drop Shadow, Glows, Bevel & Emboss, Satin, Stroke,
  Colour/Gradient/Pattern Overlay).
- **16 adjustments** (Levels, Curves, Hue/Saturation, Vibrance, Photo Filter, Black & White, Channel Mixer,
  Selective Color, Color Lookup with .cube/.3dl …) and **filters** (Gaussian/Lens/Motion blurs, Blur Gallery,
  Smart Sharpen, Reduce Noise, distortions (Twirl, Wave, Ripple …), Oil Paint, Lighting Effects …), Camera Raw.
- **Files**: PSD/PSB, layered TIFF, RAW, JPEG, PNG, WebP, HEIC (read), OpenEXR and more; 8/16/32-bit,
  RGB/CMYK/Lab, ICC colour management.
- **Photoshop shortcuts and menus**, a command palette (Ctrl+F), actions, history and snapshots.

## PIXORA Link

PIXORA starts a local control channel when it opens (127.0.0.1, port 47811 or the next free one), protected
by a token. `%APPDATA%\PIXORA\link.json` tells a client the port, the token file and where PIXORA.exe is, so
it can start PIXORA, open photos, edit them, look at them (`doc.render`) and export.

To turn it off: put an empty file named `pixora-link-off` in `%APPDATA%\PIXORA`, or set
`PIXORA_LINK=0`. The protocol is documented in [docs/control-protocol.md](docs/control-protocol.md).

## Build

Windows exe from Linux (no Windows SDK needed):

```sh
pip install ziglang && cargo install cargo-zigbuild
packaging/pixora/build-windows.sh        # -> target/pixora-dist/PIXORA-<version>-windows-x64.zip
```

On Windows with Rust (MSVC) installed: `BUILD_PIXORA.bat`, or `cargo run --release -p photocraft`.
Linux: `cargo run --release -p photocraft`. Tests: `cargo test --workspace`.

Internally the crates keep PhotoCraft's names (`photocraft-*`), so changes from PhotoCraft can still be
merged in. PhotoCraft's own README is in [docs/PhotoCraft-README.md](docs/PhotoCraft-README.md); its developer
documentation is in [docs/](docs/) and [AGENTS.md](AGENTS.md).

## License

MIT OR Apache-2.0, like PhotoCraft. Copyright (c) 2026 Imal Madhusanka (PIXORA) and the ArtCraft Team and the
PhotoCraft contributors. See [NOTICE](NOTICE) and [ATTRIBUTION.md](ATTRIBUTION.md).
