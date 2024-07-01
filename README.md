# AoC 2023 day12

## MacOS

The C++ part uses pstl which macOS clang does not offer. You can install gcc via homebrew and use it to compile the C++ part.

```bash
brew install gcc@14
brew install tbb
CC=gcc-14 CXX=g++-14 CXXSTDLIB=stdc++ RUSTFLAGS="-L/opt/homebrew/lib -L/opt/homebrew/opt/gcc/lib/gcc/current" cargo bench 
```

## Linux

On Linux you can install the TBB library needed for pstl via the package manager.

```bash
sudo apt install libtbb-dev
cargo bench 
```
