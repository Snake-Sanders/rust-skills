# Grinder 🦀

A focused Rust exercise system designed for deep practice and mastery of specific concepts.

## Philosophy

Unlike other exercise collections that jump between topics, Grinder focuses on **intensive practice** of individual Rust concepts through multiple, carefully crafted exercises. Each topic includes:

- Progressive difficulty levels
- Comprehensive test coverage
- Detailed explanations and learning objectives
- Real-world applicable patterns

## Getting Started

### Running All Tests
```bash
cargo test
```

### Running Specific Exercise Tests
```bash
# Run only iterator exercises
cargo test iterators

# Run specific exercise
cargo test transform_pipeline
cargo test analyze_positive_numbers
```

### Working on Exercises

1. Open `src/lib.rs`
2. Find the exercise functions (marked with `TODO`)
3. Read the detailed comments explaining the goals and requirements
4. Implement the function
5. Run tests to verify your solution

## Current Exercises

### 🔄 Iterators Module
- **Exercise 1: transform_pipeline** - Practice chaining iterator methods with filtering and mapping
- **Exercise 2: analyze_positive_numbers** - Learn reduction operations and statistical calculations

## Exercise Structure

Each exercise includes:

- **Clear Goal**: What you're trying to achieve
- **Requirements**: Specific steps to implement
- **Learning Objectives**: Key concepts being practiced
- **Examples**: Input/output demonstrations
- **Comprehensive Tests**: Edge cases and various scenarios

## Tips for Success

1. **Read the comments thoroughly** - They contain all the information you need
2. **Look at the test cases** - They show expected behavior and edge cases
3. **Start simple** - Get basic functionality working before handling edge cases
4. **Use the compiler** - Rust's error messages are your friend
5. **Test frequently** - Run `cargo test exercise_name` after each change

## Expanding Grinder

Future topics planned:
- Ownership & Borrowing
- Error Handling  
- Traits & Generics
- Pattern Matching
- Lifetimes
- Smart Pointers

## Contributing

This is a learning-focused project. Feel free to:
- Add more exercises to existing modules
- Create new topic modules
- Improve test coverage
- Enhance documentation

---

**Happy grinding! 🚀** 
