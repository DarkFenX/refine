# Workarounds
List of hacks used in the project due to external bugs, with intention to check it periodically and clean code up as issues get resolved.

#### HTTP
- Every item command which manipulates effect mode uses crate `serde_with` to work around `serde` [issue #1183](https://github.com/serde-rs/serde/issues/1183). Once it's resolved, hacks need to be removed, along with dependency on `serde_with`.
- `educe` is used only to provide alternate defaults for some fields. After `serde` is made compatible with [RFC 3681: Default field values](https://github.com/rust-lang/rust/issues/132162) (possibly by resolving [issue #368](https://github.com/serde-rs/serde/issues/368)), educe dependency can be removed in favor of that.
- `#[serde(default = "...")]` is using functions to define default literals, change to literals once [issue #368](https://github.com/serde-rs/serde/issues/368) is implemented.
