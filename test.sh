assert() {
    expected="$1"
    input="$2"

    ./9cc "$input" > tmp.s
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input -> $actual"
    else
        echo "$input -> $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 "12 + 34 - 5 "
assert 47 "5 + 6 * 7"
assert 15 "5 * (9 - 6)"
assert 2  "12 / 6"
assert 4  "(3 + 5) / 2"
assert 4  "2 * (-3) + 10"
assert 7  "-3 + 10"

assert 1  "2 * 4 + 1 == 9"
assert 0  "9 * 8 == -34 + 40"
assert 0  "5 * 6 / 2 != 7 + 8"
assert 1  "24 != 4 * 8"

assert 1  "2 < 2 + 3"
assert 0  "3 < 3"
assert 1  "2 * 3 > 1"
assert 0  "1 + 2 > 4"

assert 1  "3 >= 2"
assert 0  "2 >= 3"
assert 1  "3 <= 6"
assert 0  "6 <= 3"


echo OK