# Dependency inventory

## Workspace crates

| Crate | Role | Status |
|---|---|---|
| mumble-web-proxy | application | Rust 1.89 / Edition 2024 |
| mumble-protocol | Mumble control/voice protocol | Rust 1.89 / protobuf 2.28 |
| rtp | RTP/RTCP/SRTP + RFC5764 | Rust 1.89 / modern crypto+Tokio |
| libnice | safe ICE bindings | Rust 1.89; GLib 0.10 API retained |
| libnice-sys | libnice FFI | Rust 1.89; bindgen 0.72 |
| webrtc-sdp | SDP/JSEP parser | Rust 1.89 |

## Native dependencies

Required on Linux:

- libnice development package (`nice.pc`)
- GLib 2.x development files
- OpenSSL development files
- pkg-config
- clang/libclang
- C compiler/build-essential

## Remaining legacy crates

The following were intentionally not blindly replaced because they are part of stable source APIs or examples:

- `argparse 0.2`
- `toml 0.5`
- `handy_async 0.2`
- `trackable 0.1`
- GLib/GObject/GIO Rust bindings 0.10
- protobuf 2.x

These are isolated and should be migrated only after the first real Rust 1.89 build identifies actual source/API breakage. This avoids mixing unrelated rewrites into the DTLS-SRTP migration.
