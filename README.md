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

## Ownership

### 4.1 conception
- heap
The heap is less organized: when you put data on the heap, 
1. you request a certain amount of space. 
2. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use
3.  returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating.

- ownership rule
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
- Move

#### Ownership and Functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.Returning values can also transfer ownership. 

### 4.2References and Borrowing
#### borrowing
```rs
let a = String::from("hello");
func(&a)
```
#### Mutable References
```rs
let mut s = String::from("hello");
change(&mut s);
```
#### rules
At any given time, 
you can have either one mutable reference or any number of immutable references.
References must always be valid. **scopes is matter**
#### Dangling References
Data will not go out of scope before the reference to the data does
So when create a variable, return it rather than its reference

### 4.3 Slice
Having to worry about the index in word getting out of sync with the data in s is tedious and error prone! So slice
String Literals Are Slices

**iter** is a method that returns each element in a collection  
**enumerate** wraps the result of iter and returns each element as part of a tuple instead. 

## structure

### 5.1 
- Field Init Shorthand 
- Struct Update Syntax
- Tuple 
  
### 5.2 method
