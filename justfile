# Show this help page
help:
    just --list

# Build the whole workspace for linux wayland 
build-linux:
    cargo build --features winit/wayland

test:
    cargo test
    