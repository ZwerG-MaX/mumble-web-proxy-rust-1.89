# Second-pass migration notes

## Removed

- `rsdparsa` — replaced by `webrtc-sdp` 0.3.14.
- Generic/current RTP tree — replaced by the uploaded DTLS-SRTP tree containing RFC 5764.
- Legacy RTP `fibers`/`futures 0.1` example dependencies; the old `srtpsrv` example is preserved as `.disabled`.

## Updated

- Workspace resolver 3.
- Rust 1.89 / Edition 2024.
- `tokio-util` 0.7.
- `tokio-tungstenite` 0.28 / `tungstenite` 0.28.
- `http` 1.x.
- `tokio-openssl` 0.6.
- `bindgen` 0.72.
- protobuf code generation 2.28.
- RTP crypto: `aes-ctr` 0.6.0, `hmac` 0.12, `sha-1` 0.10, `fixedbitset` 0.5, `num-bigint` 0.4, `num-integer` 0.1.

## Source API changes

- `NewStreamCipher` / `SyncStreamCipher` -> `KeyIvInit` / `StreamCipher`.
- `new_var` -> `new_from_slices`.
- HMAC `new_varkey` -> `new_from_slice`.
- `Tokio02AsyncReadCompatExt` -> `TokioAsyncReadCompatExt`.
- `u16::max_value()` -> `u16::MAX`.
- Tungstenite `max_send_queue` -> write-buffer configuration.
- `Message::Binary(Vec<u8>)` construction -> `Message::binary(...)`.

## Deliberately deferred

`rust-libnice` still uses the original GLib 0.10 wrapper API. Upgrading that layer to GLib 0.21 should be a separate, native-build-verified rewrite.

## Rust 1.89 build-fix pass

The RTP crate was aligned with the historical `aes-ctr 0.6.0` API (`new_var`/`SyncStreamCipher`), which is the API actually used by the supplied RTP DTLS/SRTP source. The DTLS OpenSSL adapter was also updated for `tokio-openssl 0.6` by constructing `SslStream` and driving its async `connect`/`accept` methods. Rust 1.89's `next_multiple_of` call was updated to pass `16` by value, and the obsolete `tokio` cfg declaration was removed.

## Rust 1.89 build-fix pass (v7)

- Restored the historical `aes-ctr 0.6.0` `NewStreamCipher` trait import so `Aes128Ctr::new_var(...)` resolves under the current Rust 1.89 build.


## Rust 1.89 warning-cleanup pass

- Replaced deprecated `protobuf::parse_from_bytes` with the `Message` trait API.
- Suppressed lifetime warnings on generated protobuf modules at generation time rather than editing generated files.
- Removed redundant `mut` bindings in the RFC 5764 async I/O adapter.
- Made `StreamBuilder<'_>` and `ArgumentParser<'_>` lifetimes explicit.
- Added `Display`/`Error::source` implementations for the proxy error type so its wrapped error payloads are intentionally used.
- Scoped compatibility allowances for legacy GLib/libnice FFI code generated or expanded under Rust 2024's `unsafe_op_in_unsafe_fn` lint.
- Marked the callback-retention field in `AttachRecvHandle` as intentionally retained for its drop semantics.
