# AoC 2023 day12 Rust, C++ and Swift

The Day 12 challenge of the Advent of Code 2023 solved in Rust, C++ and Swift. It happens to be a
problem that can be solved efficiently using massive parallelism. For Rust rayon is used, for C++ pstl
plus tbb and for Swift Grand Central Dispatch.

## Features
To aid portability, all of Rust using rayon, the C++ part or the Swift part are bound to features:

| Feature | Description |
|---------|-------------|
| par | use rayon in the Rust part (note that C++ and Swift always use parallelism) |
| cpp | build the C++ part (see note below for requirements on macOS)|
| swift | build the Swift part (see note below for requirements of macOS, does not work on Linux)|

The default feature set is empty, but that allows divan to benchmark memory usage.

## cmake required for cpp and swift

The C++ and the Swift part are built using cmake.

## MacOS

To build the Swift part with cargo/cmake, the build system _ninja_ is required. You can install it via homebrew.

```bash
brew install ninja
```

The C++ part uses pstl which macOS *clang* does not offer. You can install *gcc* via homebrew and use it to compile the C++ part. And *tbb* is required for the C++ part.

```bash
brew install gcc@14 # or any other version, adjust CC and CXX below accordingly
brew install tbb
CC=gcc-14 CXX=g++-14 CXXSTDLIB=stdc++ \
RUSTFLAGS="-L$(brew --prefix)/lib -L$(brew --prefix gcc)/lib/gcc/current" cargo bench 
```

## Linux

Swift won't work on Linux. That's due to linking issues with the Swift runtime and/or the shared swift library. Neither a static nor a shared swift library works on Linux. However, if you have Swift installed on Linux, you may use the swift package manager to build and test the Swift part in the swift subdirectory. It includes a benchmark for both parts of the solution using XCTest. Run that with `swift test -c release`.

For the cpp part on Linux you need to install the *TBB* library needed for pstl via the package manager.

```bash
sudo apt install libtbb-dev
```

No special CXX/CC settings are required to run and bench the cpp part on Linux.
