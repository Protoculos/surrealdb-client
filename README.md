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
surrealdb_client = "1.5.5"
```

Here's a quick example:

// Using with default settings
```rust
use surrealdb_client::{SurrealDBClient, ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_client = SurrealDBClient::new(ClientOptions::default())?;
    let result = default_client.execute_sql("SELECT * FROM accounts").await?;
    println!("Result: {}", result);
    Ok(())
}
```

// Using with user settings
```rust
use surrealdb_client::{SurrealDBClient, ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let custom_options = ClientOptions::new(
        "crypto".to_string(),
        "crypto".to_string(),
        "http://localhost:8000/sql".to_string(),
        Some("root".to_string()),
        Some("root".to_string()),
    );
    let custom_client = SurrealDBClient::new(custom_options)?;
    let result = custom_client.execute_sql("SELECT * FROM accounts").await?;
    println!("Result: {}", result);
    Ok(())
}
```

## Compatibility

Version 1.x of this crate is compatible with SurrealDB version 1.x.

| Crate Version | Compatible SurrealDB Version |
|---------------|------------------------------|
| 1.5.5         | 1.5.4                        |
| 2.0.0         | v2.0.0-alpha.7               |

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.