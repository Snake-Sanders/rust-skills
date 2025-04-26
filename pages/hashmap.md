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
