# Vectors

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();  // Empty vector
    v.push(10);
    v.push(20);
    println!("{:?}", v);  // Output: [10, 20]
}
```

```rust
fn main() {
    let v = vec![1, 2, 3, 4];  // Macro for easy initialization
    println!("{:?}", v);  // Output: [1, 2, 3, 4]
}
```

```rust
fn main() {
    let v = vec![10, 20, 30];
    
    println!("Index 1: {}", v[1]);         // Output: 20
    println!("Get index 2: {:?}", v.get(2)); // Output: Some(30)
    println!("Get index 10: {:?}", v.get(10)); // Output: None
}
```

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.insert(1, 99);  // Insert 99 at index 1
    println!("{:?}", v);  // Output: [1, 99, 2, 3, 4]
}
```

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.pop();  // Removes 4
    v.remove(1);  // Removes the element at index 1
    println!("{:?}", v);  // Output: [1, 3]
}
```

```rust
fn main() {
    let v = vec![10, 20, 30];
    for x in &v {
        println!("{}", x);  // 10 20 30
    }
}
```
Notes: 

&v → Means "borrow a reference" to the vector.
v is a Vec<i32>.

- &v borrows the vector as &[i32] → creates an immutable slice.
- The iterator produced by &v is over references → &i32.
- for x in &v gives you x as &i32, not i32.
- Here, the vector is moved into the loop.
- You cannot use v after the loop because it has been consumed.

|Code	            |Behavior	    |Explanation|
|-------------------|---------------|-----------|
|for x in v	        |Moves vector	|Ownership transferred, vector can't be reused|
|for x in &v	    |Borrows vector	|Borrow as immutable reference|
|for x in &mut v	|Mutable borrow	|Allows modifying elements|

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x *= 2;  // Multiply each element by 2
    }
    println!("{:?}", v);  // Output: [2, 4, 6]
}
```

```rust
fn main() {
    let mut v = Vec::with_capacity(2);
    println!("Capacity: {}", v.capacity()); // Output: 2
    v.push(1);
    v.push(2);
    v.push(3);  // Triggers reallocation
    println!("Capacity after push: {}", v.capacity()); // Output: 4
}
```

```rust
fn main() {
    let v = vec![10, 20, 30, 40];
    
    let first = v.first();
    let last = v.last();

    println!("First: {:?}", first); // Output: First: Some(10)
    println!("Last: {:?}", last);   // Output: Last: Some(40)
}
```

```rust
fn main() {
    let v = vec![10, 20, 30, 40];

    let first = v.get(0);
    let last = v.get(v.len() - 1);

    println!("First: {:?}", first); // Output: First: Some(10). Some(&int)
    println!("Last: {:?}", last);   // Output: Last: Some(40). Some(&int)
    
    println!("Last: {:?}", v.last().copied()); // Some(int)
}
```

Note:

In Rust, the last() method on a vector (or slice) returns a reference to the
last element, not the element itself. This is why it returns Some(&u32) instead
of Some(u32).

This behavior is by design and it's more efficient since it avoids unnecessary
copying of data

The .copied() method (or .map(|&score| score)) transforms an Option<&u32> into 
an Option<u32> by copying the value if it exists.

```rust
fn main() {
    let v = vec![40, 20, 30, 10];

    let max = v.iter().max().copied()
    println!("Max: {:?}", max); // 40 Some(int)

```
Note:

Using `v.sort()` will sort the vector `v` in place and return `()`. 
`sort()` cannot be piped.

```rust
pub fn personal_top_three(&self) -> Vec<u32> {
    let mut scores = self.scores.clone();
    scores.sort();  // Sort in ascending order
    
    // Take last 3 elements in reverse order
    scores.into_iter()
        .rev()
        .take(3)
        .collect()
}
```
Another way:
```rust
pub fn top_three(&self) -> Vec<u32> {
    let mut scores = self.scores.clone();
    scores.sort_by(|a, b| b.cmp(a));  // Sort in descending order
    scores.truncate(3);  // Keep only top 3
    scores
}
```

