use clap::{App, SubCommand};

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
        .subcommand(SubCommand::with_name("verify").about("Verifies if a BSP file isn't corrupted"))
        .get_matches();
}
