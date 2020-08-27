
build: pre-build obj/resources.res
	g++ src/main.cpp -o bin/Template\ Searcher.exe obj/resources.res

pre-build:
	mkdir -p bin
	mkdir -p obj

obj/resources.res:
	windres src/resources.rc -O coff -o obj/resources.res