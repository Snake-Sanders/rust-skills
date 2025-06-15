## Functional programming features from Elixir to Rust

The most useful Rust functions for functional programming with their Elixir equivalents:

**Iterator Creation & Conversion:**
- `Enum.to_list/1` → `collect()` - Convert iterator to collection
- `Stream.iterate/2` → `std::iter::successors()` - Generate infinite sequence
- `Enum.with_index/1` → `enumerate()` - Add indices to elements
- `Stream.cycle/1` → `cycle()` - Repeat iterator infinitely
- `Enum.zip/2` → `zip()` - Combine two iterators

**Transformation:**
- `Enum.map/2` → `map()` - Transform each element
- `Enum.flat_map/2` → `flat_map()` - Map and flatten
- `Enum.filter/2` → `filter()` - Keep elements matching predicate
- `Enum.reject/2` → `filter()` with negated predicate - Remove matching elements
- `Enum.scan/3` → `scan()` - Accumulate with intermediate results

**Aggregation:**
- `Enum.reduce/3` → `fold()` or `reduce()` - Accumulate to single value
- `Enum.find/2` → `find()` - First element matching predicate
- `Enum.any?/2` → `any()` - Check if any element matches
- `Enum.all?/2` → `all()` - Check if all elements match
- `Enum.count/1` → `count()` - Count elements

**Taking/Dropping:**
- `Enum.take/2` → `take()` - Take first n elements
- `Enum.drop/2` → `skip()` - Skip first n elements  
- `Enum.take_while/2` → `take_while()` - Take while predicate true
- `Enum.drop_while/2` → `skip_while()` - Skip while predicate true

**Partitioning:**
- `Enum.partition/2` → `partition()` - Split based on predicate
- `Enum.group_by/2` → `fold()` with HashMap - Group by key function
- `Enum.chunk_every/2` → `array_chunks()` (nightly) or external crate

**Utilities:**
- `Enum.reverse/1` → `rev()` - Reverse iterator
- `Enum.min/1` → `min()` - Find minimum
- `Enum.max/1` → `max()` - Find maximum

Most of these work on Rust's `Iterator` trait. Remember that Rust iterators are lazy like Elixir's `Stream` module - they don't execute until consumed with methods like `collect()`, `for_each()`, or `fold()`.
