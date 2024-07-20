# SurrealDB Client

A Rust client library for interacting with SurrealDB.

## Features

- Simple and easy-to-use API
- Customizable client options
- Asynchronous SQL execution

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
surrealdb_client = "0.1.0"
```

Here's a quick example:

```rust
use surrealdb_client::{SurrealDBClient, ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SurrealDBClient::new(ClientOptions::default())?;
    let result = client.execute_sql("SELECT * FROM accounts").await?;
    println!("Result: {}", result);
    Ok(())
}
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.