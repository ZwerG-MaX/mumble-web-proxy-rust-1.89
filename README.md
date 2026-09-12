# Mumble Rust 1.89 — second migration pass

This workspace combines the six source trees needed by the original Mumble WebRTC proxy.

- `proxy` — mumble-web-proxy
- `protocol` — rust-mumble-protocol
- `rtp` — RTP/RTCP + RFC 5764 DTLS-SRTP
- `libnice` — safe Rust bindings
- `libnice-sys` — FFI bindings
- `sdp` — Mozilla `webrtc-sdp` (used instead of rsdparsa)

## Toolchain

- Rust 1.89.0
- Cargo resolver 3
- Edition 2024

## Native dependencies (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install build-essential pkg-config clang libclang-dev libnice-dev libglib2.0-dev libssl-dev
```

Check:

```bash
./scripts/check-native-deps.sh
```

Build/test:

```bash
cargo build --workspace --release
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Docker:

```bash
docker build -t mumble-rust-1.89 .
```

## Migration choices

`rsdparsa` is not included. `webrtc-sdp` 0.3.14 is the single SDP implementation.

The RTP source is the DTLS-SRTP tree containing `rtp::rfc5764::DtlsSrtp`; a generic RTP tree without RFC 5764 is not a drop-in replacement.

`libnice-sys` remains an FFI crate and therefore still needs the native `libnice` development package.

The high-level `rust-libnice` crate deliberately retains its original GLib 0.10 wrapper layer in this pass. A GLib 0.21 rewrite is a separate ABI/API migration and should be done with a real native build environment.

## Verification

This environment does not contain `cargo`/`rustc`, so a real compile and Cargo.lock generation cannot be performed here. The archive is therefore a source-level migration, not a claim of a completed build.
