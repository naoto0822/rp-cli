use crate::api::{APIClient, API};
use crate::api_types::{compile, execute, fmt};
use crate::input;
use crate::printer::Printer;

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
}
