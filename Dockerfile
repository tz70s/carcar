FROM rust:1.19.0

# Install carcar
WORKDIR /usr/src/carcar
COPY . .
RUN cargo install

COPY ./model /model
