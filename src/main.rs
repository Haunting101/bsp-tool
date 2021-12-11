use std::fs::File;

use clap::{App, Arg, SubCommand};
use spooky_bsp::BspDecoder;

fn main() {
    let matches = App::new("bsp-tool")
        .author("Bakamono")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("info").about("Provides information about a BSP file's content"),
        )
        .subcommand(
            SubCommand::with_name("convert")
                .about("Converts a BSP file into another format and vice versa"),
        )
        .subcommand(SubCommand::with_name("extract").about("Extracts a BSP file"))
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verifies if a BSP file isn't corrupted")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        let input = matches.value_of("input").unwrap();

        match File::open(input) {
            Ok(file) => match BspDecoder::decode(file) {
                Ok(_) => println!("File is not corrupted"),
                Err(_) => println!("File is corrupted"),
            },
            Err(_) => println!("Failed to open a file"),
        }
    }
}
