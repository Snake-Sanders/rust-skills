fn bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => format!("{} bottle", n),
        n => format!("{} bottles", n),
    }
}

fn take(n: u32) -> String {
    if n == 1 {
        "it".to_string()
    } else {
        "one".to_string()
    }
}

pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
         Go to the store and buy some more, 99 bottles of beer on the wall."
            .to_string()
    } else {
        let s1: String = format!("{} of beer on the wall, ", bottles(n));
        let s2: String = format!("{} of beer.\n", bottles(n));
        let s3: String = format!("Take {} down and pass it around, ", take(n));
        let s4: String = format!("{} of beer on the wall.", bottles(n - 1));

        [s1, s2, s3, s4].join("")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n\n")
}
fn main() {
    let verse: String = verse(1);
    println!("verse: {}", verse);
    println!("sing: {}", sing(99, 98));
}
