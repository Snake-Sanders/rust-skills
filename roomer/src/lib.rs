use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Wall {
    start: u32,
    end: u32,
}
impl Wall {
    fn new() -> Wall {
        Wall { start: 0, end: 0 }
    }
}

#[derive(Debug)]
struct Room {
    wall: Wall,
    name: String,
    // objects: Vec<(String, u32)>,
}

impl Room {
    fn new() -> Room {
        Room {
            name: "".to_string(),
            wall: Wall { start: 0, end: 0 },
        }
    }
}

pub fn run(file_path: String) -> () {
    let content = read_file(file_path);
    display_content(&content);
    for line in content {
        let rooms = parse_line(&line);

        dbg!(rooms);
    }
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

fn parse_line(line: &str) -> Vec<Room> {
    let mut rooms: Vec<Room> = vec![];
    let _walls = find_horizontal_walls(line);
    let room = Room::new();

    rooms.push(room);

    rooms
}

// let re = Regex::new(r"\+-+\+").unwrap();
fn find_horizontal_walls(line: &str) -> Vec<Wall> {
    let re = Regex::new(r"\.?\+-+\+").unwrap();

    let walls: Vec<_> = re
        .find_iter(line)
        .map(|m| m.range())
        .map(|range| Wall {
            start: range.start as u32,
            end: (range.end as u32) - 1,
        })
        .collect();
    walls
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_invalid_start_h_wall() {
        let content = " -+";
        let walls = find_horizontal_walls(content);
        assert_eq!(0, walls.len());

        let content = " ++ ";
        let walls = find_horizontal_walls(content);
        assert_eq!(0, walls.len());
    }
    #[test]
    fn find_two_valid_start_h_wall() {
        let content = " +-+ +---+ ";
        let walls = find_horizontal_walls(content);
        assert_eq!(2, walls.len());
        assert_eq!(Wall { start: 1, end: 3 }, walls[0]);
        assert_eq!(Wall { start: 5, end: 9 }, walls[1]);
    }
    #[test]
    fn find_one_valid_start_h_wall() {
        let content = "+--+";
        let walls = find_horizontal_walls(content);
        assert_eq!(1, walls.len());
        assert_eq!(Wall { start: 0, end: 3 }, walls[0]);
    }
}
