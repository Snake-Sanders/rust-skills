# Grinder - Focused Rust Exercise System

## Project Goal

Create a focused exercise system for learning Rust concepts in depth. Unlike the 100-exercises-to-learn-rust which skips between topics, Grinder provides intensive practice on specific Rust concepts to solidify understanding through repetition and variation.

## Original Request

The user wanted:
- A new cargo project called "Grinder" 
- Exercise functions with detailed comments describing goals and requirements
- Comprehensive unit tests for each exercise
- Starting with 2 iterator exercises, with plans to expand later
- More focused, in-depth practice compared to existing exercise systems

## Current Status

- ✅ Cargo project created
- ✅ Initial iterator exercises implemented
- 🔄 Ready for expansion to other Rust topics

## Future Expansion Ideas

### Planned Topics:
- **Iterators** (current focus)
  - Basic iteration patterns
  - Transformations (map, filter, etc.)
  - Reductions (fold, reduce, etc.)
  - Chaining and composition
  - Custom iterators

- **Ownership & Borrowing**
  - Move semantics
  - Reference patterns
  - Lifetime management
  - Smart pointers

- **Error Handling**
  - Result patterns
  - Option handling
  - Error propagation
  - Custom error types

- **Traits & Generics**
  - Trait implementation
  - Associated types
  - Generic constraints
  - Trait objects

- **Pattern Matching**
  - Match expressions
  - Destructuring
  - Guard patterns
  - Exhaustiveness

## Exercise Structure

Each exercise should include:
1. Clear function signature with descriptive name
2. Detailed comment explaining the goal and requirements
3. Examples of expected behavior
4. Comprehensive unit tests covering edge cases
5. Progressive difficulty within each topic

## Usage Instructions

Run tests with: `cargo test`
Run specific exercise tests with: `cargo test exercise_name` 
