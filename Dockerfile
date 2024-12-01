FROM ubuntu

# A. Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# B. Install packages for swim tests:

#   1. Add APK packages
RUN apt-get -y update
RUN apt-get install -y \
       gcc pkg-config python3-venv libssl-dev pipx iverilog \
       snapd wget xz-utils # verilator=4.106

#   2. Setup Python
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"
ENV PATH="${PATH}:/sbin"

#   3. Install pipx, Maturin 1.2.3
RUN pip install pipx
RUN pipx install maturin==1.2.3
RUN pipx ensurepath

#   4. Install Zig
ARG ZIG_VERSION="0.13.0"
RUN wget https://ziglang.org/download/$ZIG_VERSION/zig-linux-x86_64-$ZIG_VERSION.tar.xz \
    && tar -xf zig-linux-x86_64-$ZIG_VERSION.tar.xz \
    && mv zig-linux-x86_64-$ZIG_VERSION /usr/local/zig \
    && ln -s /usr/local/zig/zig /usr/local/bin/zig \
    && rm zig-linux-x86_64-$ZIG_VERSION.tar.xz

# C. Spade
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/spade

# D. Swim
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/swim
RUN cd swim && cargo install --path .

WORKDIR /home
