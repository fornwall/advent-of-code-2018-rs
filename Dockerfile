FROM scratch
COPY target/x86_64-unknown-linux-musl/release/advent-of-code-2018 /
ENTRYPOINT ["/advent-of-code-2018"]
