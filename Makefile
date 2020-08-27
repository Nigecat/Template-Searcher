
build: obj/resources.res
	mkdir -p bin
	mkdir -p obj
	g++ src/main.cpp -o bin/Template\ Searcher.exe obj/resources.res

obj/resources.res:
	windres src/resources.rc -O coff -o obj/resources.res