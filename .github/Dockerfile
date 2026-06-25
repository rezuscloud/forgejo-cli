# Minimal OCI image: the fj binary on a small runtime.
# Used by the GHCR publish job. The binary is statically-ish built in the
# workflow; this just packages it for container-based consumption.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 libgit2-1.7 && rm -rf /var/lib/apt/lists/*
COPY target/release/fj /usr/local/bin/fj
ENTRYPOINT ["fj"]
