FROM golang:1.18.3-alpine

RUN apk add gcc && \
  mkdir /lib64 && \
  ln -s /lib/libc.musl-x86_64.so.1 /lib64/ld-linux-x86-64.so.2