use log::{debug, info};
use std::env;
use std::fs;

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let input_file = &args[2];

    info!("Day {}, Files {}", day, input_file);

    let contents = fs::read_to_string(input_file)
        .unwrap_or_else(|_| panic!("Unable to open input file: {}", input_file));

    debug!("Input contents:\n{}", contents);
}
