# Build stage
FROM rust:latest as builder

WORKDIR /app

COPY . .

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Go
RUN curl -OL https://go.dev/dl/go1.20.6.linux-amd64.tar.gz \
    && tar -C /usr/local -xzf go1.20.6.linux-amd64.tar.gz \
    && rm go1.20.6.linux-amd64.tar.gz

# Set Go environment variables
ENV PATH="${PATH}:/usr/local/go/bin"
ENV GOPATH="/go"

RUN go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway@latest
RUN go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2@latest
RUN go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
RUN go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

ENV PATH="${PATH}:${HOME}/go/bin"

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y ca-certificates
RUN apt-get install -y protobuf-compiler libprotobuf-dev libpq-dev

RUN cargo build --release --bin openapi-server

# Production stage
FROM debian:bookworm

WORKDIR /

RUN apt-get update && apt-get -y install libpq5

COPY --from=builder /app/target/release/openapi-server /usr/local/sbin/

EXPOSE 50051
EXPOSE 50052

CMD ["/usr/local/sbin/openapi-server"]