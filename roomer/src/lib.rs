// use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Wall {
    p1: u32,
    p2: u32,
}
impl Wall {
    fn new() -> Wall {
        Wall { p1: 0, p2: 0 }
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
            wall: Wall { p1: 0, p2: 0 },
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
    // let walls: Vec<Wall> = vec![];
    let cols: Vec<_> = line.match_indices("+").map(|(i, _match)| i).collect();

    let walls: Vec<_> = line
        .split("+")
        .zip(cols)
        .filter(|(in_beween, _col)| in_beween.chars().all(|c| c == '-'))
        .map(|(bricks, col)| Wall {
            p1: col as u32,
            p2: (col + bricks.len()) as u32,
        })
        .collect();

    // let walls: Vec<_> = vec![Wall::new()];
    walls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_one_start_h_wall() {
        let content = "+--+-+";
        let walls = find_horizontal_walls(content);

        assert_eq!(1, walls.len());
        assert_eq!(Wall { p1: 0, p2: 2 }, walls[0]);
    }

    // #[test]
    // fn find_two_start_h_wall(){
    //     let content = "+--+--";
    //     let walls = find_horizontal_walls(content);
    //
    //     assert_eq!(2, walls.len());
    //      assert_eq!(0, walls[0].p1 );
    //     assert_eq!(8, walls[0].p2 );
    //    assert_eq!(0, walls[1].p1 );
    //     assert_eq!(8, walls[1].p2 );
    // }
}
