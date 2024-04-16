# Error

## Message

```shell
/ 💼 Waiting to start building the project...                                                                                                                                                                      Listening on 127.0.0.1:8080
thread 'main' panicked at /xxx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dioxus-fullstack-0.5.2/src/serve_config.rs:111:37:
Failed to find index.html. Make sure the index_path is set correctly and the WASM application has been built.: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
| ⚙️ Compiling ipnet 2.9.0 (registry+https://github.com/rust-lang/crates.io-index)                                                                                                                                  The following warnings were emitted during compilation:


error: failed to run custom build command for `secp256k1-sys v0.9.2`

Caused by:
  process didn't exit successfully: `/xxxxxx/capybastr/.dioxus/web/debug/build/secp256k1-sys-eecfdd96989127ae/build-script-build` (exit status: 1)
  --- stdout
  TARGET = Some("wasm32-unknown-unknown")
  OPT_LEVEL = Some("0")
  HOST = Some("aarch64-apple-darwin")
  cargo:rerun-if-env-changed=CC_wasm32-unknown-unknown
  CC_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CC_wasm32_unknown_unknown
  CC_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  cargo:rerun-if-env-changed=CFLAGS_wasm32-unknown-unknown
  CFLAGS_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32_unknown_unknown
  CFLAGS_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=CC_wasm32-unknown-unknown
  CC_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CC_wasm32_unknown_unknown
  CC_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32-unknown-unknown
  CFLAGS_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32_unknown_unknown
  CFLAGS_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=CC_wasm32-unknown-unknown
  CC_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CC_wasm32_unknown_unknown
  CC_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32-unknown-unknown
  CFLAGS_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32_unknown_unknown
  CFLAGS_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:warning=error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-unknown"'
  cargo:warning=1 error generated.
  cargo:rerun-if-env-changed=CC_wasm32-unknown-unknown
  CC_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CC_wasm32_unknown_unknown
  CC_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32-unknown-unknown
  CFLAGS_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32_unknown_unknown
  CFLAGS_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=CC_wasm32-unknown-unknown
  CC_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CC_wasm32_unknown_unknown
  CC_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32-unknown-unknown
  CFLAGS_wasm32-unknown-unknown = None
  cargo:rerun-if-env-changed=CFLAGS_wasm32_unknown_unknown
  CFLAGS_wasm32_unknown_unknown = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:warning=error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-unknown"'
  cargo:warning=1 error generated.

  --- stderr


  error occurred: Command "clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=wasm32-unknown-unknown" "-I" "depend/secp256k1/" "-I" "depend/secp256k1/include" "-I" "depend/secp256k1/src" "-I" "wasm/wasm-sysroot" "-I" "wasm/wasm-sysroot" "-Wall" "-Wextra" "-DSECP256K1_API=" "-DENABLE_MODULE_ECDH=1" "-DENABLE_MODULE_SCHNORRSIG=1" "-DENABLE_MODULE_EXTRAKEYS=1" "-DENABLE_MODULE_ELLSWIFT=1" "-Dprintf(...)=" "-DECMULT_GEN_PREC_BITS=4" "-DECMULT_WINDOW_SIZE=15" "-DUSE_EXTERNAL_DEFAULT_CALLBACKS=1" "-o" "/xxxxxx/capybastr/.dioxus/web/wasm32-unknown-unknown/debug/build/secp256k1-sys-ef782783256ffa61/out/3fec8a5f3c4f77fb-wasm.o" "-c" "wasm/wasm.c" with args clang did not execute successfully (status code exit status: 1).


Error: 🚫 Serving project failed:

Caused by:
    Build failed
```

## Solution

- [Install LLVM](https://github.com/llvm/llvm-project/releases)
- [Install MSVC (Windows xx SDK ...)](https://visualstudio.microsoft.com/)
