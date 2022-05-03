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

The `assert!` macro, provided by the standard library, is useful when we want to ensure that some condition in a test evaluates to `true`. 

We give the `assert` macro an argument that evaluates to a Boolean. 
If the value is `true`, `assert!` does nothing and the test passes. 
If the value if `false`, the `assert!` macro calls the `panic!` macro, which causes the test to fail. 

Using the `assert!` macro helps us check that our code is functioning in the way we intend. 

Example: Using the `Rectangle` struct and its `can_hold` method. 

File - `src/lib.rs`

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

The `can_hold` method return a Boolean, which makes it a perfect case for the `assert!` macro. 

Example: Test for `can_hold` that checks whether a larger rectangle can indeed hold a smaller rectangle

File - `src/lib.rs`

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

<ins>Note: that we’ve added a new line inside the `tests` module: `use super::*;`. 
The `tests` module is a regular module that follows the usual visibility rules</ins>

Because the `tests` module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. 
We use a glob here so anything we define in the outer module is available to this `tests` module. 

Test output: 

```
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/rectangle-6584c4561e48942e)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Example: We're going to add another test in which we asset that a smaller rectangle cannot hold a larger rectangle. 

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
```

We're going to add another function in which we need to negate the result that result before we pass it to the `assert!` macro. 
The test will pass because we added a produces a false negative. 

```
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Both tests pass, but we'll now see what happens when we introduce bugs in our code. 

Example: Changing the implementation of the `can_hold` method by replacing the greater than sign with a less than sign when it compares the widths.

```
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
```

Running `cargo test` will give an error: 

```
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... FAILED
test tests::smaller_cannot_hold_larger ... ok

failures:

---- tests::larger_can_hold_smaller stdout ----
thread 'main' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:28:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

The test caught the bug. 
Because `larger.width` is 8 and `smaller.width` is 5, the comparison of the widths in `can_hold` now returns `false`: 8 is not less than 5.


## Testing Equality with the `assert_eq!` and the `assert_ne!` Macros

A common way to test functionality is to compare the result of the code under test to the value we expect the code to return to make sure they're equal. 

We could do this useing the `assert!` macro and passing it an expression using the `==` operator. 

However, this is such a common test that the standard library provides a pair of macros - `assert_eq!` and `assert_ne!` - to perform this test easier. 

These macros compare two arguments for equality or inequality. 
They'll also print the two values if the assertion fails, which makes it easier to see **why** the test failed;
Conversely, the `assert!` macro only indicates that it got a `false` value for the `==` expression, no the values that led to the `false` value.

Example: Testing the function `add_two` using the `assert_eq!` macro

File - `src/lib.rs`

```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

Test Output: 

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The first argument we gave to the `assert_eq!` macro, `4`, is equal to the result of calling `add_two(2)`. 
The line for the test is `test tests::it_adds_two ... ok`, and the `ok` text indicates that the test passed. 

Example: Introducing a bug that would cause the `assert_eq!` test to fail. 

```
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

Result of what happens when we test it with the same `assert_eq!` test case:

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

The test caught the bug, the `it_adds_two` test failed, displaying the message `assertion failed: '(left == right)'` and shows that `left` was `4` and `right` was `5`. 
This message helps us start debugging, it means that the `left` argument to `assert_eq!` was `4` but the right argument where we had `add_two(2)` was `5`, 

In some languages and test frameworks, the parameters to the functions that assert two values are equal are called `expected` and `actual`, and the order in which we specify the arguments matters. 

However, in Rust, they're called `left` and `right`, and the order in which we specify the value we expect and the value that the code under the test produces doesn't matter. 

The `assert_ne!` macro will pass if the two values we give it aren't equal and fail if they are equal. 
The macro is most useful for cases when we're not sure what a value **will** be, but we know what the value definitely **won't** be if our code is functioning as intended. 

<ins>Example for `assert_ne!` will be useful: If we're testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.</ins>

The `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`. 
When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` trais. 
All of the primitive types and most of the standard library types implement these traits. 

For structs and enums that we define, we'll need to implement `PartialEq` to assert that values of those types are equal or not equal. We'll need to implement `Debug` to print the values when the assertion fails. 


## Adding Custom Failure Messages

We can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. 
Any arguments specified after the one required to `assert!` or the two required arguments to `assert_eq!` and `assert_ne!` are passed along to the `format!` macro, allows us to pass a format string that contains `{}` placeholders and values to go in these placeholders. 

Custom messages are useful to document what an assertion means; when a test fails, we'll have a better idea of what the problem is with the code. 

Example: A function that greets people by name and we want to test that the name we pass into the function appears in the output

File - `src/lib.rs`

```
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

The requirements for the  program haven’t been agreed upon yet, and we’re pretty sure the `Hello` text at the beginning of the greeting will change. 
We decided we don’t want to have to update the test when the requirements change, so instead of checking for exact equality to the value returned from the `greeting` function, we’ll just assert that the output contains the text of the input parameter.

Example: Introducing a bug into the code by changing `greeting` to not include `name` to see what the failure looks like 

```
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

Test output as a result of the failure

```
$ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished test [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'assertion failed: result.contains(\"Carol\")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

The result indicates that the assertion failed and which line the assertion is on. 
A better failure message would print the value we got from the `greeting` function. 

Example: Changing the test function by giving it a custom failure message made from a string format string with a placeholder filled in with the actual value we got from the `greeting` function

```
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

Failure test output

```
$ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished test [unoptimized + debuginfo] target(s) in 0.93s
     Running unittests (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.


## Checking for Panics with `should_panic`

In addition to checking that our code returns the correct values we expect, it's also important to check that our code handles error conditions as we expect. 

Example: Consider the `Guess` type. Other code that uses `Guess` depends on the guarantee that `Guess` instances will contain only values between 1 and 100. 

We can write a test that ensures that attempting to create a `Guess` instance with a value outside that range panics. 

We do this by adding another attribute, `should_panic`, to our test function. This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn't panic. 

Code Example: A test that checks that the error conditions of `Gues::new` happen when we expect them to

File - `src/lib.rs`

```
pub struct Guess { 
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to. Let’s look at the result when this test passes:

```
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests guessing_game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This looks good, we can introduce a bug in our code by removing the condition that the `new` function will panic if the value is greater than 100. 

```
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
```

When we run the test, it will fail

```
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.62s
     Running unittests (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

We don’t get a very helpful message in this case, but when we look at the test function, we see that it’s annotated with `#[should_panic]`. The failure we got means that the code in the test function did not cause a panic.

Tests that use `should_panic` can be imprecise because they only indicate that the code has caused some panic. 
A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting to happen. 
To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute. 

The test harness will make sure that the failure message contains the provided text. 

Example: Testing that a condition will cause a `panic!` with a particular panic message

Code Example: Consider the modified code for `Guess` where the `new` function panics with different messages depending on whether the value is too small or too large. 

File - `src/lib.rs`

```
// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

The test will pass because the value we put in the `should_panic` attribute's `expected` parameter is a substring of the message that the `Guess::new` function panics with. 

We could have specified the entire panic message that we expect. We can also choose to specify in the expected parameter for `should_panic` depends on how much of the panic message is unique or dynamic and how precise we want our test to be.

Example: Checking to see what happens when a `should_panic` test with an `expected` message fails. 

Code Example: Introducing a bug into our code by swapping the bodies of the `if value < 1` and the else `if value > 100` blocks:

```
if value < 1 {
    panic!(
        "Guess value must be less than or equal to 100, got {}.",
        value
    );
} else if value > 100 {
    panic!(
        "Guess value must be greater than or equal to 1, got {}.",
        value
    );
}
```

The test output failing with the according debugging information

```
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

The failure message indicates that the test did indeed panic as we expected, but the panic message didn't include the expected string `'Guess value must be less than or equal to 100'`. 
The panic message that we did get in this case was `Guess value must be greater than or equal to 1, got 200`.


## Using `Result<T, E>` in Tests

We've mostly written tests that panic when they fail. 

We can also write tests that use `Result<T, E>`. 

Here is the test from the first example rewritten to use `Result<T, E>` and return an `Err` instead of panicking: 

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

The `it_works` function now has a return type, `Result<(), String>`. 
In the body of the function, rather than calling the `assert_eq!` macro, we return `Ok(())` when the test passes and an `Err` when a `String` inside when the test fails. 

Writing test so they return a `Result<T, E>` enables us to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant. 

We can't use the `#[should_panic]` annotation on tests that use `Result<T, E>`. To assert that an operation returns an `Err` variant, **don't** use the qustion mark operator on the `Result<T, E>` value. 
Instead, we should use `assert!(value.is_err())`. 

