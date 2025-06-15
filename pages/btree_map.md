## BTree Map

- Example

Receives a map with points as key, and list of letters as values.

1 point: "A", "E", "I", "O", "U", "L", "N", "R", "S", "T",
2 points: "D", "G",
3 points: "B", "C", "M", "P",

It sorts them, where the letter is they key and the value is the point.

Note there is no need to create a BTreeMap to return, the collect will create
it.

The `move` keyword here is necessary because of how closures work in Rust.

Closure Ownership: When we create a closure (the |&letter| { ... } part), it
needs to capture variables from its surrounding scope. In this case, it needs
to capture point from the outer closure.

- Why move is needed:
The inner closure needs to use point to create the tuple
(letter.to_ascii_lowercase(), *point)
The closure will be used in an iterator chain that might outlive the current
scope
Without move, the closure would try to borrow point, but the borrow might not
live long enough

- What move does:
It forces the closure to take ownership of the captured variables (point in
this case)
This means the closure owns its own copy of point rather than borrowing it
This is necessary because the iterator chain might be stored or used later,
and we need to ensure the data lives long enough

- Why it's safe here:
point is a reference to an i32 (&i32)
When we use move, we're moving the reference, not the actual value
This is fine because the reference is valid for the entire lifetime of the
iterator chain.

```rust
use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(point, letters)| {
            letters.iter().map(move |&letter| {
                (letter.to_ascii_lowercase(), *point)
            })
        })
        .collect()
}
```
