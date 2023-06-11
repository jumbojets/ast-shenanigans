mkdir -p bin
rustc -A dead_code ast.rs -o bin/ast
./bin/ast > out.c
clang -O2 out.c -o bin/out
./bin/out