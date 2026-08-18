# competitive-library

[![library-test](https://github.com/uesugi6111/competitive-library/actions/workflows/rust.yml/badge.svg)](https://github.com/uesugi6111/competitive-library/actions/workflows/rust.yml)
[![documentation](https://github.com/uesugi6111/competitive-library/actions/workflows/doc.yml/badge.svg)](https://github.com/uesugi6111/competitive-library/actions/workflows/doc.yml)

Algorithms and data structures for competitive programming in Rust.

- Rust 1.89.0
- Rust edition 2024
- Compatible with the current AtCoder and Codeforces Rust environments
- No external crate dependencies

## Categories

| Module | Contents |
| --- | --- |
| [`algorithm`](src/algorithm) | Sequence algorithms, prime sieves, cumulative sums, and convex hull trick |
| [`graph`](src/graph) | Shortest paths, spanning trees, LCA, SCC, and tree algorithms |
| [`math`](src/math) | GCD/LCM, modular arithmetic, primality testing, and permutations |
| [`other`](src/other) | Search utilities, random number generation, and Zobrist hashing |
| [`string`](src/string) | Aho-Corasick, Manacher, rolling hash, and Z algorithm |
| [`structure`](src/structure) | Union-find, tries, heaps, Fenwick/segment/sparse trees, and treap |

Generated API documentation is available at
[github.uesugi.app/competitive-library](https://github.uesugi.app/competitive-library/doc/competitive_library/).

## Verification

The repository pins the judge-compatible toolchain in `rust-toolchain.toml`.

```console
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets --release --offline
```

CI checks two judge profiles separately:

- AtCoder: Cargo release build in offline mode
- Codeforces: direct `rustc --edition=2024 -O` compilation

## Compatibility aliases

Previously published misspelled module paths remain available as deprecated aliases.
New code should use the corrected paths, such as `tree_diameter`, `manacher`,
`rolling_hash`, and `minimum_spanning_tree_*`.
