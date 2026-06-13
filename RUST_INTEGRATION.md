# Integrating Rust with ngspice

This document explains how to integrate Rust code into the ngspice C codebase, enabling a hybrid C/Rust simulation engine.

## Overview

The integration uses the **Foreign Function Interface (FFI)**. Rust code is compiled into a **static library** (`.a`), which is then linked into the final `ngspice` binary by the Autotools build system.

## Project Structure

- `ngspice/src/rust_lib/`: The Rust crate directory.
  - `Cargo.toml`: Configured with `crate-type = ["staticlib"]`.
  - `src/lib.rs`: Contains the Rust implementation and FFI exports.
- `ngspice/src/Makefile.am`: Modified to trigger `cargo build` and link the library.
- `ngspice/src/main.c`: Modified to declare and call Rust functions.

## How to Add New Rust Functions

### 1. Write the Rust Code
In `ngspice/src/rust_lib/src/lib.rs`, define your function using the `extern "C"` calling convention and `#[no_mangle]` attribute to prevent Rust from changing the symbol name.

```rust
#[no_mangle]
pub extern "C" fn my_rust_function(input: i32) -> i32 {
    input * 2
}
```

### 2. Declare in C
In the relevant C file (e.g., `main.c` or a specific header), declare the function as `extern`.

```c
extern int my_rust_function(int);
```

### 3. Build
The build system is already configured. Simply run:
```bash
make
```
This will automatically run `cargo build --release` and relink `ngspice`.

## Build System Changes (Reference)

### `Cargo.toml`
The library MUST be a `staticlib`:
```toml
[lib]
crate-type = ["staticlib"]
```

### `Makefile.am`
The following rules were added to `ngspice/src/Makefile.am`:
- **Define Paths:**
  ```make
  RUST_LIB_PATH = $(top_srcdir)/src/rust_lib
  RUST_LIB = $(RUST_LIB_PATH)/target/release/libngspice_rust.a
  ```
- **Build Rule:**
  ```make
  $(RUST_LIB): $(RUST_LIB_PATH)/src/lib.rs $(RUST_LIB_PATH)/Cargo.toml
      cd $(RUST_LIB_PATH) && cargo build --release
  ```
- **Linking:**
  - Added `$(RUST_LIB)` to `ngspice_DEPENDENCIES`.
  - Added `$(RUST_LIB)` to `ngspice_LDADD`.
  - Added `-lpthread -ldl` to `LIBS` (required by the Rust standard library).

## Requirements
- Rust & Cargo (1.56+)
- Standard ngspice build dependencies (gcc, autoconf, etc.)
