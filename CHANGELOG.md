# Changelog

## Unreleased

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

## v0.1.0 (24/07/2026)

Initial release