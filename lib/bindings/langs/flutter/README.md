# breez_sdk_liquid_workspace

Binding generation & build scripts for Dart/Flutter packages of Breez Liquid SDK.

## Prerequisites
### Required
This guide assumes you have the following tools installed on any development machines:
- [Flutter](https://docs.flutter.dev/get-started/install)
- [rustup](https://rustup.rs)
- [just](https://github.com/casey/just?tab=readme-ov-file#installation) command runner.
  - [Melos](https://melos.invertase.dev) which will be installed as part of `just bootstrap`.

### Optional
If you would like to build your binaries (for Flutter devices) locally in addition to CI 
(say, to test on a real device or emulator), you will additionally need the following:
- To compile to macOS/iOS targets
  - macOS
- To cross-compile to Android targets
  - [Android NDK](https://developer.android.com/ndk/downloads)
    - Most NDK versions should work nowadays due to fixes in `cargo-ndk`
      - Previously, NDK version 21 (`r21e`) was the only one that could be used easily
        - You might see reference to this elsewhere, but that is largely out of date
      - NDK version 25 (`r25b`) was working at the time of writing this documentation
- To cross-compile to Windows/Linux targets
  - [Zig](https://ziglang.org/learn/getting-started/#installing-zig)
  - llvm (with `clang-cl`!)
    - Need to run `brew install llvm` on macOS since Apple's llvm doesn't have it

## Getting Started
Run `just bootstrap true true` to initialize your workspace.
First parameter installs frb dependencies, second parameter builds the libraries.

To see all available recipes, run `just`.

## Troubleshooting
- [flutter_rust_bridge > Troubleshooting](https://cjycode.com/flutter_rust_bridge/manual/troubleshooting)
- [flutter_rust_bridge > Ffigen Troubleshooting](https://cjycode.com/flutter_rust_bridge/manual/ffigen-troubleshooting)


## License

Dual-licensed under Apache 2.0 and MIT.
