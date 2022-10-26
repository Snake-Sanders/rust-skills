# rust-skills
Rust Programming language training

https://doc.rust-lang.org/book/


## Ownership 

> https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html


### Ownership Rules


- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

`hello` is in the heap. the value of `s1` is moved to `s2`. `s1` becomes invalid.

```rust
let s1 = String::from("hello");
let s2 = s1; // moved
```


The value can be copied with `clone`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

On the stack. For primitive types with known sizes, the values are copied within the stack. `x` and `y` are both 5 and valid.

let x = 5;
let y = x;

Another way to copy is by using the function `Copy`, only if this is implemented. Notice that you cannot implement `Copy` if there is an implementation for `Drop` (Deallocate memory/Free).
