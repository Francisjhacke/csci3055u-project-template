# _Rust Language Analysis_

- _Francis Hackenberger_
- _Francis.hackenberger@uoit.net_

## About the language

Rust is a systems programming language that was designed with a primary focus on safe concurrency and support
for both functional and imperative programming. Its syntax is similar to C++ but it provides better memory safety
while still maintaining good performance.

Rust was designed by Graydon Hoare at Mozilla Research with the help of many other contributors. Rust's compiler was released as free and open-source software under both the MIT and Apache licenses. 

Rust provides many interesting tools alongside its safety. For instance:
- Pattern-matching
- Algebraic Datatypes
- Immunitability by default! (You must explicitly define a variable as _mut_ if you want it to be mutable)
- Built-in package manager (Cargo)
- Memory safety is __guaranteed__ without garbage collection.

## About the syntax

*Addition function*

```rust
fn add(x: i32, y: i32){
  x + y;
}
```

## About the tools

### Compilation

The rust compiler runs and compiles to a variety of platforms but is best supported
on Linux, Mac, and Windows on x86. It's as simple as installing the Rust compiler through whichever method
you prefer (ex: Curl). 

### Package Manager

Rust has two main terms that relate to its module system: _Crate_ and _Module_ which are synonymous with _Library_ or _Package_ in other languages. __Cargo__ is Rust's built-in package manager that takes care of managing these Crates and Modules.

## About the standard library

The Rust Standard Library is the foundation of portable Rust software. It offers core types such as __Vec<T>__ and
__Option<T>__, library-defined operations on language primitives, I/O and multithreading, standard macros, and much more.

The standard library is divided into a number of specific modules which all have names similar to _std::slice_ and _std::cmp_.

### A few basic Primitives:
- array: A fixed-sized array
- bool: Boolean type
- char: Character type
- f32: 32-bit floating point type
- i32: 32-bit integer type
- slice: a dynamically-sized view int oa contiguous sequence
- pointer: Raw, unsafe pointers

### Interesting Modules
- alloc: Memory allocation APIS
- hash: Generic hashing support
- sync: Useful synchronization primitives
- thread: Native threads

### Useful Macros
- concat: Concatenates literals into a static string slice
- stringify: Stringifies its arguments
- format: Creates a string using interpolation of runtime expressions
 
## About open source library

There are many, many open-source projects being contributed under the Rust language umbrella. A particularily interesting one is an implementation of Web Sockets (RFC6455) written in Rust.

### Rust-WebSocket

A framework for dealing with WebSocket connections on both clients and servers. The library is currently in an experimental state, however it provides functionality for normal and secure WebSockets, a message level API that supports fragmentation, a dataframe level API, and the ability to extend and customize its behaviour. 

# Analysis of the language

> _Organize your report according to the project description
document_.


