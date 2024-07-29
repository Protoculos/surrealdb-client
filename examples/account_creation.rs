use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb_client::{ClientOptions, SurrealDBClient};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "result")]
    results: Vec<Account>,
    status: String,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    age: u32,
    name: String,
    phones: Vec<String>,
    id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let custom_options = ClientOptions::new(
        "accounts".to_string(),
        "accounts".to_string(),
        "http://localhost:8000/sql".to_string(),
        Some("root".to_string()),
        Some("root".to_string()),
    );
    let custom_client =
        SurrealDBClient::new(custom_options).expect("Failed to create custom client");

    let name: String = serde_json::from_value(john["name"].clone())?;
    let json_str = custom_client
        .execute_sql(
            format!(
                "CREATE account:`{}` CONTENT {{
            {}
        }}",
                &name,
                john.to_string()
            )
            .as_str(),
        )
        .await?;
    tracing::info!("{}", &json_str);

    let json_str = custom_client
        .execute_sql(format!("SELECT * FROM account:`{}`", &name).as_str())
        .await?;
    tracing::info!("{}", &json_str);

    let deserialized: Vec<Response> = serde_json::from_str(json_str.as_str())?;
    let acc = &deserialized[0].results[0];
    println!("Deserialized data: {:#?}", acc);

    Ok(())
}
