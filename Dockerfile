FROM rust:1.80

ARG TARGET_PLATFORM

LABEL tool="spade-docker"

RUN apt-get -y update

# B. Install packages for swim tests:

#   1. Add APK packages
RUN apt-get install -y \
    gcc pkg-config python3-venv libssl-dev pipx iverilog \
    wget xz-utils git # verilator=4.106

#   2. Setup Python
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"
ENV PATH="${PATH}:/sbin"

#   3. Install pipx, Maturin 1.2.3
RUN pip install pipx
RUN pipx install maturin==1.2.3
RUN pipx ensurepath

#   4. Install Zig
ARG ZIG_VERSION
RUN wget https://ziglang.org/download/$ZIG_VERSION/zig-linux-$TARGET_PLATFORM-$ZIG_VERSION.tar.xz \
    && tar -xf zig-linux-$TARGET_PLATFORM-$ZIG_VERSION.tar.xz \
    && mv zig-linux-$TARGET_PLATFORM-$ZIG_VERSION /usr/local/zig \
    && ln -s /usr/local/zig/zig /usr/local/bin/zig \
    && rm zig-linux-$TARGET_PLATFORM-$ZIG_VERSION.tar.xz

# C. Spade
ARG SPADE_REV
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/spade
WORKDIR /home/spade
RUN git reset --hard $SPADE_REV
RUN cargo install --path spade-compiler

# D. Swim
WORKDIR /home
ARG SWIM_REV
RUN git clone https://gitlab.com/spade-lang/swim
WORKDIR /home/swim
RUN git reset --hard $SWIM_REV
RUN cargo install --path .

WORKDIR /home
