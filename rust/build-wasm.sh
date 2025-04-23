#!/bin/sh
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

cd `dirname "$0"`

# We build the host gdextension first so that the godot editor doesn't complain.
cargo +nightly build --package explorer-tech-test &&
cargo +nightly build --package explorer-tech-test --target wasm32-unknown-emscripten -Zbuild-std $@
