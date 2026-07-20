# Contributing

Before opening a pull request, please run:

```bash
cargo fmt
./scripts/check_every_target.sh
./scripts/clippy_every_target.sh
cargo test
```

If your change affects public API or behavior, please update the relevant documentation or examples as well.

For larger features, API changes, or engine architecture changes, please open an issue first so the direction can be discussed before implementation.

Bug reports should include reproduction steps when possible. Feature requests should explain the use case and how the change fits the engine.

Thanks for contributing to Vyxen.