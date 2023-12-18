FROM debian:12
RUN apt update
RUN apt install libssl-dev ca-certificates -y
COPY target/x86_64-unknown-linux-gnu/release/fj /usr/local/bin/fj
