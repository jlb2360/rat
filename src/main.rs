
use std::env;
use rat::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = rat::run(config) {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }
}
