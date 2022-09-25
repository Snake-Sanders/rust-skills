fn count_20() {
    let mut timer = 0;

    let result = loop {
        timer += 1;
        if timer == 10 {
            break timer * 2;
        }
    };

    println!("The result is {result}");
}

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let counter = 6;

    if counter % 4 == 0 {
        println!("counter is divisible by 4");
    } else if counter % 3 == 0 {
        println!("counter is divisible by 3");
    } else if counter % 2 == 0 {
        println!("counter is divisible by 2");
    } else {
        println!("counter is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let id = if condition { 5 } else { 6 };
    println!("The value of id is: {id}");

    count_20();
}
