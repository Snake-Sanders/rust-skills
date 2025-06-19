# 2D vectors

## Basic 2D Vector Creation Function

```rust
fn create_2d_vector(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    vec![vec![0; cols]; rows]
}
```

## With Custom Initial Value

```rust
fn create_2d_vector_with_value(rows: usize, cols: usize, value: u8) -> Vec<Vec<u8>> {
    vec![vec![value; cols]; rows]
}
```

## Using Iterator Approach (More Idiomatic)

```rust
fn create_2d_vector_iter(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    (0..rows)
        .map(|_| vec![0; cols])
        .collect()
}
```

## With Custom Initialization Function

```rust
fn create_2d_vector_with_init<F>(rows: usize, cols: usize, init_fn: F) -> Vec<Vec<u8>>
where
    F: Fn(usize, usize) -> u8,
{
    (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| init_fn(row, col))
                .collect()
        })
        .collect()
}
```

## Using `Vec::with_capacity` for Performance

```rust
fn create_2d_vector_capacity(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut matrix = Vec::with_capacity(rows);
    for _ in 0..rows {
        matrix.push(vec![0; cols]);
    }
    matrix
}
```

## Usage Examples

```rust
fn main() {
    // Basic usage
    let matrix1 = create_2d_vector(3, 4);
    println!("{:?}", matrix1);
    // Output: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    
    // With custom value
    let matrix2 = create_2d_vector_with_value(2, 3, 42);
    println!("{:?}", matrix2);
    // Output: [[42, 42, 42], [42, 42, 42]]
    
    // With custom initialization
    let matrix3 = create_2d_vector_with_init(3, 3, |row, col| (row + col) as u8);
    println!("{:?}", matrix3);
    // Output: [[0, 1, 2], [1, 2, 3], [2, 3, 4]]
}
```

## For Image/Grid Applications

```rust
fn create_image_matrix(width: usize, height: usize) -> Vec<Vec<u8>> {
    vec![vec![0; width]; height]
}

// Or with a specific pattern
fn create_checkerboard(size: usize) -> Vec<Vec<u8>> {
    (0..size)
        .map(|row| {
            (0..size)
                .map(|col| if (row + col) % 2 == 0 { 0 } else { 255 })
                .collect()
        })
        .collect()
}
```

## Safe Access Helper Functions

Based on safe vector access patterns:

```rust
fn get_safe(matrix: &Vec<Vec<u8>>, row: usize, col: usize) -> Option<u8> {
    matrix.get(row)?.get(col).copied()
}

fn set_safe(matrix: &mut Vec<Vec<u8>>, row: usize, col: usize, value: u8) -> bool {
    if let Some(row_vec) = matrix.get_mut(row) {
        if let Some(cell) = row_vec.get_mut(col) {
            *cell = value;
            return true;
        }
    }
    false
}
```

## Iterating Over 2D Vectors

```rust
fn iterate_2d_vector(matrix: &Vec<Vec<u8>>) {
    // Using enumerate for indices
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            println!("matrix[{}][{}] = {}", row_idx, col_idx, value);
        }
    }
    
    // Using iterators (functional approach)
    matrix.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, &value)| (row_idx, col_idx, value))
        })
        .for_each(|(row, col, value)| {
            println!("matrix[{}][{}] = {}", row, col, value);
        });
}
```

## Common Patterns

### Creating Identity Matrix
```rust
fn create_identity_matrix(size: usize) -> Vec<Vec<u8>> {
    (0..size)
        .map(|row| {
            (0..size)
                .map(|col| if row == col { 1 } else { 0 })
                .collect()
        })
        .collect()
}
```

### Creating Spiral Pattern
```rust
fn create_spiral_matrix(size: usize) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut value = 1u8;
    let mut top = 0;
    let mut bottom = size - 1;
    let mut left = 0;
    let mut right = size - 1;
    
    while top <= bottom && left <= right {
        // Fill top row
        for col in left..=right {
            matrix[top][col] = value;
            value = value.wrapping_add(1);
        }
        top += 1;
        
        // Fill right column
        for row in top..=bottom {
            matrix[row][right] = value;
            value = value.wrapping_add(1);
        }
        right = right.saturating_sub(1);
        
        if top <= bottom {
            // Fill bottom row
            for col in (left..=right).rev() {
                matrix[bottom][col] = value;
                value = value.wrapping_add(1);
            }
            bottom = bottom.saturating_sub(1);
        }
        
        if left <= right {
            // Fill left column
            for row in (top..=bottom).rev() {
                matrix[row][left] = value;
                value = value.wrapping_add(1);
            }
            left += 1;
        }
    }
    
    matrix
}
```

## Performance Considerations

1. **Use `vec!` macro** for simple initialization - it's optimized
2. **Use `Vec::with_capacity`** when you know the size and need performance
3. **Use iterators** for functional programming style and memory efficiency
4. **Avoid nested loops** when possible - use iterator methods instead

## Memory Layout

A 2D vector `Vec<Vec<u8>>` is actually a vector of vectors:
- Each row is a separate `Vec<u8>` allocated on the heap
- Rows may not be contiguous in memory
- For large matrices, consider using a single `Vec<u8>` with index calculation

```rust
// Alternative: Single vector with index calculation
fn create_flat_matrix(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn get_flat_index(row: usize, col: usize, cols: usize) -> usize {
    row * cols + col
}

fn get_from_flat(matrix: &Vec<u8>, row: usize, col: usize, cols: usize) -> Option<u8> {
    let index = get_flat_index(row, col, cols);
    matrix.get(index).copied()
}
```
