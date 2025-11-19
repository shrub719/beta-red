build:
    wasm-pack build --target web

test:
    cargo test -- --no-capture

run:
    python3 -m http.server
