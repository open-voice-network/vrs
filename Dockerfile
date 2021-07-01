FROM rust:slim
COPY . /build-vrs/
RUN cd /build-vrs/ && rustup default nightly && rustup update && cargo build --release
LABEL Name=VRS Version=0.0.1
EXPOSE 8000
ENV ROCKET_ENV=stage
CMD ["/build-vrs/target/release/vrs"]
