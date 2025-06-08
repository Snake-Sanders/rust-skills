use std::fs;

pub fn run(file_path: String) -> (){

    let content = fs::read_to_string(file_path).unwrap();

    for line in content.lines() {
        println!("{}", line)
    }
}

