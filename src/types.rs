use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Request {
    #[serde(rename(serialize = "crateType"))]
    pub crate_type: String,
    pub tests: bool,
    pub mode: String,
    pub channel: String,
    pub edition: String,
    pub backtrace: bool,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    stderr: String,
    #[serde(default)]
    error: String,
}
