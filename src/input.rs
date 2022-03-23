use std::fs;
use std::path;

pub struct RunInput {
    pub mode: String,
    pub channel: String,
    pub edition: String,
    pub backtrace: bool,
    pub file_path: path::PathBuf,
}

pub struct FmtInput {
    pub edition: String,
    pub file_path: path::PathBuf,
}

pub struct ShareInput {
    pub file_path: path::PathBuf,
}

pub struct DownloadInput {
    pub id_or_url: String,
}

pub fn code_from_path(file_path: &path::PathBuf) -> Result<String, Box<std::error::Error>> {
    let body = fs::read_to_string(file_path)?;
    Ok(body)
}
