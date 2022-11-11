.PHONY: all debug format test 

all: debug format test

debug: 
	nim --gcc.exe:musl-gcc --gcc.linkerexe:musl-gcc c --mm:orc -d:debug --debugger:native --passL:-static --app:console --outdir:./bin ./src/main.nim	

format:
	nimpretty ./src/*.nim

test:
	testament pattern "tests/*.nim"

build:
	nim --gcc.exe:musl-gcc --gcc.linkerexe:musl-gcc c --mm:orc -d:danger --opt:speed --passL:-static --app:console --outdir:./bin ./src/main.nim	
