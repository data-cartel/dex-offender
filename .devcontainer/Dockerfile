# FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as rust
FROM ghcr.io/foundry-rs/foundry as foundry

FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

USER vscode

# helper tools
RUN sudo apt update && sudo apt install -y jq tldr git

# foundry
COPY --from=foundry /usr/local/bin/forge /usr/local/bin/forge
COPY --from=foundry /usr/local/bin/cast /usr/local/bin/cast
COPY --from=foundry /usr/local/bin/anvil /usr/local/bin/anvil
COPY --from=foundry /usr/local/bin/chisel /usr/local/bin/chisel

ENV PATH=/usr/local/bin:$PATH

# rust
RUN cargo install cargo-watch