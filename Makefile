test-wasm:
	wasm-pack test --node

build-wasm: test-wasm
	wasm-pack build -d ./pkg --target web
	cd ./docs; npm install


build-wasm-fast:
	wasm-pack build -d ./docs/reverse-regex  --dev --target web
	cd ./docs; npm install

server:
	python3 -m http.server 8000 -d docs