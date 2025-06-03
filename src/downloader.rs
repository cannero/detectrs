use anyhow::Result;

pub async fn download_site(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        // user agent must be set, otherwise an error page is returned
        // from oracle.
        .header(reqwest::header::USER_AGENT, "curl/8.0.1")
        .send()
        .await?;
    let text = resp.text().await?;
    Ok(text)
}
