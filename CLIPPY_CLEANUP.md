# Clippy cleanup pass

Base: `mumble-rust-1.89-warning-clean-v3.zip`

This pass addresses the Clippy diagnostics from the supplied `clippy.log`,
including the large mechanical groups promoted to errors by `-D warnings`:

- redundant struct field names
- needless borrows
- needless `&` on constructor arguments
- `matches!` opportunities
- derived `Default` implementations
- collapsible `else if`
- `RangeInclusive::contains`
- single-pattern `if let`
- `std::io::Error::other`
- unnecessary casts
- unit-struct construction without `.default()`

Verify in the Rust 1.89 environment with:

```text
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

- protocol/build.rs: removed two needless borrows in protobuf_codegen inputs/includes.
