### Getting Started with Apache Iggy: A Beginner's Guide

Apache Iggy is a high-performance message streaming platform. Think of it as a super-fast post office. You have:

*   **Producers**: Applications that send messages (letters).
*   **Consumers**: Applications that receive messages.
*   **Streams**: Mailboxes for different categories of messages.
*   **Topics**: Sub-categories within a stream (like "invoices" or "notifications").
*   **Partitions**: Ways to split topics for even faster processing.

Here’s how you can get started:

#### Step 1: Run the Iggy Server

The easiest way to start is by using Docker. This avoids building the project from the source code.

1.  **Install Docker**: If you don't have it, download and install Docker Desktop.
2.  **Pull the Iggy image**: Open your terminal and run:
    ```bash
    docker pull apache/iggy
    ```
3.  **Run the Iggy server**:
    ```bash
    docker run -d -p 8090:8090 -p 8080:8080 apache/iggy
    ```
    This command starts the Iggy server in the background. It will be accessible on your machine at `127.0.0.1:8090`.

#### Step 2: Set Up Your Rust Project

You'll need Rust installed. If you don't have it, go to [rust-lang.org](https://www.rust-lang.org/).

1.  **Create a new project**:
    ```bash
    cargo new iggy-example
    cd iggy-example
    ```
2.  **Add dependencies**: You'll need `iggy` for the client and `tokio` for asynchronous operations.
    ```bash
    cargo add iggy
    cargo add tokio --features full
    cargo add tracing tracing-subscriber
    ```
3.  **Project Structure**: The "Getting Started" guide recommends creating separate files for the producer and consumer.
    *   Create `src/producer.rs` and `src/consumer.rs`.
    *   Update `Cargo.toml` to define two separate binaries:

    ```toml
    [[bin]]
    name = "producer"
    path = "src/producer.rs"

    [[bin]]
    name = "consumer"
    path = "src/consumer.rs"
    ```

#### Step 3: Build the Producer

This application will send a message. Create the file `src/producer.rs` with the following code:

```rust
use iggy::client::Client;
use iggy::clients::client::IggyClient;
use iggy::messages::send_messages::{Message, SendMessages};
use iggy::models::header::{HeaderKey, HeaderValue};
use iggy::streams::create_stream::CreateStream;
use iggy::topics::create_topic::CreateTopic;
use iggy::users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let client = IggyClient::default();
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;
    let stream_id = 1;
    client
        .create_stream(&CreateStream {
            stream_id: Some(stream_id),
            name: "sample-stream".to_string(),
        })
        .await?;
    info!("Created stream with ID: {}", stream_id);
    let topic_id = 1;
    client
        .create_topic(&CreateTopic {
            stream_id: stream_id.try_into()?,
            topic_id: Some(topic_id),
            partitions_count: 1,
            name: "sample-topic".to_string(),
        })
        .await?;
    info!("Created topic with ID: {}", topic_id);
    let mut messages = Vec::new();
    let mut headers = HashMap::new();
    headers.insert(
        HeaderKey::new("key").unwrap(),
        HeaderValue::from_str("value").unwrap(),
    );
    let message = Message {
        id: 1,
        length: 13,
        payload: Arc::new("Hello, Iggy!".as_bytes().to_vec()),
        headers: Some(headers),
    };
    messages.push(message);
    client
        .send_messages(&mut SendMessages {
            stream_id: stream_id.try_into()?,
            topic_id: topic_id.try_into()?,
            partitioning: 0.try_into()?,
            messages,
        })
        .await?;
    info!("Sent message: 'Hello, Iggy!'");
    client.disconnect().await?;
    Ok(())
}
```

#### Step 4: Build the Consumer

This application will receive the message. Create the file `src/consumer.rs` with the following code:

```rust
use iggy::client::Client;
use iggy::clients::client::IggyClient;
use iggy::messages::poll_messages::{PollMessages, PollingStrategy};
use iggy::models::consumer::{Consumer, ConsumerKind};
use iggy::users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let client = IggyClient::default();
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;
    let stream_id = 1;
    let topic_id = 1;
    let partition_id = 1;
    let consumer = Consumer {
        kind: ConsumerKind::Consumer,
        id: 1,
    };
    loop {
        let messages = client
            .poll_messages(&PollMessages {
                consumer: consumer.clone(),
                stream_id: stream_id.try_into()?,
                topic_id: topic_id.try_into()?,
                partition_id: Some(partition_id),
                strategy: PollingStrategy::offset(0),
                count: 1,
                auto_commit: true,
            })
            .await?;
        if messages.messages.is_empty() {
            info!("No messages found");
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        for message in messages.messages {
            info!(
                "Received message: {}",
                String::from_utf8(message.payload.to_vec())?
            );
        }
    }
}
```

#### Step 5: Run Your Applications

1.  **Run the consumer**: In one terminal, run:
    ```bash
    cargo run --bin consumer
    ```
    It will start and wait for messages.

2.  **Run the producer**: In another terminal, run:
    ```bash
    cargo run --bin producer
    ```
    This will send the message "Hello, Iggy!". You should see the consumer terminal print that it has received the message.

### Summary

You have now:
1.  Started an Apache Iggy server.
2.  Created a Rust project with a producer and a consumer.
3.  Sent a message from the producer.
4.  Received the message in the consumer.

This is the foundation of message streaming. From here, you can explore more advanced topics like consumer groups, different data types, and error handling.
