FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    bind9-utils \
    iproute2 \
    tcpdump \
    nftables \
    sqlite3 \
    socat \
    jq \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY bin/agentic-dns /usr/local/bin/agentic-dns
RUN chmod +x /usr/local/bin/agentic-dns

EXPOSE 8099

ENTRYPOINT ["/usr/local/bin/agentic-dns"]
CMD ["status"]
