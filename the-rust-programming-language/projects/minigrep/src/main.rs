// run with:
// cargo run -- search_word input_file
// cargo run -- nobody poem.txt
// IGNORE_CASE=1 cargo run -- To poem.txt

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Bad arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error {e}");
        process::exit(1);
    }
}
