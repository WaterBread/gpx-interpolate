use reqwest::Client;
use serde_json::Value;
use std::error::Error;

async fn fetch_map_track() -> Result<Value, Box<dyn Error>> {
    let url = "http://overpass-api.de/api/interpreter";

    let query = r#"[out:json][timeout:25];
    (
      way["highway"]({{bbox}});
    );
    out geom;"#;

    let query = query.replace("{{bbox}}", "50.745,7.17,50.75,7.18");

    let client = Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("data={}", query))
        .send()
        .await?;

    let json: Value = response.json().await?;

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_overpass_query() {
        let track = fetch_map_track().await.unwrap();
        println!("{:?}", track.to_string())
    }
}
