use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

pub async fn get(url: &str, token: &str, arr: &str) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    println!("now thread is: {:?}", arr);
    headers.insert("X-PIXELPAI-TK", token.parse().unwrap());
    Ok(client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
    // Ok(reqwest::get(url).await?.json::<HashMap<String,String>>().await?)
}

pub async fn post(
    url: &str,
    token: &str,
    body: Value,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-PIXELPAI-TK", token.parse().unwrap());

    let mut data = HashMap::new();
    data.insert("k", "v");

    Ok(client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}


