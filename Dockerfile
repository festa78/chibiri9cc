FROM --platform=linux/amd64 ubuntu:focal

RUN apt update && \
    apt install -y \
    gcc make git binutils libc6-dev curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH $PATH:$HOME/.cargo/bin