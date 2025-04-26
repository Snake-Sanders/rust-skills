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
