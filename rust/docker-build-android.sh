#!/bin/bash

if [[ -z "${ANDROID_NDK}" ]]; then
    # Tested with NDK 27.1.12297006
    if [[ -z "${ANDROID_SDK}" ]]; then
        ANDROID_NDK=$ANDROID_SDK/ndk/27.1.12297006
    else
        ANDROID_NDK=~/Android/Sdk/ndk/27.1.12297006
    fi
    ANDROID_NDK_HOME=$ANDROID_NDK
fi

echo "ANDROID_NDK: $ANDROID_NDK"
echo "ANDROID_SDK: $ANDROID_SDK"
echo "ANDROID_HOME: $ANDROID_HOME"

# Run the specified commands
export TARGET_CC=$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang
export TARGET_CXX=$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang++
export TARGET_AR=$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar
export CARGO_FFMPEG_SYS_DISABLE_SIZE_T_IS_USIZE=1
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang"
export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true

export CXXFLAGS="-v --target=aarch64-linux-android"
export RUSTFLAGS="-L${ANDROID_NDK}/toolchains/llvm/prebuilt/linux-x86_64/lib/aarch64-unknown-linux-musl"

# CI 
export RUSTY_V8_MIRROR=https://github.com/leanmendoza/rusty_v8/releases/download
V8_BINDING_FILE_NAME=src_binding_debug_aarch64-linux-android.rs
V8_BINDING=$RUSTY_V8_MIRROR/v0.105.1/$V8_BINDING_FILE_NAME
export RUSTY_V8_SRC_BINDING_PATH=$(pwd)/target/$V8_BINDING_FILE_NAME
# download if not exists
if [ ! -f "target/$V8_BINDING_FILE_NAME" ]; then
    curl -L -o target/$V8_BINDING_FILE_NAME $V8_BINDING
fi

GN_ARGS=use_custom_libcxx=false RUST_BACKTRACE=full cargo build --release --target aarch64-linux-android
GN_ARGS=use_custom_libcxx=false RUST_BACKTRACE=full cargo build --target aarch64-linux-android