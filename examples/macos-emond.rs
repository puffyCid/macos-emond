use std::{env, error::Error, fs::OpenOptions, io::Write, path::Path};

use log::LevelFilter;
use macos_emond::emond::EmondData;
use simplelog::{Config, SimpleLogger};

fn main() {
    println!("Parsing macOS Emond Rules...");
    SimpleLogger::init(LevelFilter::Warn, Config::default())
        .expect("Failed to initialize simple logger");

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let path = &args[1];
        if Path::new(path).is_file() {
            let emond_results = macos_emond::parser::parse_emond_file(path).unwrap();
            output_emond(&emond_results);
        } else {
            let emond_results = macos_emond::parser::parse_emond_rules(path).unwrap();
            output_emond(&emond_results);
        }
    } else {
        let emond_paths = macos_emond::parser::get_emond_rules_paths().unwrap();
        for paths in emond_paths {
            let emond_results = macos_emond::parser::parse_emond_rules(&paths).unwrap();
            output_emond(&emond_results);
        }
    }
}

fn output_emond(results: &[EmondData]) {
    for data in results {
        let output_results = output(data, &data.name);
        match output_results {
            Ok(_) => {}
            Err(error) => println!("Failed to output data: {:?}", error),
        }
    }
}

fn output(results: &EmondData, output_name: &str) -> Result<(), Box<dyn Error>> {
    let mut json_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}.json", output_name))?;

    let serde_data = serde_json::to_string(&results)?;

    json_file.write_all(serde_data.as_bytes())?;

    Ok(())
}
