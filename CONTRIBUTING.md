# Contributing

Before opening a pull request, please run the following commands with `build.py`:
 - fmt
 - check
 - clippy
 - test
 - doc

To build the Vyxen Book, run:

```bash
build.py book
```

Or to serve the book locally, run:

```bash
build.py book --serve
```

If your change affects public API or behavior, please update the relevant documentation or examples as well.

For larger features, API changes, or engine architecture changes, please open an issue first so the direction can be discussed before implementation.

Bug reports should include reproduction steps when possible. Feature requests should explain the use case and how the change fits the engine.

Thanks for contributing to Vyxen.