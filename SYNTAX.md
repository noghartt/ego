# The Ego syntax

Thinking a bit about how the syntax of this programming language should work. I
will document here for understand purpose. The main languages that will serve as
an example for it is: Rust and OCaml. So, we will follow some good aspects about it.

## Declaring variables
Ego is imutable by default. To declare a variable you will find three ways to do it:

1. Being an imutable assignment;
2. Being a mutable assignment;
3. Being a constant;

### The imutable assignment

To declare an imutable assignment, you will do it like this:

```rust
let foo = "...";
```

As in Rust, given a scope, you can _shadow_ the same `let` statement as many times
as you wish.

## Declaring and executing functions

### Function overloading (with pattern matching)
