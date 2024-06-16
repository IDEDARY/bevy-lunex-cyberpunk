cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/bevypunk.wasm