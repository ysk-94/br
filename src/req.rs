use serde_json::{Value};

#[tokio::main]
pub async fn get() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.punkapi.com/v2/beers/random")
        .await?
        .text()
        .await?;

    let v: Value = serde_json::from_str(&resp)?;
    println!("{}", v[0]["name"]);
    Ok(())
}
