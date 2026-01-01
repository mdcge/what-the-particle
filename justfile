# -*-Makefile-*-

test colours='':
     cargo {{colours}} nextest run

run:
	wasm-pack build --target web --mode no-install
	python -m http.server 8000
