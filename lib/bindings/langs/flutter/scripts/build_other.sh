#!/bin/bash

# ! This script is not being used by Melos and is added for local testing !

# Setup
BUILD_DIR=platform-build
mkdir -p $BUILD_DIR
cd $BUILD_DIR

# Install build dependencies
cargo install cargo-zigbuild
cargo install cargo-xwin

zig_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    local PROFILE="$4"
    rustup target add "$TARGET"
    cargo zigbuild --package breez-liquid-sdk --target "$TARGET" --profile $PROFILE
    mkdir -p "$PLATFORM_NAME"
    cp "../../../../target/$TARGET/$PROFILE/$LIBNAME" "$PLATFORM_NAME/"
}

win_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    local PROFILE="$4"
    rustup target add "$TARGET"
    cargo xwin build --package breez-liquid-sdk --target "$TARGET" --profile $PROFILE
    mkdir -p "$PLATFORM_NAME"
    cp "../../../../target/$TARGET/$PROFILE/$LIBNAME" "$PLATFORM_NAME/"
}

PROFILE=frb-min
# Build all the dynamic libraries
LIBNAME=breez_sdk_liquid
LINUX_LIBNAME=lib$LIBNAME.so
zig_build aarch64-unknown-linux-gnu linux-arm64 $LINUX_LIBNAME $PROFILE
zig_build x86_64-unknown-linux-gnu linux-x64 $LINUX_LIBNAME $PROFILE
WINDOWS_LIBNAME=$LIBNAME.dll
win_build aarch64-pc-windows-msvc windows-arm64 $WINDOWS_LIBNAME $PROFILE
win_build x86_64-pc-windows-msvc windows-x64 $WINDOWS_LIBNAME $PROFILE

# Archive the dynamic libs
tar -czvf other.tar.gz linux-* windows-*

# Cleanup
rm -rf linux-* windows-*