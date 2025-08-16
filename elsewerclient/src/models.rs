use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub(crate) token: String,
}

#[derive(Debug, Deserialize)]
pub struct BooksResponse {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub count: usize,
    #[serde(default)]
    pub books: Vec<AudioBookRow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetaResponse {
    pub count: usize,
    pub data: Vec<FileRow>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRow {
    pub id: i64,
    pub book_id: i64,
    pub file_id: Option<i64>,
    pub file_name: String,
    pub file_path: String,
    pub duration: Option<i64>,
    pub channels: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioBookRow {
    pub id: i64,
    pub author: String,
    pub series: Option<String>,
    pub title: String,
    pub files_location: String,
    pub duration: i64,
    pub cover_art: Option<String>,
    pub metadata: Option<String>,
}
