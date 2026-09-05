FROM alpine:3.20

# Install required low-level networking and shell primitives
RUN apk add --no-cache \
    bash \
    coreutils \
    iproute2 \
    curl \
    jq \
    bind-tools

WORKDIR /app

# Copy binaries
COPY bin/ /usr/local/bin/

RUN chmod +x /usr/local/bin/*

# Set default entrypoint
ENTRYPOINT ["/usr/local/bin/agentic-dns"]
CMD ["status"]