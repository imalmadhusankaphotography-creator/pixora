PIXORA 0.4 — a Photoshop-style photo editor by Imal Madhusanka
================================================================

Start: double-click PIXORA.exe (no installation needed). "Create Desktop Shortcut.bat" adds PIXORA to the
desktop and the Start menu.

Settings, brushes and the PIXORA Link files live in %APPDATA%\PIXORA.

PIXORA Link
  PIXORA opens a local control channel when it starts (127.0.0.1:47811, token-protected), so other programs on this computer can open
  photos, edit, look at them and export. %APPDATA%\PIXORA\link.json tells them where PIXORA is.
  To turn it off, put an empty file named pixora-link-off in %APPDATA%\PIXORA.

pixora-cli.exe is the command-line tool (pixora-cli --help): batch processing, conversions, an MCP server.

PIXORA is built on PhotoCraft by the ArtCraft team and the PhotoCraft contributors (MIT OR Apache-2.0); it is
not made or endorsed by the ArtCraft Team. Licences: LICENSE-MIT, LICENSE-APACHE, NOTICE, ATTRIBUTION.md,
OFL-NotoSinhala.txt (the bundled Noto Sinhala fonts).
