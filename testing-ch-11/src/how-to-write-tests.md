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

Example: The test module and function generated automatically by `cargo new`

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

The `#[test]` annotation before the fn line indicates that this is a test function, so the test runner knows to treat this function as a test. 

We could also have non-test functions in the `tests` module to help set up common scenarios or perform common operations, so we need to indicate which functions are tests by using the `#[test]` attribute.

The function body uses the `assert_eq!` macro to asset that 2+2 equals 4. This assertion serves as an example of the format for a typical test.

Example: Output from running the automatically generated test

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

After the `Compiling`, `Finished`, and `Running` lines is the line `running 1 test`. 
The line afterwards shows the name of the generated test function, `it_works`, and the result of running that test `ok`. 
The summary of running the tests appears next. 
The text `test result: ok` means that all of the tests passed, and the portion that reads `1 passed; 0 failed` totals the number of tests that passed or failed. 

Because we don’t have any tests we’ve marked as ignored, the summary shows `0 ignored`. 
We also haven’t filtered the tests being run, so the end of the summary shows `0 filtered out`.

The `0 measured` statistic is for benchmark tests that measure performance. 
Benchmark tests are, as of this writing, only available in nightly Rust.

The next part of the test output, which starts with `Doc-tests adder`, is for the results of any documentation tests.
We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation
This feature helps us keep our docs and our code in sync.

Example: Changing the test function to a different name

File - `src/lib.rs`

```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```

New output: 

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.59s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The output shows `exploration` as the function name instead of `it_works`. 

Example: Adding a second test that will fail because we called the `panic!` macro

Tests fail when something in the test function panics. 
Each test is ran in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. 

File - `src.lib.rs`

```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

When we run the tests with `cargo test`. The output will show us that the `exploration` test passed and `another` failed. 

Output: 

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::another ... FAILED
test tests::exploration ... ok

failures:

---- tests::another stdout ----
thread 'main' panicked at 'Make this test fail', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

Instead of `ok`, the line `test tests::another` shows `FAILED`. 

2 new sections appear between the individual results and the summary: 

- The first section displays the detailed reason for each test failure. In this case, `another` failed because it `panicked at 'Make this test fail'`, which happens on line 10 in the `src/lib.rs` file. 
- The second section lists the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output. 

We can use the name of a failing test to run just that test to more easily debug it.

The summary line displays at the end: overall, our test result is `FAILED`. We had one test pass and one test fail.


## Checking Results with the `assert!` Macro 

