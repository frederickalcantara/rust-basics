# How to Write Tests

Tests are Rust functions that verify that the prod code is functioning in the expected manner. 

The bodies of test functions typically perform these 3 actions: 

1. Set up any needed data or state 
2. Run the we want to test
3. Asset the results are what we expect

We should look at the features Rust provides specifically writing tests that take these acitons, which include the `test` attribute, a few macros, and the `should_panic` attribute. 

## The Anatomy of a Test Function

At its simplest, a test in Rust is a function that’s annotated with the `test` attribute.
Attributes are metadata about pieces of Rust code; one example is the `derive` attribute we used with structs.

To change a function into a test function add `#[test]` on the line before `fn`. 
When we run our tests with the `cargo test` command, Rust builds a test runner binary that runs the functions annotated with the `test` attribute and reports on whether each test function passes or fails. 

<ins>Note: When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.
This module helps us start writing our tests so we don't have to look up the exact structure and syntax of test functions every time we start a new project</ins>
We can add as s many additional test functions and as many test modules as we want.

To create a new library run this command

```
cargo new <library_name> --lib
```

