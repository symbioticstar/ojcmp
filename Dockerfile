FROM rust
COPY . /build
RUN cd /build && sh build.sh