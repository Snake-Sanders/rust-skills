
## String manipulation

- [String manipulation](#string-manipulation)
  - [summary](#summary)
  - [String Concatenation](#string-concatenation)
  - [String indexing](#string-indexing)
  - [String slice](#string-slice)
  - [Finding \& Replacing](#finding--replacing)

### summary 

| Operation                 | Example                                       |
|---------------------------|-----------------------------------------------|
| Create String             | String::from("Rust"), "text".to_string()      |
| Concatenate               | s1 + &s2, format!("{}, {}", s1, s2)           |
| Append                    | s.push_str("text"), s.push('!')               |
| Iterate                   | s.chars(), s.bytes()                          |
| Find                      | s.contains("Rust"), s.find("Rust")            |
| Replace                   | s.replace("old", "new")                       |
| Split                     | s.split(" "), s.split_whitespace()            |
| Trim                      | s.trim()                                      |
| Convert Number <-> String | num.to_string(), "42".parse::<i32>().unwrap() |
| Change Case               | s.to_uppercase(), s.to                        |

`str` is stored in `rodata`, `String` is stored in the heap.

```rust
fn main() {
    let s1 = String::new(); // Empty string
    let s2 = String::from("Hello"); // Create from string literal
    let s3 = "World".to_string(); // Convert &str to String

    println!("{}, {}, {}", s1, s2, s3);
}
```

### String Concatenation

`s1 + &s2` works because String implements Add<str>.

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    
    let s3 = s1 + &s2; // `s1` is moved, so it can't be used again
    println!("{}", s3);
}
```

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    
    let combined = format!("{}, {}!", s1, s2); // s1 and s2 are not moved
    println!("{}", combined);
}
```

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world"); // Append string slice
    s.push('!'); // Append a single character, notice the quotes ''
    
    println!("{}", s);
}
```

### String indexing

Rust does not allow direct indexing (s[0]) because Rust strings are UTF-8 encoded, and characters can be more than 1 byte.

```rust
// let c = s[0]; // ERROR: No direct indexing
```

Iterate Over Characters

```rust
fn main() {
    let s = String::from("Hello");
    
    for c in s.chars() {
        println!("{}", c);
    }
}
```


Iterate Over bytes

```rust
fn main() {
    let s = String::from("Hello");
    
    for b in s.bytes() {
        println!("{}", b); // Prints UTF-8 byte values
    }
}
```

### String slice

Use slicing (&s[start..end]) instead of substring functions.
Rust ensures UTF-8 correctness (invalid ranges will panic).

```rust
fn main() {
    let s = String::from("Hello, Rust!");
    
    let hello = &s[0..5]; // Slices "Hello"
    let rust = &s[7..11]; // Slices "Rust"
    
    println!("{}, {}", hello, rust);
}
```

### Finding & Replacing


```rust
fn main() {
    let s = String::from("Rust is awesome");
    
    if s.contains("Rust") {
        println!("Found Rust!");
    }
}
```
