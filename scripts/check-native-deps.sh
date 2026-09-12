#!/usr/bin/env bash
set -euo pipefail
command -v pkg-config >/dev/null || { echo "pkg-config is required"; exit 1; }
pkg-config --exists 'nice >= 0.1.0' || {
  echo "libnice development files not found (nice.pc). Install libnice-dev.";
  exit 1;
}
pkg-config --exists glib-2.0 || {
  echo "GLib development files not found. Install libglib2.0-dev.";
  exit 1;
}
pkg-config --exists openssl || {
  echo "OpenSSL development files not found. Install libssl-dev.";
  exit 1;
}
echo "Native dependencies OK"
