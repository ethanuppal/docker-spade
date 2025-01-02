FROM rust:1.80

LABEL tool="spade-docker"

RUN apt-get -y update

RUN apt-get install -y gcc pkg-config libssl-dev python3-dev pipx iverilog \
    wget xz-utils git

RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"
ENV PATH="${PATH}:/sbin"

RUN pip install pipx
RUN pipx install maturin==1.2.3
RUN pipx ensurepath

ARG ZIG_VERSION
ARG ZIG_TARGET_PLATFORM
RUN wget https://ziglang.org/download/$ZIG_VERSION/zig-linux-$ZIG_TARGET_PLATFORM-$ZIG_VERSION.tar.xz \
    && tar -xf zig-linux-$ZIG_TARGET_PLATFORM-$ZIG_VERSION.tar.xz \
    && mv zig-linux-$ZIG_TARGET_PLATFORM-$ZIG_VERSION /usr/local/zig \
    && ln -s /usr/local/zig/zig /usr/local/bin/zig \
    && rm zig-linux-$ZIG_TARGET_PLATFORM-$ZIG_VERSION.tar.xz

ARG SPADE_GIT
ARG SPADE_REV
WORKDIR /home
RUN git clone $SPADE_GIT spade
WORKDIR /home/spade
RUN git reset --hard $SPADE_REV
RUN cargo install --path spade-compiler --target-dir /home/.local/rust-target-dir

WORKDIR /home
ARG SWIM_GIT
ARG SWIM_REV
RUN git clone $SWIM_GIT swim
WORKDIR /home/swim
RUN git reset --hard $SWIM_REV
RUN cargo install --path . --target-dir /home/.local/rust-target-dir

RUN if [ "$TARGETARCH" = "x86_64" ]; then swim install-tools; fi

WORKDIR /home
