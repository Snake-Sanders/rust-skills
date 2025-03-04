## Option Some and None

```rust
// Creating Options
let some_value: Option<i32> = Some(5);
let no_value: Option<i32> = None;

// Basic matching
match some_value {
    Some(n) => println!("Got value: {}", n),
    None => println!("No value"),
}
```

## Common Methods

```rust
let x = Some(10);
let none: Option<i32> = None;

// is_some() and is_none()
assert!(x.is_some());
assert!(none.is_none());

// unwrap() - gets value or panics
let value = x.unwrap(); // Returns 10
// none.unwrap(); // Would panic!

// unwrap_or() - gets value or default
let value = none.unwrap_or(0); // Returns 0

// unwrap_or_else() - with closure
let value = none.unwrap_or_else(|| {
    // Compute default value
    42
});
```


## Safe Value Extraction

```rust
let opt = Some(5);

// if let - when you only care about Some
if let Some(value) = opt {
    println!("Got value: {}", value);
}

// map - transform the contained value
let doubled = opt.map(|x| x * 2); // Some(10)

// and_then - chain optional operations
let result = opt
    .map(|x| x * 2)
    .and_then(|x| if x < 20 { Some(x) } else { None });

// filter - keep Some if condition is true
let filtered = opt.filter(|x| x % 2 == 0); // None (5 is not even)
```


## Combining Options

```rust
let first = Some(2);
let second = Some(4);

// zip - combine two Options
let pairs = first.zip(second); // Some((2, 4))

// and - returns None if either is None
let both = first.and(second); // Some(4)

// or - returns first Some or last None
let either = none.or(Some(10)); // Some(10)
```

## Working with References

```rust
let value = Some(String::from("hello"));

// as_ref() - borrow the contained value
let borrowed = value.as_ref().map(|s| s.len());

// as_mut() - mutable borrow
let mut value = Some(String::from("hello"));
if let Some(s) = value.as_mut() {
    s.push_str(" world");
}
```

## Pattern Matching with Guards

```rust
let number = Some(4);

match number {
    Some(x) if x < 0 => println!("Negative"),
    Some(x) if x > 0 => println!("Positive"),
    Some(0) => println!("Zero"),
    None => println!("No value"),
}
```


## Collecting and Converting

```rust
let numbers = vec![Some(1), None, Some(3)];

// Collect all Some values
let collected: Vec<i32> = numbers
    .into_iter()
    .filter_map(|x| x)
    .collect();

// Convert Option<Option<T>> to Option<T>
let nested = Some(Some(5));
let flattened = nested.flatten(); // Some(5)
```

## Error Handling Patterns

```rust 
fn divide(numerator: i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Using ?. operator with Option
fn compute_average(nums: Vec<i32>) -> Option<f64> {
    let sum = nums.iter().sum::<i32>();
    let count = nums.len();
    if count == 0 {
        None
    } else {
        Some(sum as f64 / count as f64)
    }
}
```

## Practical Examples

```rust 
struct User {
    id: i32,
    name: String,
    email: Option<String>,
}

impl User {
    // Working with optional fields
    fn get_email_domain(&self) -> Option<&str> {
        self.email
            .as_ref()
            .and_then(|email| email.split('@').nth(1))
    }

    // Chaining operations
    fn formatted_email(&self) -> Option<String> {
        self.email
            .as_ref()
            .map(|email| format!("{} <{}>", self.name, email))
    }
}
```
