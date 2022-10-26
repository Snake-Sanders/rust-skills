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

### References

- An immutable variable can be borrowed but cannot be modified.
- An immutable variable can be borrowed multiple times.
- If a mutable variable is mutably borrowed it can only be borrowed once.
- If a mutable variable is borrowed as read-only, then it cannot be borrowed as mutable.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // Error

println!("{}, {}", r1, r2);
// cannot borrow `s` as mutable more than once at a time
```

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
// cannot borrow `s` as mutable because it is also borrowed as immutable
```

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

The scopes of the immutable references r1 and r2 end after the `println!` where 
they are last used, which is before the mutable reference r3 is created.
