#!/bin/bash
# It assumes k8s pod servers are up and running.

server_ip="http://$(kubectl get node -o json | jq .items[0].status.addresses[0].address | sed 's/"//g'):30090/compilerapi/compile"
echo "Server IP: $server_ip"

assert() {
  expected="$1"
  input="$2"

  curl "$server_ip" --data "$input" > tmp.s || exit
  gcc -static -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 + 34 - 5 "
assert 47 "5+6*7"
assert 15 "5*(9-6)"
assert 4 "(3+5)/2"
assert 10 "-10+20"
assert 10 "- -10"
assert 10 "- - +10"

assert 0 '0==1'
assert 1 '42==42'
assert 1 '0!=1'
assert 0 '42!=42'

assert 1 '0<1'
assert 0 '1<1'
assert 0 '2<1'
assert 1 '0<=1'
assert 1 '1<=1'
assert 0 '2<=1'

assert 1 '1>0'
assert 0 '1>1'
assert 0 '1>2'
assert 1 '1>=0'
assert 1 '1>=1'
assert 0 '1>=2'

echo OK
