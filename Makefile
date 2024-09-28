docs-test-wasm:
	wasm-pack test --node

docs-build-fast-wasm:
	wasm-pack build -d ./docs/reverse-regex  --dev --target web
	rm ./docs/reverse-regex/.gitignore

docs-build-wasm: docs-test-wasm docs-build-fast-wasm

test:
	cargo test

run-dev:
	cargo run

build-release: test
	cargo build --release

server: docs-build-wasm
	python3 -m http.server 8000 -d docs