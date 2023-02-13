FROM ubuntu:22.04

# Update packages
RUN apt-get update

# Add sudo
RUN apt-get install -y sudo

# Add codespace user
RUN adduser --disabled-password --gecos '' codespace
RUN adduser codespace sudo
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER codespace
WORKDIR /home/codespace

# Get Ubuntu packages
RUN sudo apt-get install -y \
    build-essential \
    curl \
    git

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 

# Add .cargo/bin to PATH
ENV PATH="/home/codespace/.cargo/bin:${PATH}"

# Install ARM GCC
RUN sudo apt-get install -y gcc-arm-none-eabi

# Install elf2uf2-rs
# RUN cargo install elf2uf2-rs
RUN git clone https://github.com/rej696/elf2uf2.git &&\
    cd elf2uf2 &&\
    make &&\
    sudo cp elf2uf2 /usr/local/bin &&\
    cd .. &&\
    rm -r elf2uf2

# Install elf2tab from source
RUN cargo install --git https://github.com/tock/elf2tab.git

# Install runner from source
RUN cargo install --git https://github.com/tock/libtock-rs.git runner

# Install Rust nightly
RUN rustup install nightly-2022-10-22
