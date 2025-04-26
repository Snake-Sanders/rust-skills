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
