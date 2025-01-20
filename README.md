# rust-skills
Rust Programming language training

https://doc.rust-lang.org/book/


## Ownership 

> https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html


### Ownership Rules


- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Examples

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

## Immutable Borrow (&T)

You can have multiple immutable references to a value at the same time.
These references cannot modify the value.

```rust
fn main() {
    let x = String::from("hello");
    let r1 = &x;
    let r2 = &x; // OK: multiple immutable borrows
    println!("{}, {}", r1, r2);
}
```
## Mutable Borrow (&mut T)

You can have only one mutable reference at a time.
Ensures no other references can access the value while it is being modified.

```rust
fn main() {
    let mut x = String::from("hello");
    let r1 = &mut x;
    // let r2 = &mut x; // ERROR: second mutable borrow
    println!("{}", r1);
}
```

## Combining Immutable and Mutable Borrows

You cannot have both an immutable and a mutable reference to the same value at the same time.
Prevents potential data races.

```rust
fn main() {
    let mut x = String::from("hello");
    let r1 = &x;
    let r2 = &mut x; // ERROR: cannot borrow as mutable while borrowed as immutable
    println!("{}", r1);
}
```

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem multiple immutable borrows
let r2 = &s; // no problem multiple immutable borrows
let r3 = &mut s; // ERROR: second mutable borrow

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

## Dangling References Prevention

The borrow checker prevents references to values that go out of scope.

```rust
fn dangling_ref() -> &String {
    let s = String::from("hello");
    &s // ERROR: `s` goes out of scope, leaving a dangling reference
}
```
## Borrowing in Function Parameters

Functions can accept immutable (&T) or mutable (&mut T) references.
Ownership is not transferred.

```rust
fn print_str(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    print_str(&s); // Borrowing `s`, but ownership remains
}
```

# Common Exceptions & Pitfalls

## Temporary Scope of Mutable References

A mutable reference’s scope ends as soon as it's not used anymore. 

```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // `r1` goes out of scope here, so another mutable borrow is allowed
    let r2 = &mut s;
}
```

## Returning References from Functions

Cannot return a reference to a local variable.

```rust
fn get_ref() -> &String {
    let s = String::from("hello");
    &s // ERROR: `s` goes out of scope
}
```
## Interior Mutability (Cell<T> and RefCell<T>)

Rust allows mutable access inside an immutable reference using RefCell<T>. 

```rust
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello"));
    let r = s.borrow_mut(); // ✅ Allowed via runtime checks
}
```
## RefCell<T> – Borrow Checking at Runtime

Unlike Cell<T>, it allows mutable references (&mut T) dynamically.
Performs runtime borrowing checks instead of compile-time.
Panics if borrow rules are violated.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(String::from("Hello"));

    {
        let mut borrow_mut = data.borrow_mut(); // Mutable borrow at runtime
        borrow_mut.push_str(" World");
    } // Mutable borrow goes out of scope here

    let borrow = data.borrow(); // Immutable borrow is now allowed
    println!("{}", borrow);
}
```

## Mutex<T>– Thread-Safe Interior Mutability

Used for **multi-threading**.
Ensures safe concurrent access using locking.

Prevents data races in multi-threaded programs.
Locks the value so only one thread can modify at a time.
Must handle potential deadlocks (always release locks).

```rust
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(5);

    {
        let mut num = data.lock().unwrap(); // Locking for mutation
        *num += 1;
    } // Lock is released here

    println!("Updated value: {:?}", data.lock().unwrap());
}
```

## RwLock<T> – Optimized for Reads

Like Mutex<T>, but allows multiple readers or one writer at a time.
Ideal for read-heavy workloads.

Allows multiple readers simultaneously.
Only one writer at a time (blocks others).
Used for concurrent data structures.

```rust
use std::sync::RwLock;

fn main() {
    let data = RwLock::new(42);

    {
        let read1 = data.read().unwrap();
        // Multiple readers allowed
        let read2 = data.read().unwrap(); 
        println!("Readers: {} and {}", *read1, *read2);
    }

    {
        // Exclusive write lock
        let mut write = data.write().unwrap(); 
        *write += 1;
    }

    println!("Updated value: {:?}", data.read().unwrap());
}
```

## Lifetime Issues

The borrow checker needs lifetimes ('a) to ensure references outlive their usage.

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```
