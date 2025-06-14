## Iterators

The iter() method is part of Rust's iterator system. Let me break this down:

### iter()  

- Creates an iterator that borrows each element of the collection
- Returns references (&T) to the elements
- Collection remains owned by original scope
- Implemented via the IntoIterator trait

```rust
// Different ways to create iterators
let vec = vec![1, 2, 3];
vec.iter()        // Creates iterator of references: &T
vec.iter_mut()    // Creates iterator of mutable references: &mut T
vec.into_iter()   // Takes ownership of collection: T

// String and chars
let string = String::from("hello");
string.chars()    // Creates iterator over chars
string.bytes()    // Creates iterator over bytes
```

### Common Iterator patterns

```rust
let vec = vec![1, 2, 3, 4, 5];

// Reference iteration
for x in vec.iter() {
    println!("{}", x); // x is &i32
}

// Mutable reference iteration
for x in vec.iter_mut() {
    *x += 1;          // x is &mut i32
}

// Taking ownership
for x in vec.into_iter() {
    println!("{}", x); // x is i32
    // vec is consumed here
}
```

### Enumerate()

`enumerate()` is an iterator adaptor that wraps an iterator and yields pairs of
(index, value).

```rust
let string = String::from("hello");

// with index
for (i, c) in text.chars().enumerate() {
    println!("Position {}: {}", i, c);
}
```

```rust
let words = vec!["apple", "banana", "cherry"];

// Basic enumeration
for (index, word) in words.iter().enumerate() {
    println!("{}. {}", index, word);
}

// Collecting into a vector of tuples
let indexed: Vec<(usize, &str)> = words.iter()
    .enumerate()
    .collect();
```

### Ranges

```rust
//(1 + 2 + ... + 10)² = 55² = 3025.
pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

//1² + 2² + ... + 10² = 385.
pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|s| s.pow(2)).sum()
}

```

### Common Patterns

```rust
let chars = vec!['a', 'b', 'c'];

// Skip and take
let subset: Vec<(usize, &char)> = chars.iter()
    .enumerate()
    .skip(1)        // Skip first element
    .take(2)        // Take next two elements
    .collect();

// Filter
let filtered: Vec<(usize, &char)> = chars.iter()
    .enumerate()
    .filter(|&(i, _)| i % 2 == 0)  // Keep even indices
    .collect();

// Map
let modified: Vec<String> = chars.iter()
    .enumerate()
    .map(|(i, &c)| format!("{}:{}", i, c))
    .collect();
```

```rust
// Creating a numbered list with formatting
let tasks = vec!["Buy groceries", "Walk dog", "Read book"];

let formatted: Vec<String> = tasks.iter()
    .enumerate()
    .map(|(i, task)| format!("{}. {}", i + 1, task))
    .collect();

// Result:
// 1. Buy groceries
// 2. Walk dog
// 3. Read book

// With pattern matching
let results = vec![Ok(1), Err("error"), Ok(3)];
for (i, result) in results.iter().enumerate() {
    match result {
        Ok(val) => println!("Success at {}: {}", i, val),
        Err(e) => println!("Error at {}: {}", i, e),
    }
}
```

```rust
fn is_valid(dna: &str) -> bool {
    match dna {
        "" => true,
        dna => dna.find(['A', 'C', 'G', 'T']).is_some(),
    }
}
```

### Collect

This method behaves differently depending on the expected result variable type.

```rust
let input = "hello";

// Collecting into Vec<char>
let chars: Vec<char> = input.chars().collect();
assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);

// Collecting into String
let string: String = input.chars().collect();
assert_eq!(string, "hello");

// Collecting into Vec<String> (each char as a String)
let string_vec: Vec<String> = input.chars().map(|c| c.to_string()).collect();
assert_eq!(string_vec, vec!["h", "e", "l", "l", "o"]);
```

Example of series of digits

With `len=3` and `digigts=1234567` will return:
`["123", "234", "345", "456", "567"]`

```rust
  digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|window| window.iter().collect::<String>())
        .collect()
```
