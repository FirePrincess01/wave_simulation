curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup override set nightly

cargo build 
cargo run


cargo install wasm-pack
wasm-pack build --target web 
python3 -m http.server 8000