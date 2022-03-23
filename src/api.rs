use crate::api_types::{compile, execute, fmt, share, download};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const BASE_URL: &str = "https://play.rust-lang.org/";
const PATH_EXECUTE: &str = "execute";
const PATH_COMPILE: &str = "compile";
const PATH_FMT: &str = "format";
const PATH_SHARE: &str = "meta/gist";
const PATH_DL: &str = "meta/gist";

pub trait API {
    fn execute(
        &self,
        req: execute::Request,
    ) -> Result<execute::Response, Box<dyn std::error::Error>>;
    fn compile(
        &self,
        req: compile::Request,
    ) -> Result<compile::Response, Box<dyn std::error::Error>>;

    fn fmt(&self, req: fmt::Request) -> Result<fmt::Response, Box<dyn std::error::Error>>;

    fn share(&self, req: share::Request) -> Result<share::Response, Box<dyn std::error::Error>>;

    fn download(&self, req: download::Request) -> Result<download::Response, Box<dyn std::error::Error>>;
}

pub struct APIClient {
    base_url: String,
}

impl APIClient {
    pub fn new() -> APIClient {
        APIClient {
            base_url: BASE_URL.to_string(),
        }
    }

    fn get<D>(&self, path: &str) -> Result<D, Box<dyn std::error::Error>> where D: DeserializeOwned, {
        let url = self.base_url.clone() + path;
        let res: D = reqwest::blocking::get(url)?.json()?;

        Ok(res)
    }

    fn post<S, D>(&self, path: &str, body: S) -> Result<D, Box<dyn std::error::Error>>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        let url = self.base_url.clone() + path;
        let client = reqwest::blocking::Client::new();
        let res: D = client.post(url).json(&body).send()?.json()?;

        Ok(res)
    }
}

impl API for APIClient {
    fn execute(
        &self,
        req: execute::Request,
    ) -> Result<execute::Response, Box<dyn std::error::Error>> {
        let res: execute::Response = self.post(PATH_EXECUTE, req)?;
        Ok(res)
    }

    fn compile(
        &self,
        req: compile::Request,
    ) -> Result<compile::Response, Box<dyn std::error::Error>> {
        let res: compile::Response = self.post(PATH_COMPILE, req)?;
        Ok(res)
    }

    fn fmt(&self, req: fmt::Request) -> Result<fmt::Response, Box<dyn std::error::Error>> {
        let res: fmt::Response = self.post(PATH_FMT, req)?;
        Ok(res)
    }

    fn share(&self, req: share::Request) -> Result<share::Response, Box<dyn std::error::Error>> {
        let res: share::Response = self.post(PATH_SHARE, req)?;
        Ok(res)
    }

    fn download(&self, req: download::Request) -> Result<download::Response, Box<dyn std::error::Error>> {
        let path = format!("{}/{}", PATH_DL, req.id);
        let res: download::Response = self.get(&path)?;

        Ok(res)
    }
}
