use std::{
    fs::{self, File},
    io::ErrorKind,
    path::Path,
};

use clap::{App, Arg, SubCommand};
use image::{ImageBuffer, Rgba};
use spooky_bsp::{Bsp, Decode};

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
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extracts a BSP file")
                .arg(Arg::with_name("input").required(true).takes_value(true))
                .arg(Arg::with_name("output").required(true).takes_value(true))
                .arg(
                    Arg::with_name("texture-format")
                        .long("texture-format")
                        .takes_value(true)
                        .default_value("png")
                        .possible_values(&["png", "jpg", "tiff", "bmp"]),
                ),
        )
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verifies if a BSP file isn't corrupted")
                .arg(Arg::with_name("input").required(true).takes_value(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        let input = matches.value_of("input").unwrap();

        match File::open(input) {
            Ok(mut file) => match Bsp::decode(&mut file) {
                Ok(_) => println!("File is not corrupted"),
                Err(_) => println!("File is corrupted"),
            },
            Err(_) => println!("Failed to open a file"),
        }
    } else if let Some(matches) = matches.subcommand_matches("extract") {
        let input = matches.value_of("input").unwrap();
        let output = matches.value_of("output").unwrap();
        let texture_format = matches.value_of("texture-format").unwrap();

        if let Ok(mut file) = File::open(input) {
            if let Ok(bsp) = Bsp::decode(&mut file) {
                match fs::create_dir(output) {
                    Ok(_) => (),
                    Err(err) => match err.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => {
                            println!("Failed to create a directory {}", output);
                            return;
                        }
                    },
                }

                let textures_directory = Path::new(output).join("textures");

                match fs::create_dir(&textures_directory) {
                    Ok(_) => (),
                    Err(err) => match err.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => {
                            println!(
                                "Failed to create a directory {}",
                                textures_directory.display()
                            );
                            return;
                        }
                    },
                }

                for texture in bsp.textures {
                    let image_buffer = ImageBuffer::from_fn(
                        texture.get_width() as u32,
                        texture.get_height() as u32,
                        |x, y| {
                            let pixel = &texture.get_pixels()
                                [(y * (texture.get_width() as u32) + x) as usize];

                            Rgba([pixel.r as u8, pixel.g as u8, pixel.b as u8, pixel.a as u8])
                        },
                    );

                    if let Err(_) = image_buffer.save(textures_directory.join(format!(
                        "{}.{}",
                        texture.get_name(),
                        texture_format
                    ))) {
                        println!("Failed to extract a texture {}", texture.get_name());
                    }
                }
            } else {
                println!("Failed to decode a file");
            }
        } else {
            println!("Failed to open a file");
        }
    }
}
