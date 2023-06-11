mkdir -p bin
ocaml ast.ml > out.c
clang -O2 out.c -o bin/out
./bin/out