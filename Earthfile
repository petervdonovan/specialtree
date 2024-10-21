VERSION 0.8

IMPORT github.com/earthly/lib/rust:2.2.11 AS rust

FROM rust:1.82.0-bookworm

WORKDIR /repo

install:
    DO rust+INIT --keep_fingerprints=true

source:
    FROM +install
    COPY --keep-ts Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir langspec langspec-examples pattern sexprfmt langspec-gen-util langdatastructure-gen extensionof-gen ./

build:
    FROM +source
    DO rust+CARGO --args="build" --output="target/debug/[^/\.]+"
    SAVE ARTIFACT target AS LOCAL target

generate-tests:
    FROM +build
    RUN ls
    RUN ls target
    RUN cat langdatastructure-gen/examples/generate-tests.rs
    DO rust+CARGO --args="run --example generate-tests" --output="target/debug/[^/\.]+"
    SAVE ARTIFACT langdatastructure-gen/tests AS LOCAL langdatastructure-gen/tests

test:
    FROM +generate-tests
    BUILD +generate-tests
    DO rust+CARGO --args="test" --output="target/debug/[^/\.]+"
