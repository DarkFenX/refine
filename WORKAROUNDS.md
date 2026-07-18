# Workarounds
List of hacks used in the project due to external bugs, with intention to check it periodically and clean code up as issues get resolved.

#### HTTP
- Every item command which manipulates effect mode uses crate `serde_with` to work around `serde` [issue #1183](https://github.com/serde-rs/serde/issues/1183). Once it's resolved, hacks need to be removed, along with dependency on `serde_with`.
- To make `serde` compatible with `default_field_values`, lots of macros were expanded. Once `syn` [issue #1774](https://github.com/dtolnay/syn/issues/1774) and `serde` [issue #3084](https://github.com/serde-rs/serde/issues/3084), they should be removed, and derives restored. Look for "DFVTD" across all crates.
