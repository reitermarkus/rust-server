FROM scratch

COPY target/x86_64-unknown-linux-musl/release/server /
CMD ["/server"]

COPY www /www
