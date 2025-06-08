use std::env;
use std::process;

use roomer;

fn main() {
    let params: Vec<String> = env::args().collect();

    if params.len() != 2 {
        println!("Invalid paramas. Passed {}, expected 1", params.len() - 1);
        process::exit(1);
    }

    roomer::run(params[1].clone());
}
