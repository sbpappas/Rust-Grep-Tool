use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for {}", config.query); //the print! could be sketch
    //io::stdout().flush().unwrap();
    println!(" in file {}: ", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

