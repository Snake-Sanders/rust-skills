# Ranges

Ranged only counts ascending, if the start value is smaller than the end value the range is empty!

`(5..=1)` this will not work, instead use ascending count and then reverse it `(1..=5).rev`

```rust

fn main() {
    let result: String = (1..=5).rev()
        .map(|n| 
        {let x = n.to_string(); 
            println!("iteration for  {}",x);
            x
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("Collected result: {:#?}", result); 
}
```

