build:
    wasm-pack build --target web

test:
    cargo test -- --no-capture

run: build
    python3 -m http.server
