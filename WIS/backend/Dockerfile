# temporary build image
FROM rust:1.31.0 as build
RUN rustup install nightly
COPY ./ ./
RUN cargo +nightly build --release

# final release image
FROM scratch
COPY --from=build ./target/release/wis-backend .
CMD ["./wis-backend"]
EXPOSE 8000/tcp
