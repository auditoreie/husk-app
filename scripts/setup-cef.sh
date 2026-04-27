#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Auditore
#
# Downloads the Chromium Embedded Framework (CEF) binaries needed by the
# `husk-cef-spike` crate. CEF binaries are NOT redistributed in this repo —
# they are huge (~600 MB compressed, ~1.2 GB extracted) and licensed under
# the BSD license owned by Marshall A. Greenblatt.
#
# After running this script, set CEF_PATH=$HOME/.local/share/cef before
# building the spike crate. The README documents the full flow.

set -euo pipefail

CEF_RS_REPO="${CEF_RS_REPO:-https://github.com/tauri-apps/cef-rs}"
CEF_RS_REF="${CEF_RS_REF:-dev}"
CEF_OUT_DIR="${CEF_OUT_DIR:-$HOME/.local/share/cef}"
WORKDIR="$(mktemp -d -t husk-cef-rs.XXXXXX)"
trap 'rm -rf "$WORKDIR"' EXIT

uname_s=$(uname -s)
uname_m=$(uname -m)

case "$uname_s/$uname_m" in
  Darwin/arm64) ;;
  Darwin/x86_64)
    echo "warn: husk-cef-spike currently targets macOS arm64 only. x86_64 may work but is untested." >&2
    ;;
  *)
    echo "error: husk-cef-spike currently supports macOS only (got $uname_s/$uname_m)." >&2
    exit 2
    ;;
esac

if [[ -d "$CEF_OUT_DIR" && -n "$(ls -A "$CEF_OUT_DIR" 2>/dev/null)" ]]; then
  echo "info: $CEF_OUT_DIR already populated. Set FORCE=1 to wipe and re-download."
  if [[ "${FORCE:-0}" != "1" ]]; then
    echo "info: skipping. Export CEF_PATH=$CEF_OUT_DIR and proceed to cargo build."
    exit 0
  fi
  rm -rf "$CEF_OUT_DIR"
fi

mkdir -p "$CEF_OUT_DIR"

echo "==> Cloning $CEF_RS_REPO ($CEF_RS_REF) into temporary workdir"
git clone --depth=1 --branch "$CEF_RS_REF" "$CEF_RS_REPO" "$WORKDIR/cef-rs"

echo "==> Running export-cef-dir to download + extract CEF binaries (~600 MB)"
echo "    target: $CEF_OUT_DIR"
(
  cd "$WORKDIR/cef-rs"
  cargo run --release -p export-cef-dir -- --force "$CEF_OUT_DIR"
)

echo
echo "==> Done. CEF binaries are at: $CEF_OUT_DIR"
echo
echo "Next steps:"
echo "  export CEF_PATH=\"$CEF_OUT_DIR\""
echo "  cargo check -p husk-cef-spike"
echo
echo "On macOS you may also need:"
echo "  export DYLD_FRAMEWORK_PATH=\"\$CEF_PATH/Release\""
