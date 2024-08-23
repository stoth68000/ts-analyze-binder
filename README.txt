
# Env
export LIBCLANG_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64 RUSTFLAGS="-C relocation-model=dynamic-no-pic"

# Clean env
cargo clean

# Build a debug build
cargo build

# Build a release build
cargo build --release

# Run a debug build
cargo run
