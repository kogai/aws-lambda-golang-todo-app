FROM amd64/rust:1.23.0
ADD . /app
WORKDIR /app

RUN cargo install && \
  cargo build --release
