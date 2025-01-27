# Time

## Standard Library (std::time)

### Get System Time

```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => println!("Unix timestamp: {}", duration.as_secs()),
        Err(_) => println!("Time went backwards!"),
    }
}
```
Returns a timestamp (seconds since 1970-01-01 00:00:00 UTC):

> `Unix timestamp: 1706298543`

## Measure Time Duration

```rust
use std::time::{Instant};

fn main() {
    let start = Instant::now();
    
    // Simulate work
    std::thread::sleep(std::time::Duration::from_secs(2));

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}
```

> `Elapsed time: 2.00s`

## Chrono Crate – Full Date & Time Handling

```rust
use chrono::Local;

fn main() {
    let now = Local::now(); // or Utc::now()
    println!("{}", now); // Example: 2025-01-26 14:45:12.345678 +01:00
}
```

### Add & Subtract Time (Duration)

```rust
use chrono::{Duration, Utc};

fn main() {
    let now = Utc::now();
    let future_time = now + Duration::days(7); // Add 7 days
    let past_time = now - Duration::hours(5); // Subtract 5 hours

    println!("Now: {}", now);
    println!("One week later: {}", future_time);
    println!("Five hours ago: {}", past_time);
}
```
### Format a Date/Time (format!())

```rust
use chrono::Local;

fn main() {
    let now = Local::now();
    let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("Formatted: {}", formatted);
}
```

## Time Crate – Lightweight Alternative


```rust
// Cargo.toml
[dependencies]
time = { version = "0.3", features = ["std", "local-offset"] }
```

### Get Current UTC & Local Time

```rust
use time::OffsetDateTime;

fn main() {
    let now_utc = OffsetDateTime::now_utc();
    let now_local = OffsetDateTime::now_local().expect("Failed to get local time");

    println!("UTC: {}", now_utc);
    println!("Local: {}", now_local);
}
```

### Add & Subtract Durations

```rust
use time::{Duration, OffsetDateTime};

fn main() {
    let now = OffsetDateTime::now_utc();
    let future = now + Duration::days(10);
    let past = now - Duration::hours(12);

    println!("Now: {}", now);
    println!("+10 days: {}", future);
    println!("-12 hours: {}", past);
}
```
