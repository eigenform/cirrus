# cirrus - CIRCT Rust bindings

Not clear what I'm doing with this yet. Please note:

- This is experimental, probably broken, etc
- This expects you to locally clone and build CIRCT/LLVM
- This was written for using the FIRRTL dialect
- This was written/tested on a Linux machine

# Installation

`cirrus` depends on `cirrus-sys` to generate bindings. In order to build
`cirrus-sys`, you must define a `$CIRCT_PATH` environment variable which 
points to a directory containing the libraries/headers for CIRCT/LLVM.
For now, we expect that you're going to build both of these locally. 

First, clone the [llvm/circt](https://github.com/llvm/circt) repository
(which includes LLVM as a submodule). 
I'm currently building it with the following: 

```
$ CIRCT_SRC_DIR=${PWD}
$ mkdir build
$ cd build
$ cmake -G Ninja ../llvm/llvm \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_C_COMPILER=clang \
    -DCMAKE_CXX_COMPILER=clang++ \
    -DCMAKE_INSTALL_PREFIX=${CIRCT_SRC_DIR}/install \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DLLVM_ENABLE_PROJECTS=mlir \
    -DLLVM_EXTERNAL_PROJECTS=circt \
    -DLLVM_EXTERNAL_CIRCT_SOURCE_DIR=${CIRCT_SRC_DIR} \
    -DMLIR_INSTALL_AGGREGATE_OBJECTS=OFF \
    -DLLVM_ENABLE_ASSERTIONS=OFF \
    -DLLVM_ENABLE_BINDINGS=OFF \
    -DLLVM_ENABLE_OCAMLDOC=OFF \
    -DLLVM_INSTALL_UTILS=ON \
    -DLLVM_OPTIMIZED_TABLEGEN=ON \
    -DLLVM_STATIC_LINK_CXX_STDLIB=ON \
	-DLLVM_ENABLE_TERMINFO=OFF \
	-DVERILATOR_DISABLE=ON
$ ninja
$ ninja install
```

If you're working solely on *this* library, the Cargo configuration for 
this workspace sets `$CIRCT_PATH` to `./cirrus-sys/circt` (ignored by Git). 
You'll want to install CIRCT/LLVM there (or create a symlink).

Otherwise, users of this library are expected to create their own 
Cargo configuration (`.cargo/config.toml`) file defining `$CIRCT_PATH`. 
All crates that depend on `cirrus` are expected to pass this responsibility 
down to the user at build-time. 

You can set this up with something like:

```
# Create a project
$ cargo init --bin example
$ cd example

# Add this crate as a dependency
$ cargo add --git 'https://github.com/eigenform/cirrus'

# Create a Cargo configuration file
$ mkdir .cargo
$ touch .cargo/config.toml

# Set the appropriate value in '.cargo/config.toml', ie:

    [env]
    CIRCT_PATH = { value = "/opt/circt" }
```

# Other Notes

If you run into linking problems, you might have to make sure you're using 
`lld`. I needed to add this to the Cargo configuration for the workspace.
I think `cc` was failing to link against `stdc++` for whatever reason?

```
[target.x86_64-unknown-linux-gnu]
rustflags = [
    "-C", "link-arg=-fuse-ld=lld",
]
```

