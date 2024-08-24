# Rust Revision

## 1. Rust Fundamentals

### Cargo

- **Cargo**: Rust's build system and package manager.
- **Crates**: Reusable code packages in Rust.
- **Crates.io**: The central repository for Rust crates.
- **cargo.toml**: Configuration file defining dependencies, version, and project metadata.
- **main.rs**: The entry point of a Rust application.

### Why Rust?

- **Memory Safety**: Prevents null pointers and buffer overflows.
- **Concurrency**: Safe concurrency without data races.
- **Performance**: Comparable to C/C++ with zero-cost abstractions.
- **Ownership System**: Ensures memory safety without garbage collection.
- **Type Inference**: Strong static typing with type inference.

### Comparing Rust to JavaScript

- **Performance**: Rust is faster, with lower-level control.
- **Memory Management**: Rust uses ownership; JavaScript uses garbage collection.
- **Concurrency**: Rust offers safe concurrency; JavaScript uses single-threaded event loop.
- **Compiled vs. Interpreted**: Rust is compiled; JavaScript is interpreted.
- **Type Safety**: Rust is statically typed; JavaScript is dynamically typed.

### VS Code Extensions

- **CodeLLDB**: Debugger for Rust, enabling step-through debugging.
- **Rust Analyzer**: Advanced language server providing IDE-like features, such as autocompletion and code navigation.

## 2. Rust Variables

### Variable Declaration and Initialization

- **Immutable Variables**: Variables are immutable by default.
- **Mutable Variables**: Declare with `mut` to allow modification.

#### Example:

```rust
fn main() {
    let num: i32 = 10; // Immutable integer
    let mut mutable_num: i32 = 20; // Mutable integer
    mutable_num += 5;

    let name: &str = "Rust"; // Immutable string slice
    let mut mutable_name = String::from("Rust"); // Mutable String object
    mutable_name.push_str(" Programming");
  
    println!("Number: {}", num);
    println!("Mutable Number: {}", mutable_num);
    println!("Name: {}", name);
    println!("Mutable Name: {}", mutable_name);
}
```

### Data Types

* **Integers** : `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
* **Strings** : `&str` (string slice), `String` (growable heap-allocated string)

### Memory Locations and Allocation

- **Stack**:

  - Fixed-size variables (e.g., `i32`) are stored here.
  - Fast and automatically managed.
- **Heap**:

  - Growable types (e.g., `String`) are stored here.
  - Allows dynamic size but requires manual management.

#### Example:

```rust
fn main() {
    let num: i32 = 10; // Stack
    let name: &str = "Rust"; // Static memory
    let mut mutable_name = String::from("Rust"); // Heap

    mutable_name.push_str(" Programming");

    println!("Number: {}", num);
    println!("Name: {}", name);
    println!("Mutable Name: {}", mutable_name);
}
```

## 3.If-Else Implementation

**`if`-`else` statements** allow branching based on conditions. Rust requires that all branches of an `if-else` chain return values of the same type.

#### Example:

```rust
fn main() {
    let is_male: bool = true;
    let is_female: bool = false;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a man");
    } else if is_female && is_above_18 {
        println!("You are a woman and above 18");
    } else {
        println!("You do not meet the criteria");
    }
}


```


## 4. String Implementation

**Initialization:**

- **`String`**: Created with `String::from("text")`, a growable heap-allocated string.
- **`&str`**: A string slice or reference, initialized with a string literal.

### Explanation

- **`String`**: For dynamically sized strings. Initialized using `String::from`.
- **`&str`**: For immutable string slices. Initialized with string literals.

**Accessing a Character at an Index:**

- `string2.chars()`: Returns an iterator over the characters of `string2`.
- `.nth(1000)`: Attempts to access the character at index 1000.
- **`Option<char>`**: `Some(char)` if the index is valid, or `None` if out of bounds.
- Unicode characters can make direct indexing inefficient or error-prone. Using `.chars().nth()` safely handles out-of-bounds access.

#### Example:

```rust
fn main() {
    let name: String = String::from("John"); // Initialize a String
    let string2: &str = "Hello, world!"; // Initialize a string slice

    println!("Hello, {}!", name);
    println!("Hello, {}!", string2);

    // Attempting to access a character at index 1000 
    let char1: Option<char> = string2.chars().nth(1000);

    match char1 {
        Some(c) => println!("The char is {}", c), // Character found at index
        None => println!("The char is None"),     // Index is out of bounds
    }
}
```
## 5. Function Implementation

**Function Definition:**

- Functions in Rust are defined using the `fn` keyword.
- They can take parameters and return values.

### Example

```rust
fn main() {
    let mut num: i32 = 3;

    for _i in 0..5 {
        num = num + 1;
        println!("{}", num);   
    }

    if num > 5 {
        println!("It is really greater than 5 and the value is {}", num);
    } else {
        println!("It is not {}", num);
    }

    // Calling the addition function
    addition(3, num);
    addition(2, num);

    let mut _data_owning_string: String = String::from("The data_owning_string is heap stored, mutable, owned by declaration");
    let _string_slice: &str = "string_slice is referenced and used for borrowing part of string";
}

pub fn addition(a: i32, b: i32) {
    println!("The Sum is {}", a + b);
}
```

## 6. Mutability Implementation

**Mutable Variables:**

- **`mut` Keyword**: Allows modification of a variable after its initialization.

### Example

```rust
fn main() {
    let mut num: i32 = 3;
    num = num + 1; // Modify the value of num
    println!("{}", num);

    let mut s: &str = "Mutable string";  // A &str (string slice) is a reference to a string, usually immutable by nature. Even if you declare it as mut, it only allows you to reassign the reference, not mutate the underlying data.
    s = "mutated string"; // string slice is reassigned to a new value

    let mut s2 = String::from("Mutable string slice");
    s2.clear(); // Clears the contents of the string
    s2.push_str("Completely new content"); // Replaces the contents

    println!("{}  {}", s, s2);
}
```
