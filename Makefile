test-wasm:
	wasm-pack test --node

build-wasm-fast:
	wasm-pack build -d ./docs/reverse-regex  --dev --target web
	rm ./docs/reverse-regex/.gitignore

build-wasm: test-wasm 
	make build-wasm-fast

server:
	python3 -m http.server 8000 -d docs