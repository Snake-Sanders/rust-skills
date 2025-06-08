use std::fs;

pub fn run(file_path: String) -> () {
    let content = read_file(file_path);
    display_content(&content);
}

fn read_file(file_path: String) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let content = fs::read_to_string(file_path).expect("could not read the input file");
    for line in content.lines() {
        res.push(line.to_string());
    }
    res
}
fn display_content(content: &Vec<String>) -> () {
    for line in content {
        println!("{}", line)
    }
}
