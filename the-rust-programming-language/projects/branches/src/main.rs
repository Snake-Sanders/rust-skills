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

fn loop_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Lift off!")
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

    loop_labels();

    while_loops();
}
