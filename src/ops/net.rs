use std::str::FromStr;

use anyhow::Error;
use deno_core::op;
use reqwest::header::{HeaderMap, HeaderName};

#[op]
pub async fn get(url: String, headers: Vec<String>) -> Result<String, Error> {
    let mut request_headers = HeaderMap::new();
    for header in headers.iter() {
        let s = header.split(":").collect::<Vec<&str>>();
        if s.len() != 2 {
            continue;
        }

        request_headers.insert(HeaderName::from_str(s[0])?, s[1].parse()?);
    }

    let client = reqwest::Client::builder()
        .default_headers(request_headers.clone())
        .build()?;

    let request = client.get(url).headers(request_headers).build()?;
    let response = client.execute(request).await?;
    let response = response.text().await?;

    Ok(response)
}
