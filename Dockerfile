FROM ubuntu

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install packages for swim tests:

# 1. Add APK packages
RUN apt-get -y update
RUN apt-get install -y \
       gcc pkg-config python3-venv libssl-dev pipx iverilog \
       snapd # verilator=4.106

# 2. Setup Python
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"
ENV PATH="${PATH}:/sbin"

# 3. Install pipx, Maturin 1.2.3, and Zig
RUN pip install pipx
RUN pipx install maturin==1.2.3
RUN pipx ensurepath
RUN snap install zig --classic --beta

# Spade
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/spade

# Swim
WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/swim
RUN cd swim && cargo install --path .

WORKDIR /home
