#!/bin/bash
assert(){
    expected="$1"
    input="$2"

    ./target/debug/nine_cc "$input" > tmp.s 2> /dev/null
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

# ビルドする
cargo build

assert 0 0
assert 42 42
assert 21 '5+20-4'
assert 41 '12 + 34 - 5'
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 11 '-10 + 21'
assert 0 '-20 * 2 / 4 + 10'
assert 1 '1 == 1'
assert 0 '1 == 0'
assert 1 '1 < 2'
assert 0 '2 < 2'
assert 0 '3 < 2'
assert 1 '1 <= 2'
assert 1 '2 <= 2'
assert 0 '3 <= 2'
assert 1 '2 > 1'
assert 0 '2 > 2'
assert 0 '2 > 3'
assert 1 '2 >= 1'
assert 1 '2 >= 2'
assert 0 '2 >= 3'


echo "All test passed🎉"
