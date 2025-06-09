use regex::Regex;
use std::fs;
use std::ops::Range;

#[derive(Debug, PartialEq)]
struct Wall {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
struct Area {
    width: Range<usize>,
    content: String,
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
    let _areas = find_open_areas(line);
    let room = Room::new();
    dbg!(&room);
    rooms.push(room);

    rooms
}

fn find_horizontal_walls(line: &str) -> Vec<Wall> {
    let re = Regex::new(r"\.?\+-+\+").unwrap();

    let walls: Vec<_> = re
        .find_iter(line)
        .map(|m| m.range())
        .map(|range| Wall {
            start: range.start, 
            end: ((range.end as u32) - 1) as usize,
        })
        .collect();
    walls
}

fn find_open_areas(line: &str) -> Vec<Area> {
    // let re = Regex::new(r"(/|\||\\|\+)[^-].+(/|\||\\|\+)").unwrap();
    let re = Regex::new(r"(/|\||\\|\+)[^-/\|\\\+]+").unwrap();

    let areas: Vec<_> = re
        .find_iter(line)
        .map(|m| {
            let width = m.range();
            let content = m.as_str().to_string();

            Area {
                width: width,
                content: content,
            }
        })
        .collect();

    areas
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_empty_multi_open_areas() {
        let line = "  |   | |";
        let areas = find_open_areas(line);
        assert_eq!(2, areas.len());
        assert_eq!(areas[0], Area{content: "|   ".to_string(), width: 2..6});
        assert_eq!(areas[1], Area{content: "| ".to_string(), width: 6..8});

        let line = " +--+  /  \\  +-- ";
        let areas = find_open_areas(line);
        assert_eq!(3, areas.len());
        assert_eq!(areas[0], Area{content: "+  ".to_string(), width: 4..7});
        assert_eq!(areas[1], Area{content: "/  ".to_string(), width: 7..10});
        assert_eq!(areas[2], Area{content: "\\  ".to_string(), width: 10..13});
    }

    #[test]
    // #[ignore = "alrady tested"]
    fn find_empty_open_areas() {
        let line = "|   |";
        let areas = find_open_areas(line);
        assert_eq!(1, areas.len());
        assert_eq!(
            areas[0],
            Area {
                content: "|   ".to_string(),
                width: 0..4
            }
        );

        let line = "+  \\";
        let areas = find_open_areas(line);
        assert_eq!(1, areas.len());
        assert_eq!(
            areas[0],
            Area {
                content: "+  ".to_string(),
                width: 0..3
            }
        );

        let line = " /  +--";
        let areas = find_open_areas(line);
        assert_eq!(1, areas.len());
        assert_eq!(
            areas[0],
            Area {
                content: "/  ".to_string(),
                width: 1..4
            }
        );
    }
    #[test]
    //#[ignore = "alrady tested"]
    fn find_invalid_start_h_wall() {
        let line = " -+";
        let walls = find_horizontal_walls(line);
        assert_eq!(0, walls.len());

        let line = " ++ ";
        let walls = find_horizontal_walls(line);
        assert_eq!(0, walls.len());
    }
    #[test]
    fn find_two_valid_start_h_wall() {
        let line = " +-+ +---+ ";
        let walls = find_horizontal_walls(line);
        assert_eq!(2, walls.len());
        assert_eq!(Wall { start: 1, end: 3 }, walls[0]);
        assert_eq!(Wall { start: 5, end: 9 }, walls[1]);
    }
    #[test]
    fn find_one_valid_start_h_wall() {
        let line = "+--+";
        let walls = find_horizontal_walls(line);
        assert_eq!(1, walls.len());
        assert_eq!(Wall { start: 0, end: 3 }, walls[0]);
    }
}
