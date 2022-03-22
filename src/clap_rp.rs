use crate::handler::Handler;
use crate::input::{RunInput, FmtInput};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Rp {
    #[clap(subcommand)]
    command: RpSubCommand,
}

impl Rp {
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut handler = Handler::new();

        match &self.command {
            RpSubCommand::Run(args) => {
                let input = args.input();
                handler.run(input)?;
                Ok(())
            },
            RpSubCommand::Fmt(args) => {
                let input = args.input();
                handler.fmt(input)?;
                Ok(())
            },
            // error handling...
        }
    }
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum RpSubCommand {
    #[clap(name = "run")]
    Run(RunArgs),

    #[clap(name = "fmt")]
    Fmt(FmtArgs),
}

#[derive(Args, PartialEq, Debug)]
pub struct RunArgs {
    // TODO: support other run_type (build, test, wasm,,,)
    // https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/04_01_enum.rs
    // run_type: ENUM
    #[clap(long = "mode", default_value = "debug", required = false)]
    mode: String,

    #[clap(long = "channel", default_value = "stable", required = false)]
    channel: String,

    #[clap(long = "edition", default_value = "2021", required = false)]
    edition: String,

    #[clap(long = "backtrace", parse(try_from_str = true_or_false), default_value = "false", required = false)]
    backtrace: bool,

    #[clap(required = true, parse(from_os_str))]
    file_path: PathBuf,
}

impl RunArgs {
    fn input(&self) -> RunInput {
        RunInput {
            mode: self.mode.clone(),
            channel: self.channel.clone(),
            edition: self.edition.clone(),
            backtrace: self.backtrace,
            file_path: self.file_path.clone(),
        }
    }
}

#[derive(Args, PartialEq, Debug)]
pub struct FmtArgs {

    #[clap(long = "edition", default_value = "2021", required = false)]
    edition: String,

    #[clap(required = true, parse(from_os_str))]
    file_path: PathBuf,
}

impl FmtArgs {
    fn input(&self) -> FmtInput {
        FmtInput {
            edition: self.edition.clone(),
            file_path: self.file_path.clone(),
        }
    }
}

fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}
