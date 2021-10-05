FROM rust:slim
RUN apt-get update && apt-get install -y \
build-essential checkinstall zlib1g-dev -y
COPY . /build-vrs/
RUN cd /build-vrs/ && rustup default nightly && rustup update && cargo build --release
LABEL Name=VRS Version=0.0.1
EXPOSE 8000
ENV ROCKET_ENV=stage
CMD ["/build-vrs/target/release/vrs"]
