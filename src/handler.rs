use std::collections::HashMap;
use crate::api::{APIClient, API, BASE_URL};
use crate::api_types::{compile, execute, fmt, share, download};
use crate::input;
use crate::printer::Printer;
use url::{Url, ParseError};

pub struct Handler {
    // TODO: mockable for test
    api_cli: APIClient,
    printer: Printer,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            api_cli: APIClient::new(),
            printer: Printer::new(),
        }
    }

    pub fn run(&mut self, input: input::RunInput) -> Result<(), Box<dyn std::error::Error>> {
        let code = input::code_from_path(&input.file_path)?;

        // TODO: refactor for run_type
        let request = execute::Request {
            crate_type: "bin".to_string(),
            tests: false,
            mode: input.mode,
            channel: input.channel,
            edition: input.edition,
            backtrace: input.backtrace,
            code: code,
        };

        let resp = self.api_cli.execute(request)?;

        // TODO: handle following pattern.
        // error
        // Response { success: false, stdout: "", stderr: "", error: "Unable to deserialize request: Expected request with `Content-Type: application/json`" }
        //
        // error stdrtt
        // Response { success: false, stdout: "", stderr: "   Compiling playground v0.0.1 (/playground)\nerror: expected item, found `[`\n --> src/main.rs:1:1\n  |\n1 | [package]\n  | ^ expected item\n\nerror: could not compile `playground` due to previous error\n", error: "" }
        //
        // success
        // Response { success: true, stdout: "Client of The Rust Playground!\n", stderr: "   Compiling playground v0.0.1 (/playground)\n    Finished dev [unoptimized + debuginfo] target(s) in 2.66s\n     Running `target/debug/playground`\n", error: "" }
        //

        self.printer.print_run(resp)?;

        Ok(())
    }

    pub fn fmt(&mut self, input: input::FmtInput) -> Result<(), Box<dyn std::error::Error>> {
        let code = input::code_from_path(&input.file_path)?;

        let request = fmt::Request {
            edition: input.edition,
            code: code,
        };

        let resp = self.api_cli.fmt(request)?;

        self.printer.print_fmt(resp)?;

        Ok(())
    }

    pub fn share(&mut self, input: input::ShareInput) -> Result<(), Box<dyn std::error::Error>> {
        let code = input::code_from_path(&input.file_path)?;

        let request = share::Request { code: code };

        let resp = self.api_cli.share(request)?;

        self.printer.print_share(resp)?;

        Ok(())
    }

    pub fn download(&mut self, input: input::DownloadInput) -> Result<(), Box<dyn std::error::Error>> {
        let id = pick_id_from(input.id_or_url)?;

        let request = download::Request{
            id: id,
        };

        let resp = self.api_cli.download(request)?;

        self.printer.print_download(resp)?;

        Ok(())
    }
}

// TODO: error handling
fn pick_id_from(id_or_url: String) -> Result<String, Box<std::error::Error>> {
    if id_or_url.starts_with(BASE_URL) {
        // parse url
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7b61a4b7baf87fb4b5feb8668721ff8b
        let url = Url::parse(&id_or_url);
        if let Ok(url) = url {
            let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();

            match hash_query.get("gist") {
                Some(id) => return Ok(id.to_string()),
                None => return Ok("".to_string()),
            };
        } else {
            return Ok("".to_string())
        }
    } else {
        // maybe id
        Ok(id_or_url)
    }
}
