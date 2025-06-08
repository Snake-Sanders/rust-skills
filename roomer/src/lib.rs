use std::fs;
// use regex::Regex;

struct Wall{
    p1: u32,
    p2: u32
}

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

fn parse_line(line: &str) -> Vec<Wall> {
    let mut points: Vec<Wall> = vec!();
    // let re = Regex::new(r"\+-+\+").unwrap();

    let v: Vec<_> = line.match_indices("+").map(|(i, _match)| i).collect();
    let z: Vec<_> = line.split("+").zip(v)
    .filter(|( rest, col)| {

    let mut continue_wall = true;
    for c in rest.chars(){
        continue_wall |= c == '-' 
    }
    continue_wall
     })
    .collect();

    dbg!(z);
    dbg!(line);
    // dbg!(v);


    points.push(Wall{p1: 8, p2: 0 });

    points
}

#[cfg(test)]
mod tests {
     use super::*;
    
    #[test]
    fn find_one_start_h_wall(){
        let content = "+--------";
        let points = parse_line(content);

        assert_eq!(1, points.len());
        assert_eq!(0, points[0].p1 );
        assert_eq!(8, points[0].p2 );
    }

    #[test]
    fn find_two_start_h_wall(){
        let content = "+--+--";
        let points = parse_line(content);

        assert_eq!(2, points.len());
        assert_eq!(0, points[0].p1 );
        assert_eq!(8, points[0].p2 );
    }

}
