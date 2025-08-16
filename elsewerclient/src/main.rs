mod models;
use std::path::Path;

use axum::http::HeaderMap;
use reqwest::{Client, header::RANGE};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::models::{AudioBookRow, BooksResponse, FileMetaResponse, LoginRequest, LoginResponse};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let base_uri = "http://localhost:3000/api";

    //Get token
    let login_body = LoginRequest {
        username: "admin",
        password: "admin",
    };

    let resp = client
        .post(format!("{}/login", &base_uri))
        .json(&login_body)
        .send()
        .await?
        .error_for_status()?;

    let login_resp: LoginResponse = resp.json().await?;

    let token = format!("Bearer {}", login_resp.token);

    // Get first book
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", token.clone().parse().unwrap());

    let books: BooksResponse = client
        .get(format!("{}/list_books", base_uri))
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;

    let first_book_id: i64 = match books.books.first() {
        Some(b) => b.id,
        None => -99,
    };

    // Get first file
    let files: FileMetaResponse = client
        .get(format!("{}/file_metadata/{first_book_id}", base_uri))
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;
    // println!("{:#?}", files);

    let first_file_id = match files.data.first() {
        Some(f) => f.id,
        None => 0,
    };

    // Download first file

    let chunk_size: u64 = 1024 * 1024; // 1 MB chunks
    let output_path = Path::new("combined_file.m4b");

    // Create or truncate the output file
    // let mut output_file = OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .truncate(true)
    //     .open(&output_path)?;

    // Get total file size first
    let download_uri = format!("{base_uri}/download_chunk/{first_file_id}");
    let resp = client
        .head(&download_uri) // This is a type of req just like get post
        .headers(headers.clone())
        .send()
        .await?;

    let total_size = resp
        .headers()
        .get("content-length")
        .ok_or_else(|| anyhow::anyhow!("Missing content-length"))?
        .to_str()?
        .parse::<u64>()?;

    println!("Total file size: {}", total_size);

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_path)
        .await?;

    let mut start = 0;
    while start < total_size {
        let end = std::cmp::min(start + chunk_size - 1, total_size - 1);
        let range_header_value = format!("bytes={}-{}", start, end);

        headers.insert(RANGE, range_header_value.parse()?);

        let chunk_resp = client
            .get(&download_uri)
            .headers(headers.clone())
            .send()
            .await?;

        if !chunk_resp.status().is_success()
            && chunk_resp.status() != reqwest::StatusCode::PARTIAL_CONTENT
        {
            return Err(anyhow::anyhow!("Server did not return partial content"));
        }

        let chunk_bytes = chunk_resp.bytes().await?;
        output_file.write_all(&chunk_bytes).await?;

        println!("Downloaded bytes {}-{}", start, end);
        start = end + 1;
    }

    println!("File download complete: {:?}", output_path);
    anyhow::Ok(())
}
