# PLUG_AND_PLAY — Sort

> Sorting algorithms for ternary {-1, 0, +1} and general data

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-sort = { git = "https://github.com/SuperInstance/ternary-sort" }
```

Use in your code:

```rust
use ternary_sort::{ternary_counting_sort, ternary_quicksort};

let mut data = vec![0, -1, 1, 0, -1];
let sorted = ternary_counting_sort(&data);
```

## 📚 Available Documentation

| Document | Description |
|----------|-------------|
| `docs/FROM_BINARY.md` | Understanding ternary concepts as a binary programmer |
| `docs/MIGRATION.md` | Version migration guide |
| `docs/FUTURE-INTEGRATION.md` | Planned features and roadmap |

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
