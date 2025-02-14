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
surrealdb_client = "2.0.0"
```

Here's a quick example:

_**Using with default settings:**_
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

_**Using with user settings:**_
```rust
use surrealdb_client::{SurrealDBClient, ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let custom_options = ClientOptions::new(
        "accounts".to_string(),  // ns
        "accounts".to_string(),  // db
        "http://localhost:8000/sql".to_string(), // url
        Some("root".to_string()),  // login
        Some("root".to_string()),  // password
    );
    let custom_client = SurrealDBClient::new(custom_options)?;
    let result = custom_client.execute_sql("SELECT * FROM accounts").await?;
    println!("Result: {}", result);
    Ok(())
}
```

## Examples

```
cargo run --example account_creation
```

## Compatibility

| Crate Version | Compatible SurrealDB Version |
|---------------|------------------------------|
| 1.5.5         | 1.5.4                        |
| 2.0.0         | v2.0.0-alpha.7               |

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.