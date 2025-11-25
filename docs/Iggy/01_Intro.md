# Apache Iggy

## Message Streaming

Message streaming is a distributed system that allows applications to send and receive streams of messages in real-time. It is particularly useful for applications that require real-time data processing, such as real-time analytics, IoT data collection, and microservices communication.

Message streaming is beneficial because it allows applications to process large amounts of data in real-time, without the need for storage or processing at the source. This can significantly reduce latency and improve the performance of applications. Additionally, message streaming platforms provide scalability, fault tolerance, and reliability, making it easier to build and maintain real-time data processing systems.

## System Design Example: Real-time Financial Analytics

In this example, we'll use message streaming to build a real-time financial analytics system. Here's how it works:

1. **Data Source**: We collect financial data from various sources such as stock exchanges, trading platforms, and financial databases.

2. **Data Ingestion**: We use message streaming platforms to receive the data in real-time. The data is then stored in a persistent log for processing.

3. **Data Processing**: We use real-time analytics engines to process the data in real-time. These engines perform calculations, aggregations, and statistical analysis on the data stream.

4. **Data Visualization**: We use real-time data visualization tools to display the analytics results in real-time. The visualizations are updated as new data arrives.

5. **User Interface**: We provide a user interface that allows users to view and interact with the real-time financial analytics.

Without message streaming, we would need to store and process the financial data at the source of the data, which could be challenging due to the large amount of data and the need for real-time processing. This could involve collecting and processing the data directly from the data sources, such as stock exchanges or trading platforms. This would result in increased latency and may require significant amounts of storage and processing resources to handle the volume of data. Additionally, it would be more difficult to scale the system to handle large volumes of data, as the processing and storage would be concentrated at the source.

On the other hand, with message streaming, we can process the financial data in real-time without the need for storage or processing at the source. This reduces latency and improves the performance of our real-time financial analytics system. Additionally, message streaming platforms provide scalability, fault tolerance, and reliability, making it easier to build and maintain real-time data processing systems.

To summarize, message streaming allows us to process large amounts of financial data in real-time without the need for storage or processing at the source. This significantly reduces latency and improves the performance of our real-time financial analytics system.

## Apache Iggy

Apache Iggy is a message streaming log for real-time data streaming between applications. It is an Apache Software Foundation project currently in the incubator stage. Its performance-first platform can process millions of messages per second with sub-millisecond latency.

## What is Iggy?

- **Persistent message streaming log** for real-time data streaming between applications
- **Apache Software Foundation project** currently in the incubator stage
- **Performance-first platform** capable of processing millions of messages per second with sub-millisecond latency

## Core Functionality

- **High-throughput messaging**: Handles over 5,000 MB/s throughput for both reads and writes
- **Multi-protocol support**: Supports QUIC, TCP, and HTTP protocols simultaneously
- **Multi-tenant architecture**: Organizes data in streams, topics, and partitions for different use cases
- **Persistent storage**: Provides durable, append-only log storage with configurable retention policies
- **Real-time communication**: Enables event-driven architectures and microservices communication

## Key Features

- **Rust-based**: Memory safety and zero-cost abstractions for maximum performance
- **Single binary deployment**: No external dependencies required
- **Advanced capabilities**:
  - User authentication
  - Message encryption
  - Server-side deduplication
  - Consumer groups
- **Multi-language SDKs**:
  - Rust
  - C#
  - Java
  - Python
  - Node.js
  - Go
  - And more
- **Enterprise features**:
  - S3 backup
  - OpenTelemetry monitoring
  - Prometheus metrics

## Common Use Cases

- Real-time analytics pipelines
- IoT data collection
- Microservices communication
- Log aggregation
- Event-driven architectures

## Why Choose Iggy Over Kafka?

Iggy competes with message streaming solutions like Kafka but differentiates itself through:

- Extreme performance optimizations
- Simplified deployment (single binary)
- Support for modern protocols like QUIC
- Focus on developer experience and operational simplicity
