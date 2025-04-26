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
