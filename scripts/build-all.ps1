write-host "running cargo build"
cargo build
write-host "building wasm"
cargo build --release --target wasm32-unknown-unknown
write-host "running wasm-bindgen"
wasm-bindgen --no-typescript --target web --out-dir wasm --out-name invaders .\target\wasm32-unknown-unknown\release\invaders.wasm
write-host "copying assets"
del wasm/assets -Recurse -Force -ErrorAction SilentlyContinue
copy assets wasm -Force -Recurse
