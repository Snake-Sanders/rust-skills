# rust-skills
Rust Programming language training

https://doc.rust-lang.org/book/


## Lifetime Issues

The borrow checker needs lifetimes ('a) to ensure references outlive their usage.

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```
