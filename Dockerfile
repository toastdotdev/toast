FROM rust
RUN rustup toolchain install nightly
COPY . /opt/app
WORKDIR /opt/app
RUN make build-production