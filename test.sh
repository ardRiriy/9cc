#!/bin/bash
assert(){
    expected="$1"
    input="$2"

    cargo run "$input" > tmp.s 2> /dev/null
    clang -masm=intel -o tmp tmp.s

    ./tmp
    actual="$?"

    if [ "$expected" = "$actual" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42

echo "All test passed"
