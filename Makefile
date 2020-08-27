.PHONE: build clean

build: pre-build obj/resources.res bin/config.ini
	g++ -std=c++17 src/main.cpp -o bin/Template\ Searcher.exe obj/resources.res

pre-build:
	mkdir -p bin
	mkdir -p obj

bin/config.ini: src/config.ini
	cp src/config.ini bin/config.ini

obj/resources.res: src/resources.rc
	windres src/resources.rc -O coff -o obj/resources.res

clean:
	rm -rf bin
	rm -rf obj