# HashMap

## Initialization

It can be initialized with a vector of tuples key-values.

```rust
let counts: HashMap<char, usize> =
    HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
```

Adding values to a map

```rust
    let mut logger = HashMap::new();
    logger.insert("not-found", 3);
    logger.insert("duplicated", 7);
    logger.insert("invalid", 1);
```

Get the map keys

```rust
use std::collections::HashMap;

let map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

for key in map.keys() {
    print!("{key} ");
}
// the result is unsorted!
// b a c
```

## Iterating over a Map (Copy)

This will create a copy with the changes without modifying the original map.

```rust
let new_map: HashMap<_, _> = map.into_iter()
    .map(|(k, v)| (k, v * 2))
    .collect();
```

## Iterating over a Map (mut)

This will apply the changes directly on the original map.

- `.iter_mut()` gives you mutable references to the values.
- `.for_each()` is like Elixir’s .map, but for side effects (not for returning
a new collection).

```rust
map.iter_mut().for_each(|(_k, v)| *v *= 2);
```

If you call a function f1 for each value, how should you pass it?

1. If fun_1 only needs to read the value, pass it as &V (reference).
2. If fun_1 needs to mutate the value, pass it as &mut V (mutable reference).
3. If you want to replace the value with the function’s return, you need to
assign the result to *v

Example: Mutating in place

The function updates the value, then the value is passed as
mutable reference.

```rust
fn double_in_place(x: &mut usize) {
    *x *= 2;
}

map.iter_mut().for_each(|(_k, v)| double_in_place(v));
```

Example: Replacing with function return

```rust
fn half(x: usize) -> usize {
    x / 2
}

map.iter_mut().for_each(|(_k, v)| *v = half(*v));
```

## ok_or(E)?

.ok_or(c) - This is where ok_or() comes into play. It converts an Option<T>
into a Result<T, E>. Specifically:

- If the Option is Some, it becomes Ok(())
- If the Option is None, it becomes Err(c) where c is the invalid nucleotide

The ? operator then:

- If the result is Ok, it continues execution
- If the result is Err, it immediately returns from the function with that error

```rust
fn get_age(name: &str) -> Result<i32, &str> {
    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
    ages.insert("Bob", 25);
    
    // Try to get the age, if not found return the name as error
    ages.get(name)
        .copied()  // Convert &i32 to i32
        .ok_or(name)  // Convert Option to Result
}

fn main() {
    // This will work
    match get_age("Alice") {
        Ok(age) => println!("Alice is {} years old", age),
        Err(name) => println!("{} not found", name),
    }
    
    // This will fail
    match get_age("Charlie") {
        Ok(age) => println!("Charlie is {} years old", age),
        Err(name) => println!("{} not found", name),
    }
}
```

When we call get_age("Alice"):

- ages.get("Alice") returns Some(&30)
- .copied() converts it to Some(30)
- .ok_or("Alice") converts it to Ok(30)

When we call get_age("Charlie"):

- ages.get("Charlie") returns None
- .copied() keeps it as None
- .ok_or("Charlie") converts it to Err("Charlie")

The `?` operator is used when you want to propagate errors up the call stack.
Here's an example using it:

```rust
fn get_age_twice(name: &str) -> Result<i32, &str> {
    let age1 = get_age(name)?;  // If error, return immediately
    let age2 = get_age(name)?;  // If error, return immediately
    Ok(age1 + age2)
}

fn main() {
    match get_age_twice("Alice") {
        Ok(total) => println!("Alice's age doubled is {}", total),
        Err(name) => println!("{} not found", name),
    }
    
    match get_age_twice("Charlie") {
        Ok(total) => println!("Charlie's age doubled is {}", total),
        Err(name) => println!("{} not found", name),
    }
}
```

In this second example:

- If we call get_age_twice("Alice"), it will return Ok(60) (30 + 30)
- If we call get_age_twice("Charlie"), it will return Err("Charlie")
immediately after the first get_age call fails

The ? operator is like a shortcut for:

```rust
let age1 = match get_age(name) {
    Ok(age) => age,
    Err(e) => return Err(e),
};
```
