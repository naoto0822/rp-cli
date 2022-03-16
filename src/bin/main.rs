use clap::Parser;
use rp::clap_rp::Rp;
use std::process;

fn main() {
    let app = Rp::parse();
    let result = app.start();

    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(why) => {
            // TODO: output error message
            process::exit(1);
        }
    }
    //
    //    let cmd = RpCommand::parse();
    //
    //    println!("{:?}", cmd);
    //
    //    match cmd.action {
    //        Action::Run(args) => {
    //            println!("run!");
    //
    //            println!("{:?}", args);
    //
    //            let file_body = match readFile(args.file) {
    //                Ok(b) => b,
    //                Err(why) => panic!(why),
    //            };
    //
    //            println!("{}", file_body);
    //            println!("{}", args.backtrace);
    //
    //            let req = Request {
    //                crate_type: "bin".to_string(),
    //                tests: false,
    //                mode: args.mode,
    //                channel: args.channel,
    //                edition: args.edition,
    //                backtrace: args.backtrace,
    //                code: file_body,
    //            };
    //
    //            let client = reqwest::blocking::Client::new();
    //            let res: Response = client
    //                .post("https://play.rust-lang.org/execute")
    //                .json(&req)
    //                .send()?
    //                .json()?;
    //
    //            println!("{:?}", res);
    //
    //            println!("Execution");
    //            println!("Standard Error");
    //            println!("{}", res.stderr);
    //            println!("Standard Output");
    //            println!("{}", res.stdout);
    //        }
    //        Action::Fmt(args) => {
    //            println!("fmt!");
    //        }
    //    };
    //
    //    Ok(())
}
