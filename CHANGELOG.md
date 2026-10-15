# PIXORA changelog

## 0.4.0 — the PhotoCraft engine
PIXORA is rebuilt on [PhotoCraft](https://github.com/storytold/photocraft), the open-source Photoshop
reimplementation in Rust. Everything below comes with it.

- **67 tools** (marquees, lassos, Magic Wand, Quick/Object Selection, Select Subject and Sky, Crop and Perspective
  Crop, Spot Healing, Healing, Patch, Content-Aware Move, Remove, Clone, a real brush engine with pen pressure,
  Gradient, Dodge/Burn/Sponge, Pen and shape tools, Type), **groups, clipping and vector masks, smart objects
  with smart filters**, 27 blend modes, all Photoshop layer styles, 16 adjustment layers, Camera Raw, Liquify,
  the Blur Gallery and ~100 filters, actions, history snapshots, Photoshop's menus and shortcuts.
- **Files:** PSD/PSB (layered, round-trip), layered TIFF, RAW, JPEG, PNG, WebP, HEIC, OpenEXR and more; 8, 16 and
  32-bit; RGB, CMYK, Lab; ICC colour management. The native format is `.pcraft`; save layered work as `.psd`.
- **Fast:** GPU canvas (DirectX 12 / Vulkan), tiled copy-on-write documents, multithreaded filters.
- **PIXORA's own:** the name and icon; About credits Imal Madhusanka; Noto Sans/Serif Sinhala bundled
  (Sinhala shaping, ශ්‍රී and the zero-width joiner work in Type); settings in `%APPDATA%\PIXORA`.
- **PIXORA Link, rebuilt:** PIXORA starts its control channel itself (127.0.0.1:47811, token-protected,
  `%APPDATA%\PIXORA\link.json`). Other programs on this computer can open photos anywhere on the computer, run any of PIXORA's 900+
  commands, look at the picture (`doc.render`) and export. Programs that drive PIXORA through it can run commands, look at the picture and export.
- **Windows build from Linux** (`packaging/pixora/build-windows.sh`), and `BUILD_PIXORA.bat` for building on
  Windows with Rust.
- Projects saved as `.pixora` by PIXORA 0.1–0.3 don't open in 0.4; export them from 0.3 first.

## 0.1.0 – 0.3.1
The first PIXORA: a Python/Qt editor built from the Compositor sources (layers, adjustments, filters, the Type
tool with Sinhala, layer styles, a first local control link). Replaced by 0.4.0.
