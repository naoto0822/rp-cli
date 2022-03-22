pub mod execute {
    use serde::{Deserialize, Serialize};

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
        pub success: bool,
        #[serde(default)]
        pub stdout: String,
        #[serde(default)]
        pub stderr: String,
        #[serde(default)]
        pub error: String,
    }

    impl Response {
        pub fn is_error(&self) -> bool {
            !self.success && !self.error.is_empty()
        }
    }
}

pub mod compile {
    use serde::{Deserialize, Serialize};

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
        #[serde(rename(serialize = "assemblyFlavor"))]
        pub assembly_flavor: String,
        #[serde(rename(serialize = "demangleAssembly"))]
        pub demangle_assembly: String,
        #[serde(rename(serialize = "processAssembly"))]
        pub process_assembly: String,
        pub target: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        #[serde(default)]
        pub success: bool,
        #[serde(default)]
        pub stdout: String,
        #[serde(default)]
        pub stderr: String,
        #[serde(default)]
        pub error: String,
    }
}

pub mod fmt {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Request {
        pub edition: String,
        pub code: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        #[serde(default)]
        pub success: bool,
        #[serde(default)]
        pub stdout: String,
        #[serde(default)]
        pub stderr: String,
        #[serde(default)]
        pub code: String,
    }

    impl Response {
        pub fn is_error(&self) -> bool {
            !self.success
        }
    }
}

pub mod share {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Request {
        pub code: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        #[serde(default)]
        pub id: String,
        #[serde(default)]
        pub url: String,
        #[serde(default)]
        pub code: String,
    }

    impl Response {
        // Permalink to the playground
        pub fn playground_url(&self) -> String {
            // TODO: be const
            // TODO: relay version, mode, edition
            let url = format!(
                "https://play.rust-lang.org/?version={}&mode={}&edition={}&gist={}",
                "stable", "debug", "2021", self.id
            );

            url
        }

        // Direct link to the gist
        pub fn gist_url(&self) -> String {
            // TODO: be const
            let url = format!("https://gist.github.com/{}", self.id);

            url
        }

        // TODO: Embedded code in link

        // TODO: Open a new thread in the Rust user forum
    }
}

#[cfg(test)]
mod test {
    use super::execute;

    #[test]
    fn test_response_is_error() {
        let resp = execute::Response {
            success: false,
            stdout: "".to_string(),
            stderr: "".to_string(),
            error: "this is error".to_string(),
        };

        assert_eq!(true, resp.is_error());
    }
}
