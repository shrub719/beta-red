build:
    wasm-pack build --target web

serve: build
    python3 -m http.server 

run:
    cargo run

[confirm]
publish:
    git switch dev
    git push
    git switch main
    git rebase dev
    git push
    git switch dev

