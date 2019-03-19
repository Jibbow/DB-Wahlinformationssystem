# temporary build image
FROM rust:1.31.0 as build

ARG BUILD_VERSION

# install MUSL wrapper for GCC
RUN apt-get update && apt-get install -y musl-tools
# nightly toolchain needed for this project (because of rocket)
RUN rustup install nightly
# add MUSL to get a 100% static binary (even without dynamic bindings to libc!)
# Important for copying the binary to a scratch docker image)
RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly
# Copy the code
COPY ./Cargo.toml ./Cargo.lock ./wis-backend/
COPY ./src/ ./wis-backend/src
COPY ./queries ./wis-backend/queries
# Compile the binary [$BUILD_VERSION is set by travis; not important for local development]
RUN BUILD_VERSION=${BUILD_VERSION} cargo +nightly build --release --target x86_64-unknown-linux-musl --manifest-path=wis-backend/Cargo.toml


# final release image
FROM scratch
COPY --from=build /wis-backend/target/x86_64-unknown-linux-musl/release/wis-backend /bin/wis-backend
CMD ["/bin/wis-backend"]
EXPOSE 8000
