all: build

pre-build:
	mkdir -p bin
	mkdir -p obj

build: pre-build
	g++ src/main.cpp -o bin/Template\ Searcher.exe