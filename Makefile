all: build

pre-build:
	mkdir -p bin
	mkdir -p obj

build: pre-build obj/resource.res
	g++ -std=c++17 src/main.cpp -o bin/Template\ Searcher.exe obj/resource.res

obj/resource.res:
	windres src/resource.rc -O coff -o obj/resource.res