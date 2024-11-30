FROM rust:1.70-alpine

# Install needed packages for swim tests
RUN apk update
RUN apk add --no-cache \
       python3 py3-pip py3-virtualenv gcc pkgconf openssl-dev iverilog zig
        # verilator=4.106-y 

# Setup python
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"
ENV PATH="${PATH}:/sbin"

# More packages for swim tests
RUN pipx install maturin==1.2.3
RUN pipx ensurepath

WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/spade

WORKDIR /home
RUN git clone https://gitlab.com/spade-lang/swim
RUN cd swim && cargo install --path .

WORKDIR /home

