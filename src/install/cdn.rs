use reqwest::Client;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

pub async fn download(url: &str, output_path: &PathBuf) -> Result<(), String> {
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }

    let mut file = tokio::fs::File::create(&output_path)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {}", e))?;
    }

    println!("Downloaded to: {}", output_path.display());

    Ok(())
}
