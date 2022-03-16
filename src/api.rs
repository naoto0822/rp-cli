use crate::types::{compile, execute};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

const BASE_URL: &str = "https://play.rust-lang.org/";
const PATH_EXECUTE: &str = "execute";
const PATH_COMPILE: &str = "compile";

pub trait API {
    fn execute(
        &self,
        req: execute::Request,
    ) -> Result<execute::Response, Box<dyn std::error::Error>>;
    fn compile(
        &self,
        req: compile::Request,
    ) -> Result<compile::Response, Box<dyn std::error::Error>>;
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
}
