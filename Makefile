build:
	cargo build -p tnt-server
	cargo build --package=front-wasm --lib --target=wasm32-unknown-unknown --no-default-features
	wasm-bindgen --target web --out-dir pkg \
	--no-typescript --out-name tarantool-leptos \
	./target/wasm32-unknown-unknown/debug/front_wasm.wasm
build-release:
	cargo build -p tnt-server --release
	cargo build --package=front-wasm --lib --target=wasm32-unknown-unknown --no-default-features --release
	wasm-bindgen --target web --out-dir pkg \
	--no-typescript --out-name tarantool-leptos \
	./target/wasm32-unknown-unknown/release/front_wasm.wasm
run: build
	tarantool-runner run -p ./target/debug/libtnt_server.so -e start -i tnt-server/src/init.lua
run-release: build-release
	tarantool-runner run -p ./target/release/libtnt_server.so -e start -i tnt-server/src/init.lua
test:
	cargo build -p tnt-server --features test
	tarantool-test -p ./target/debug/libtnt_server.so