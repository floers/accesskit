# slint-web example

This is a project for testing the web accessibility features.

## Requirements

### toolchain

rustup target add wasm32-unknown-unknown

### tools

```sh
cargo install just
```

```sh
cargo install wasm-pack
```

### web driver

<https://vrtgs.github.io/thirtyfour/getting-started/installation.html>

place drivers in your PATH.

## Testing and Developing

run the tests via:

```sh
just test
```

You can run just the server via:

```sh
just run
```

And rebuild the slint wasm app via (no need to restart the server):

```sh
just wasm
```
