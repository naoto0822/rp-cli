use std::io::{self, Write};

use crate::types::{compile, execute};

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
const STDOUT_TITLE: &str = "Standard Output";
const STDERR_TITLE: &str = "Standard Error";
const ERROR_TITLE: &str = "Error";

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

    // pub fn print(&mut self) {}

    // TODO: default Stdout + Table
    pub fn print_run(&mut self, res: execute::Response) -> Result<(), Box<dyn std::error::Error>> {
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

    fn print_body(&mut self, body: String) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.writer, "{}", body)?;

        Ok(())
    }
}
