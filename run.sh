cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/scary-maze-game.wasm .
basic-http-server .
