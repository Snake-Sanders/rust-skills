fn main() {
    println!("Hello, world!");
}

fn number(n: u32) -> String {
    match n {
        0 => "no",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "",
    }
    .to_string()
}
fn bottles(n: u32) -> String {
    if n == 1 { "bottle" } else { "bottles" }.to_string()
}

pub fn verse(start_bottles: u32) -> String {
    let take_down: u32 = 1;
    let start = number(start_bottles);
    let take = number(take_down).to_lowercase();
    let left = number(start_bottles - take_down).to_lowercase();

    let bottles_start = bottles(start_bottles);
    let bottles_taken = bottles(take_down);
    let bottles_left = bottles(start_bottles - take_down);

    let s1 = format!("{} green {} hanging on the wall,\n", start, bottles_start).to_string();
    let s2 = format!("{} green {} hanging on the wall,\n", start, bottles_start).to_string();

    let s3 = format!(
        "And if {} green {} should accidentally fall,\n",
        take, bottles_taken
    )
    .to_string();

    let s4 = format!(
        "There'll be {} green {} hanging on the wall.",
        left, bottles_left
    )
    .to_string();

    s1 + &s2 + &s3 + &s4
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let first = start_bottles - take_down;
    let last = start_bottles;
    //let mut sentences = String::new();

    (first..=last).rev().map(verse).collect()
    //let idx: u32 = start_bottles;
    //for idx in idx..take_down {
    //    let verse = verse(idx);
    //    sentences.push_str(&verse);
    //}
    ////verse(first)
    //sentences
}
