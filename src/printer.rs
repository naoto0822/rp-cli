use crate::api_types::{compile, download, execute, fmt, share};
use std::io::{self, Write};

// TODO
// pub enum OutputType {
//     Stdout,
//     File,
// }
//
// pub enum FormatType {
//     Table,
//     JSON,
// }

const DEFAULT_ROW_LEN: usize = 90;
const EXECUTION_TITLE: &str = "Execution";
const FORMAT_TITLE: &str = "Format";
const SHARE_TITLE: &str = "Share";
const STDOUT_TITLE: &str = "Standard Output";
const STDERR_TITLE: &str = "Standard Error";
const ERROR_TITLE: &str = "Error";
const DL_TITLE: &str = "Download";

pub struct Printer {
    // TODO: be trait
    writer: io::BufWriter<io::Stdout>,
    width: usize,
}

impl Printer {
    pub fn new() -> Printer {
        let stdout = io::stdout();
        let mut writer = io::BufWriter::new(stdout);

        Printer {
            writer: writer,
            width: DEFAULT_ROW_LEN,
        }
    }

    //
    // TODO: refactor
    //

    pub fn print_run(&mut self, res: execute::Response) -> Result<(), Box<dyn std::error::Error>> {
        if !res.is_error() {
            self.print_horizontal_line()?;
            self.print_header(EXECUTION_TITLE.to_string())?;
            self.print_horizontal_line()?;
            self.print_header(STDERR_TITLE.to_string())?;
            self.print_horizontal_line()?;
            self.print_body(res.stderr)?;
            self.print_horizontal_line()?;
            self.print_header(STDOUT_TITLE.to_string())?;
            self.print_horizontal_line()?;
            self.print_body(res.stdout)?;
            self.print_horizontal_line()?;
        } else {
            self.print_error(res.error)?;
        }

        Ok(())
    }

    pub fn print_fmt(&mut self, res: fmt::Response) -> Result<(), Box<dyn std::error::Error>> {
        if !res.is_error() {
            self.print_horizontal_line()?;
            self.print_header(EXECUTION_TITLE.to_string())?;
            self.print_horizontal_line()?;
            self.print_body(res.code)?;
            self.print_horizontal_line()?;
        } else {
            // TODO: res.code is empty when error
            self.print_error(res.code)?;
        }

        Ok(())
    }

    pub fn print_share(&mut self, res: share::Response) -> Result<(), Box<dyn std::error::Error>> {
        self.print_horizontal_line()?;
        self.print_header(SHARE_TITLE.to_string())?;
        self.print_horizontal_line()?;
        self.print_header("Permalink to the playground".to_string())?;
        self.print_horizontal_line()?;
        self.print_body(res.playground_url())?;
        self.print_horizontal_line()?;
        self.print_header("Direct link to the gist".to_string())?;
        self.print_horizontal_line()?;
        self.print_body(res.gist_url())?;
        self.print_horizontal_line()?;

        Ok(())
    }

    pub fn print_download(
        &mut self,
        res: download::Response,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.print_horizontal_line()?;
        self.print_header(DL_TITLE.to_string())?;
        self.print_horizontal_line()?;
        self.print_body(res.code)?;
        self.print_horizontal_line()?;

        Ok(())
    }

    fn print_error(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        self.print_horizontal_line()?;
        self.print_header(ERROR_TITLE.to_string())?;
        self.print_horizontal_line()?;
        self.print_body(message)?;
        self.print_horizontal_line()?;

        Ok(())
    }

    fn print_header(&mut self, title: String) -> Result<(), Box<dyn std::error::Error>> {
        let width = (DEFAULT_ROW_LEN - title.chars().count()) / 2;

        writeln!(
            self.writer,
            "{}{}{}",
            " ".repeat(width),
            title,
            " ".repeat(width),
        )?;

        Ok(())
    }

    fn print_horizontal_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.writer, "{}", "-".repeat(self.width))?;

        Ok(())
    }

    fn print_new_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.writer, "");

        Ok(())
    }

    fn print_body(&mut self, body: String) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.writer, "{}", body)?;

        Ok(())
    }
}
