use rust_study::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = rust_study::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
