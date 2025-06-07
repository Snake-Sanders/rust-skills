use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Bad arguments: {e}");
        process::exit(1);
    });

    run(config);
}

fn run(config: Config) {
    let content = fs::read_to_string(config.file_path).expect("Ops, file  not found");
    println!("With text: \n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
