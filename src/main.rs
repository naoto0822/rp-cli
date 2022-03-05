use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust Playground for Client!!!");

    let cmd = RpCommand::parse();

    println!("{:?}", cmd);

    match cmd.action {
        Action::Run(args) => {
            println!("run!");

            println!("{:?}", args);

            let file_body = match readFile(args.file) {
                Ok(b) => b,
                Err(why) => panic!(why),
            };

            println!("{}", file_body);
            println!("{}", args.backtrace);

            let req = Request {
                crate_type: "bin".to_string(),
                tests: false,
                mode: args.mode,
                channel: args.channel,
                edition: args.edition,
                backtrace: args.backtrace,
                code: file_body,
            };

            let client = reqwest::blocking::Client::new();
            let res: Response = client
                .post("https://play.rust-lang.org/execute")
                .json(&req)
                .send()?
                .json()?;

            println!("{:?}", res);

            println!("Execution");
            println!("Standard Error");
            println!("{}", res.stderr);
            println!("Standard Output");
            println!("{}", res.stdout);
        }
        Action::Fmt(args) => {
            println!("fmt!");
        }
    };

    Ok(())
}

fn readFile(path: PathBuf) -> Result<String, String> {
    let body = fs::read_to_string(&path);
    match body {
        Ok(b) => Ok(b),
        Err(err) => Err("failed to read file".to_string()),
    }
}

#[derive(Debug, Parser)]
struct RpCommand {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, PartialEq, Subcommand)]
enum Action {
    #[clap(name = "run")]
    Run(CommandArgs),
    #[clap(name = "fmt")]
    Fmt(CommandArgs),
}

#[derive(Args, PartialEq, Debug)]
struct CommandArgs {
    #[clap(long = "mode", default_value = "debug", required = false)]
    mode: String,
    #[clap(long = "channel", default_value = "stable", required = false)]
    channel: String,
    #[clap(long = "edition", default_value = "2021", required = false)]
    edition: String,
    #[clap(long = "backtrace", parse(try_from_str = true_or_false), default_value = "false", required = false)]
    backtrace: bool,
    #[clap(required = true, parse(from_os_str))]
    file: PathBuf,
}

fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}

#[derive(Serialize)]
struct Request {
    #[serde(rename(serialize = "crateType"))]
    crate_type: String,
    tests: bool,
    mode: String,
    channel: String,
    edition: String,
    backtrace: bool,
    code: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    stderr: String,
    #[serde(default)]
    error: String,
}

fn default_error_value() -> String {
    "".to_string()
}

enum CrateType {
    // Bin run
    Bin,
}

impl CrateType {
    fn to_string(&self) -> String {
        match self {
            CrateType::Bin => "bin".to_string(),
        }
    }
}

enum Mode {
    // Debug
    Debug,
}

impl Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Debug => "debug".to_string(),
        }
    }
}

enum Channel {
    Stable,
    Beta,
    Nightly,
}

impl Channel {
    fn to_string(&self) -> String {
        match self {
            Channel::Stable => "stable".to_string(),
            Channel::Beta => "beta".to_string(),
            Channel::Nightly => "nightly".to_string(),
        }
    }
}

enum Edition {
    E2015,
    E2018,
    E2021,
}

impl Edition {
    fn to_string(&self) -> String {
        match self {
            Edition::E2015 => "2015".to_string(),
            Edition::E2018 => "2018".to_string(),
            Edition::E2021 => "2021".to_string(),
        }
    }
}
