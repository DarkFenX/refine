# Workarounds
List of hacks used in the project due to external bugs, with intention to check it periodically and clean code up as issues get resolved.

#### De/serialization
- `serde_json` normally allows integer map keys. However, this breaks in certain cases due to [issue #1183](https://github.com/serde-rs/serde/issues/1183). Once it's resolved, look for code which uses `DisplayFromStr`, and replace it by `_`.
