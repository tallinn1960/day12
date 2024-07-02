# AoC 2023 day12 Rust, C++ and Swift

The Day 12 challenge of the Advent of Code 2023 solved in Rust, C++ and Swift. It happens to be a
problem that can be solved efficiently using massive parallelism. For Rust rayon is used, for C++ pstl
plus tbb and for Swift Grand Central Dispatch.

## MacOS

To build the Swift part with cargo, the build system _ninja_ is required. You can install it via homebrew.

```bash
brew install ninja
```

The C++ part uses pstl which macOS clang does not offer. You can install gcc via homebrew and use it to compile the C++ part.

```bash
brew install gcc@14 # or any other version
brew install tbb
CC=gcc-14 CXX=g++-14 CXXSTDLIB=stdc++ RUSTFLAGS="-L$(brew --prefix)/lib -L$(brew --prefix gcc@14)/lib/gcc/current" cargo bench 
```

## Linux

For the swift part you need to install the swift toolchain from [swift.org](https://swift.org).

On Linux you can install the TBB library needed for pstl via the package manager.

```bash
sudo apt install libtbb-dev
cargo bench 
```
