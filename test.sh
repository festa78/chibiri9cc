#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  bazel-bin/chibiri9cc/chibiri9cc "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

bazel run @rules_rust//:rustfmt
bazel build //chibiri9cc

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 + 34 - 5 "
assert 41 " 12 0 "

echo OK