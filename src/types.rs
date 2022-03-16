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
