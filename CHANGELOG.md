# Changelog

## Unreleased

### New features

- Texture inverting and grayscaling.
- Audio playback

### Fixes

- Textures not on a multiple of 16 bytes, causing wasm builds to panic.
- PNG alpha channel being ignored and being colored as black

## v0.2.1 (28/09/2026)

### Fixes

- Debug rendering not having the correct amount of triangles rendered
- If a polygon has less than 3 vertices, it is skipped instead of causing a panic

## v0.2.0 (21/09/2026)

### New Features

- Resource loader
- Ui elements
  - Images
  - Text
- Ui rendering
- Font rendering
- Delta time and FPS
- Examples
- Screen to world Vector conversion
- CI tests
- Texture and text tinting
- Vyxen Book
- Error handling
- Frame capping
- Debug render mode

### Fixes

- Textured being rendered 180 degrees
- Window resizing becoming 0 pixels
- WASM build
- Nodes disappearing on collision
- Panicing if vertices aren't in 4 byte alignments
- Shell scripts not running the correct targets
- Sprites and Ui Elements not unloading

### Other

- Privatized renderer backend
- Small optimizations on renderer sorting and copying sprites
- Expanded Texture loading to instead use `png` and `zune_jpeg`
- Updated crates to latest versions
- Removed `RigidBody` and `SoftBody` is_static arguments
- Rewrote bash scripts to use python
- Changed empty nodes to be not affected by gravity

## v0.1.0 (24/07/2026)

Initial release