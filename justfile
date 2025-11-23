build:
    wasm-pack build --target web

serve: build
    python3 -m http.server 

i:
    cargo run

