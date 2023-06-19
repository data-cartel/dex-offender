FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as rust
FROM ghcr.io/foundry-rs/foundry as foundry

FROM ghcr.io/cachix/devenv:latest

# foundry
COPY --from=foundry /usr/local/bin/forge /usr/local/bin/forge
COPY --from=foundry /usr/local/bin/cast /usr/local/bin/cast
COPY --from=foundry /usr/local/bin/anvil /usr/local/bin/anvil
COPY --from=foundry /usr/local/bin/chisel /usr/local/bin/chisel

ENV PATH=/usr/local/bin:$PATH

# rust
COPY --from=rust /usr/local/cargo/bin /usr/local/cargo/bin
COPY --from=rust /usr/local/rustup /usr/local/rustup

ENV PATH=/usr/local/cargo/bin:$PATH