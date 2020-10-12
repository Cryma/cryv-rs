use clap::{App, Arg};
use std::io::prelude::*;

mod parser;

fn main() {
    let matches = App::new("Native Converter")
        .version("1.0.0")
        .author("Fabian Vowie <crymaprivat@gmail.com>")
        .about("Converts a natives.h file into rust modules")
        .subcommand(
            App::new("convert")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .takes_value(true)
                        .about("Input file")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .takes_value(true)
                        .about("Output directory")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("convert") {
        let file: String = matches.value_of_t_or_exit("file");
        let output_directory: String = matches.value_of_t_or_exit("output");

        do_convert(file, output_directory);
    }
}

fn do_convert(input_file: String, output_directory: String) {
    let text = match std::fs::read_to_string(input_file) {
        Ok(value) => value,
        Err(error) => {
            println!("Error while trying to read natives.h file: {}", error);

            return;
        }
    };

    let natives = match parser::parse_native_file(text) {
        Some(value) => value,
        None => {
            println!("Error while trying parse native file");

            return;
        }
    };

    let output_path = std::path::Path::new(&output_directory);
    if let Err(error) = std::fs::create_dir_all(output_path) {
        println!("Error while trying to create output directory: {}", error);

        return;
    }

    let all_namespaces = natives.data.keys().collect::<Vec<&String>>();

    let mut file = match std::fs::File::create(output_path.join("mod.rs")) {
        Ok(value) => value,
        Err(error) => {
            println!("Error while trying to create mod.rs file: {}", error);

            return;
        }
    };

    if let Err(error) = file.write_all(
        all_namespaces
            .into_iter()
            .map(|x| format!("pub mod {};\n", x.to_lowercase()))
            .collect::<String>()
            .as_bytes(),
    ) {
        println!("Error while trying to write to mod.rs file: {}", error);

        return;
    }

    for (namespace, natives) in &natives.data {
        let natives = natives
            .into_iter()
            .map(|x| format!("{}\n", x))
            .collect::<String>();

        let mut file = match std::fs::File::create(
            output_path.join(format!("{}.rs", namespace.to_lowercase())),
        ) {
            Ok(value) => value,
            Err(error) => {
                println!(
                    "Error while trying to create {}.rs file: {}",
                    namespace, error
                );

                return;
            }
        };

        if let Err(error) =
            file.write_all(format!("use crate::natives::NativeVector3;\n{}", natives).as_bytes())
        {
            println!(
                "Error while trying to write to {}.rs file: {}",
                namespace, error
            );
        }
    }
}
