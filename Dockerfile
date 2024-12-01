FROM rust:1.70-alpine3.18

## Install packages for swim tests:

# 1. Add APK packages
RUN apk update
RUN apk add --no-cache \
       python3 py3-pip py3-virtualenv gcc pkgconf openssl-dev iverilog

# 2. Install ziglang
RUN apk add curl xz
ARG ZIGVER=0.13.0
RUN mkdir -p /deps
WORKDIR /deps
RUN curl https://ziglang.org/deps/zig+llvm+lld+clang-$(uname -m)-linux-musl-$ZIGVER.tar.xz  -O && \
    tar xf zig+llvm+lld+clang-$(uname -m)-linux-musl-$ZIGVER.tar.xz && \
    mv zig+llvm+lld+clang-$(uname -m)-linux-musl-$ZIGVER/ local/
RUN apk add --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/v3.20/community zig

# 3. Setup Python
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"
ENV PATH="${PATH}:/sbin"

# 4. Install pipx and Maturin 1.2.3
RUN pip install pipx
RUN pipx install maturin==1.2.3
RUN pipx ensurepath

# Spade
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/spade

# Swim
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/swim
RUN cd swim && cargo install --path .

WORKDIR /home
