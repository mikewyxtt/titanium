FROM trustworthysystems/sel4:latest-amd64

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    llvm \
    libssl-dev \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m titanium
USER titanium
WORKDIR /home/titanium/titanium-src

# Install Rust using rustup (Rust toolchain installer)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to the PATH
ENV PATH="/home/titanium/.cargo/bin:${PATH}"

# Verify the installation of Rust
RUN rustc --version
RUN cargo --version
