# My Rust gRPC Server (with Tonic)

A high-performance gRPC server implemented in Rust using the `tonic` framework.

## Features

* **Protocol Buffers:**  Defines service contracts using `.proto` files for efficient communication.
* **Tonic Framework:**  Simplifies the creation of gRPC servers and clients in Rust.
* **Asynchronous:**  Leverages `tokio` for handling concurrent requests efficiently.
* **OpenTelemetry:** Export traces using otlp to jaeger and logs to loki.

## Getting Started

### Building and Running

   ```bash
   git clone https://github.com/pestebani/tonic-server
   cd tonic-server
   docker compose up
   ```

The server should be running on `localhost:50051`.