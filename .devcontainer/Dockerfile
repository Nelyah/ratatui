FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       curl xz-utils ca-certificates git sudo passwd \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 vscode \
    && useradd -m -u 1000 -g 1000 -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/vscode

RUN curl --proto '=https' --tlsv1.2 -sSf -L \
      https://install.determinate.systems/nix \
    | sh -s -- install linux --no-confirm --init none
ENV PATH="/nix/var/nix/profiles/default/bin:$PATH"

COPY flake.nix flake.lock /tmp/flake/
RUN nix build /tmp/flake#devtools -o /devtools

ENV PATH="/devtools/bin:$PATH"

# Install Node.js (needed for claude-agent-acp)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN npm install -g @zed-industries/claude-agent-acp

USER vscode
