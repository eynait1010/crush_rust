# crush_rust

## Guessing Game

## 3.Common Programming Concepts
### 3.1 Variables and Mutability
- let
- mut
- const
- shadowing 

### 3.2 Data Types
#### Scalar Types
- - [ ] i8 (-2^7 - 2^7-1)
- u8 (0 - 2^7-1)
- - [ ] Two's complement
- - [ ] Floating-Point Types
- isize usize
#### Compound Types
##### tuple
- pattern matching 
- destructure
##### array
- Arrays are useful when you want your data allocated on the stack rather than the heap
- Unlike a tuple, every element of an array must have the same type. 
- Unlike arrays in some other languages, arrays in Rust have a fixed length.

##### vector
- heap

### 3.3 Functions

Rust doesn’t care where you define your functions, only that they’re defined somewhere.
- parameters
- arguments
- statement 
Statements are instructions that perform some action and do not return a value. 
- expressions 
Expressions evaluate to a resulting value.
Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 

Function bodies are made up of a series of statements optionally ending in an expression. 

### 3.5 Control Flow

- if Expressions
The condition in this code must be a bool.
Using if in a let Statement: variables must have a single type, and Rust needs to know at compile time what type the number variable is
- loop
- while
- for
