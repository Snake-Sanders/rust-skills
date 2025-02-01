# Match


## Basic Pattern Guards**

```rust

let number = 5;

match number {
    n if n < 0 => println!("Negative"),
    n if n > 0 => println!("Positive"),
    _ => println!("Zero"),
}
```
## multiple conditions

```rust
let pair = (2, -2);

match pair {
    (x, y) if x > 0 && y > 0 => println!("Both positive"),
    (x, y) if x < 0 && y < 0 => println!("Both negative"),
    (x, y) if x * y < 0 => println!("Opposite signs"),
    _ => println!("At least one is zero"),
}
```

## Witch Enums and Structs

```rust
enum Message {
    Hello { id: i32 },
    Move { x: i32, y: i32 },
}

let msg = Message::Move { x: 3, y: 4 };

match msg {
    Message::Hello { id } if id > 0 => println!("Got positive hello: {}", id),
    Message::Move { x, y } if x.abs() + y.abs() < 10 => println!("Short move"),
    Message::Move { x, y } if x == y => println!("Diagonal move"),
    _ => println!("Something else"),
}
```

## With References and Options**

```rust
let opt = Some(4);

match &opt {
    Some(n) if *n > 5 => println!("Large number"),
    Some(n) if *n < 0 => println!("Negative number"),
    Some(_) => println!("Other number"),
    None => println!("No number"),
}
```


## Complex Conditions

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 0, y: 5 };

match point {
    Point { x, y } if x == 0 && y.abs() < 10 => println!("On y-axis, close to center"),
    Point { x, y } if y == 0 && x.abs() < 10 => println!("On x-axis, close to center"),
    Point { x, y } if x*x + y*y < 100 => println!("Close to origin"),
    _ => println!("Somewhere else"),
}
```

## With Multiple Patterns

```rust
let c = 'A';

match c {
    x if (x >= 'A' && x <= 'Z') || (x >= 'a' && x <= 'z') => println!("Letter"),
    x if x.is_digit(10) => println!("Digit"),
    _ => println!("Something else"),
}
```

## With Custom Functions

```rust
fn is_valid_id(id: i32) -> bool {
    id > 0 && id < 1000
}

let user_id = 500;

match user_id {
    id if is_valid_id(id) => println!("Valid ID: {}", id),
    id if id <= 0 => println!("Invalid negative ID"),
    _ => println!("ID too large"),
}
```

## With Enums

```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

let temp = Temperature::Celsius(25.0);

match temp {
    Temperature::Celsius(c) if c > 30.0 => println!("Hot day: {}°C", c),
    Temperature::Celsius(c) if c < 0.0 => println!("Freezing: {}°C", c),
    Temperature::Fahrenheit(f) if f > 86.0 => println!("Hot day: {}°F", f),
    _ => println!("Moderate temperature"),
}
```
