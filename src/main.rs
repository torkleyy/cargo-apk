extern crate ansi_term;
extern crate cargo_apk_lib;
extern crate clap;

use cargo_apk_lib::{BuildConfig, Config, execute_build};
use clap::{Arg, App, ArgMatches, SubCommand};

fn main() {
    let app = App::new("cargo-apk")
        .version("0.1")
        .author("Thomas Schaller (torkleyy)")
        .about("Rust project manager for targeting Android, based on tomaka's cargo-apk")
        .arg(Arg::with_name("verbose")
            .short("v")
            .help("Use verbose output"))
        .subcommand(SubCommand::with_name("build")
            .about("Builds the Rust project and links it to the APK")
            .arg(Arg::with_name("bin")
                .help("Build the specified binary")
                .value_name("NAME")
                .takes_value(true)))
            .arg(Arg::with_name("features")
                .help("Space-separated list of features to also build")
                .value_name("FEATURES")
                .multiple(true));

    let matches = app.get_matches();

    let config = Config {
        verbose: matches.is_present("verbose"),
    };

    if config.verbose {
        println!("Executing with config: {:#?}", config);
    }

    match matches.subcommand_name() {
        Some(name) => execute_sub_command(config, name, matches.subcommand_matches(name).unwrap()),
        None => {
            println!("Error: No subcommand specified. Use `--help` to display help.");
        }
    }
}

fn execute_sub_command(config: Config, command: &str, matches: &ArgMatches) {
    if config.verbose {
        println!("Executing subcommand: {}", command);
    }

    match command {
        "build" => {
            let build_config = BuildConfig {
                bin: matches.value_of("bin"),
                features: matches.values_of("features").unwrap().collect(),
            };

            if let Err(x) = execute_build(&config, &build_config) {
                use ansi_term::Color;

                println!("{} {}", Color::Red.paint("error:"), x);
            }
        },
        _ => {
            panic!("Unhandled subcommand; this shouldn't be possible");
        }
    }
}
