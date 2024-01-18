use clap::{arg, value_parser, Command};
use std::path::PathBuf;

mod cpu;

fn main() {
    let r_matches = cli().get_matches();

    match r_matches.subcommand() {
        Some(("cpu", matches)) => {
            if let Some(file) = matches.get_one::<PathBuf>("file") {
                println!("{:?}", file);
            } else {
                println!("no filepath found");
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}

fn cli() -> Command {
    Command::new("pa")
        .about("Performance aware homework")
        .subcommand_required(true)
        .subcommand(
            Command::new("cpu").about("CPU simulation").arg(
                arg!([file] "ASM file")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ),
        )
}
