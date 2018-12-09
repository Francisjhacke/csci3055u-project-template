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
- Immunitability by default! (You must explicitly define a variable as _mut_ if you want it to be mutable)
- Built-in package manager (Cargo)
- Memory safety is __guaranteed__ without garbage collection
- The concept of 'ownership' vs 'references/borrowing'
- Powerful macros

## About the syntax
Rust syntax provides a strong mix of keywords and symbols that are not too verbose but are also adequately desciptive. 

*Hello World*
```rust
fn main() {
    println!("Hello, world!");
}
```

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

The standard library is divided into a number of specific modules which all have names similar to "std::module_here"

### A Few Basic Primitives:
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
 
## Open Source Library

There are many, many open-source projects being contributed under the Rust language umbrella. A particularily interesting and popular one is a web framework for Rust, Rocket.

### Rocket

Rocket is a web framework for Rust that focuses on ease-of-use, speed, and expressibility. Rocket is extensively documented with an overview, quickstart guide, and robust API documentation. Here is an example of a Rocket app from the official Github repository (https://github.com/SergioBenitez/Rocket)

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}
```

"Visiting localhost:8000/hello/John/58, for example, will trigger the hello route resulting in the string Hello, 58 year old named John! being sent to the browser. If a string was passed in that can't be parsed the route won't get called, resulting in a 404 error."

# Analysis

## Functional vs. Procedural Programming

Rust is a _multi-paradigm_ language, it takes certain aspects from functional and procedural programming languages such as putting emphasis on immutability, but also has unrestricted side effects and mutability. 

Although Rust may not be a "pure" functional programming language, it has taken a signficant influence from functional programming with the inclusion of features such as **iterators** and **closures** allowing for functions to be used as values, passing them in arguments, and returning them from other functions. 

## Meta-programming / Macros
Macros are a big part of Rust, fundamentally they introduce a way of writing code that writes other code (metaprogramming). 

In Rust, *Macros* refer to a family of features such as:
* Declarative macros (with macro_rules!)
* Procedural macros (Custom, function-like, and attribute-like macros)

Declarative Macros (or macros by example) are the most popular form of Macros in Rust. To be brief, these macros allow you to write things similar to a Rust *match* expression.

Procedural Macros are more similar to functions which accept some code as input and produce some code as output rather than pattern matching and replacing code with other code like declarative macros do.

## Symbol Resolution & Closure
Rust has taken much inspiration from functional programming languages and one significant feature is the ability to use functions as values and pass them as arguments. In Rust, *closures* are essentially anonymous functions that you can store in a variable or pass as arguments to other functions. However, closures can also capture values from the scope in which they're defined (unlike functions). 

A quote from the offical Rust documentation: "Mastering closures and iterators is an important part of writing idiomatic, fast Rust code".

## Scoping rules
Rust has a complex scoping system that plays an important role in _ownership_, _borrowing_, and _lifetimes_. In other words they indicate to the compiler when we have valid borrows, free-able resources, and when variables are created or destroyed.

In general, Rust's _lifetimes_ (a contruct the Rust compilers uses to ensure that borrows are valid) follow __lexical scoping__ rules. 

Additionally, Rust enforces RAII (Resource Acquisition Is Initialization), whenever an object goes out of scope, its destructor is called and its _owned_ resources are freed. Variables in Rust don't simply hold data in the stack, they own _resources_. This allows for Rust to protect you from resource leak bugs, essentially preventing memory leaks.

## Functional programming constructs

Rust's inspiration from other existing functional programming languages has been deeply integrated into Rust such as _Closures_ and _Iterators_. Additionally, some of Rusts other features such as _pattern matching_ and _enums_ are influenced by functional programming style.

## Type system
Rust uses a conventional static type system, meaning it needs to know the types of all variables at _compile_ time. Every variable in Rust has a type and all built-in types are tightly integrated into the language itself. Additionally, by default variables are immutable (You can of course explicitly define variables as mutable using the _mut_ keyword)

## Strengths & Weaknesses
Rust is an incredible language for those who put performance and safety before all else. Because Rust was designed for systems level programming it comes with system-level performance akin to the level of C and C++. However, due to its powerful type system and concepts behind memory management eradicate any memory leaks or null pointer exceptions! 

On the other hand, Rust is known to have a steep learning curve. It's typed memory management implies a major source of complexity. Rust also introduces alot of new concepts that may take some time to become familiar with. Finally, Rust has a relatively small ecosystem when we compare it to other popular languages such as C++ or C#.  

In short, Rust is perfectly suited for writing robust, reliable, and high performance programs. However, as a start-up language it may feel overwhelming, especially for applications where Rust's level of performance is not necessary.

