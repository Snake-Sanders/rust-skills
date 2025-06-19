# Rust Iterator Chain Debugging

In Rust, you can achieve similar debugging functionality to Elixir's `|> dbg()` using several approaches. Here are the most common methods:

## 1. Using `dbg!()` macro (Rust 1.32+)

The `dbg!()` macro is the most direct equivalent to Elixir's `dbg()`:

```rust
phrase
    .split(&['-', ' '])
    .map(|word| word.get(0..1).expect("invalid string"))
    .map(|c| c.to_uppercase())
    .inspect(|item| dbg!(item))  // Debug each item in the iterator
    .collect()
```

Or you can wrap the entire chain:

```rust
dbg!(phrase
    .split(&['-', ' '])
    .map(|word| word.get(0..1).expect("invalid string"))
    .map(|c| c.to_uppercase())
    .collect())
```

## 2. Using `inspect()` method

The `inspect()` method is specifically designed for debugging iterator chains:

```rust
phrase
    .split(&['-', ' '])
    .inspect(|word| println!("After split: {:?}", word))
    .map(|word| word.get(0..1).expect("invalid string"))
    .inspect(|c| println!("After get: {:?}", c))
    .map(|c| c.to_uppercase())
    .inspect(|c| println!("After uppercase: {:?}", c))
    .collect()
```

## 3. Using `tap()` from the `tap` crate

If you want something more similar to Elixir's pipe debugging, you can use the `tap` crate:

```rust
use tap::Tap;

phrase
    .split(&['-', ' '])
    .tap(|words| println!("After split: {:?}", words))
    .map(|word| word.get(0..1).expect("invalid string"))
    .tap(|chars| println!("After get: {:?}", chars))
    .map(|c| c.to_uppercase())
    .tap(|uppercase| println!("After uppercase: {:?}", uppercase))
    .collect()
```

## 4. Custom debug function

You can create your own debug function:

```rust
fn debug<T: std::fmt::Debug>(item: T) -> T {
    println!("Debug: {:?}", item);
    item
}

phrase
    .split(&['-', ' '])
    .map(debug)
    .map(|word| word.get(0..1).expect("invalid string"))
    .map(debug)
    .map(|c| c.to_uppercase())
    .map(debug)
    .collect()
```

## Recommendation

For your specific case, I'd recommend using `inspect()` as it's built into the standard library and designed exactly for this purpose:

```rust
phrase
    .split(&['-', ' '])
    .inspect(|word| println!("Split word: {:?}", word))
    .map(|word| word.get(0..1).expect("invalid string"))
    .inspect(|c| println!("First char: {:?}", c))
    .map(|c| c.to_uppercase())
    .inspect(|c| println!("Uppercase: {:?}", c))
    .collect()
```

This approach:
- Doesn't change the data flow
- Provides clear debugging output
- Is part of the standard library
- Works with any iterator chain

The `inspect()` method is specifically designed for this use case and is the idiomatic Rust way to debug iterator chains.
