docs-test:
	wasm-pack test --node --features web

docs-build-fast:
	wasm-pack build -d ./docs/reverse-regex  --dev --target web --features web
	rm ./docs/reverse-regex/.gitignore

docs-build: docs-test docs-build-fast

test:
	cargo test

run-dev:
	cargo run

build-release: test
	cargo build --release

server: docs-build
	python3 -m http.server 8000 -d docs