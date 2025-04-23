#!/bin/bash
export RUSTY_V8_MIRROR=https://github.com/leanmendoza/rusty_v8/releases/download
V8_BINDING_FILE_NAME=src_binding_debug_aarch64-linux-android.rs
V8_BINDING=$RUSTY_V8_MIRROR/v0.105.1/$V8_BINDING_FILE_NAME
export RUSTY_V8_SRC_BINDING_PATH=$(pwd)/target/$V8_BINDING_FILE_NAME
# download if not exists
if [ ! -f "target/$V8_BINDING_FILE_NAME" ]; then
    curl -L -o target/$V8_BINDING_FILE_NAME $V8_BINDING
fi
export ANDROID_NDK_HOME=~/Android/Sdk/ndk/25.2.9519653/
GN_ARGS=use_custom_libcxx=false cargo ndk -t arm64-v8a build $1