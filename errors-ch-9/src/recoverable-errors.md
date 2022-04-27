# Recoverable Errors with `Result` 

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. 

The `Result` enum is defined as having 2 variants: `Ok` and `Err`: 

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters. 

- `T` represents the type of the value that will be returned in a success case within the `Ok` variant. 
- `E` represents the type of the error that will be returned in a failure case within the `Err` variant. 

Due to the fact that `Result` has these generic type parameters, we can use the `Result` type and the function defined on it in many different situations where the successful value and error value we want to return may differ.

Example: Opening a file 

```
use std::fs::File;

fn main() {
    let f: u32 = File::open("hello.txt");
}
```

`File::open` returns a `Result` enum. 
This code won't compile because when we give f a type annotation that we know is not the return type of the function and then try to compile the code, the compiler will tell us that the types don’t match.

The error message will tell us what the type of `f` is. 

Result of the code: 

```
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<File, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `error-handling` due to previous error
```

This tells us the return type of the `File::open` function is a `Result<T, E>`. The generic parameter `T` has been filled in here with the type of the success value, `std::fs::File`, which is a file handle. The type of `E` used in the error value is `std::io::Error`.
The return type means the call to `File::open` might succeed and return a file handle that we can read from or write to. 
The function call also might fail: for example, the file might not exist, or we might not have permission to access the file.

The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. 
This information is exactly what the `Result` enum conveys.

In the case where `File::open` succeeds, the value in the variable `f` will be an instance of `Ok` that contains a file handle. 
In the case where it fails, the value in `f` will be an instance of `Err` that contains more information about the kind of error that happened.

We can use the `match` expression as a way of handling the output of the `Result` enum. 

Example: Using a `match` expression to handle the `Result` variants that might be returned

File `src/main.rs`

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

Similar to the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude, so we don't need to specify the `Result::` before the `Ok` and `Err` variants in the `match` arms. 

When the result is `Ok`, the code will return the inner `file` value out of the `Ok` variant, and we can then assign that file handle value to the variable `f`. After the `match`, we can use the file handle for reading and writing. 

The other arm of the `match` handles the case where we get an `Err` value from `File::open`. 

## Matching on Different Errors

We want to take different actions for different failure reasons. 

If `File::open` fails because the file doesn't exist, we want to create the file and return the handle to the new file. 

If `File::open` failed for any other reason, (example: We don't have permissions to open the file) we will still want the code to `panic!`

Example: Handling different kinds of errors in different ways

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

The type of the value `File::open` returns inside the `Err` variants is `io::Error`, which is a struct provided by the standard library. 
This struct has a maethod `kind` that we can call to get an `io::ErrorKind` value. 
The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation.

The variant that we used is `ErrorKind::NotFound`, which indicates the file we're trying to open doesn't exist yet. 

### Alternatives to Using `match` with `Result<T, E>`

The `match` expressions is useful, but very primitive. 

We can use closures, which are used with many of the methods that are defined on `Result<T, E>`. 
These methods can be more concise than using `match` when handling `Result<T, E>` values in our code. 

Example: Using closures and the `unwrap_or_else` method: 

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}
```

## Shortcuts for Panic on Error: `unwrap` and `expect`

`match` is useful, but it can be a bit verbose and doesn't always communicate intent well. 
The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks. 
The `unwrap` method is a shortcut method implemented just like the `match` expression. 

If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. 
If the `Result` value is the `Err` variant, `unwrap` will call the panic! macro for us. 

Example: `unwrap` in action

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

If we run the code without a "hello.txt" file, then we will get a `panic!` call error message from the `unwrap` method: 

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

Similarly, the `expect` method lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier. 

File: `src/main.rs`

```
use std::fs:File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

We use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!` macro. 

Output example: 

```
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Because this error message starts with the text we specified, `Failed to open hello.txt`, it will be easier to find where in the code this error message is coming from.

## Propagating Errors

When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.
This is also known as **propagating** the error and gives us more control to the calling code, where there might be more information of logic that dictates how the error should be handled than what we have available in the context of our code. 


Verbose Example of handling errors using `match`

```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.

## A Shortcut for Propagating Errors: The `?` Operator

Example: handling errors using the `?` operator

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}
```

The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` values.

There is a difference between what the `match` expression does and what the `?` operator does: 
Error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another. 
When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function.

<ins>Note: This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.</ins>

As long as there’s an `impl From<OtherError> for ReturnedError` to define the conversion in the trait’s `from` function, the `?` operator takes care of calling the `from` function automatically.

The `?` at the end of the `File::open` call will return the value inside an `Ok` to the variable `f`.
If an error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code. 
The same thing applies to the `?` at the end of the `read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. 

Example: Same thing as above but with chained method calls after the `?` operator

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

We can make this operation short by directly reading a file into a string. 
The Std library provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it. 

## Where the `?` Operator can be Used

The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on. This is because the `?` operator is defined to perform an early return of a value out of the function, in the same manner as the `match` expression.

The `match` was using a `Result` value, and the early return arm returned an `Err(e)` value. The return type of the function has to be a `Result` so that it’s compatible with this `return`.

Example: Code that will not compile 

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

The code will open, but it might fail. The `?` operator follows the `Result` value returned by `File::open`, but the `main` function has the return type of `()`, not `Result`. 
When we compile the error, we will get an error that points out that we're only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`. 

To fix the error, we have 2 choices: 

1. Change the return type of our function to be compatible with the value we're using the `?` operator on as long as we have no restrictions preventing that. 
2. Use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way it's appropriate. 

The main function can also return a `Result<(), E>`. 

Example: Changing `main` to return `Result<(), E>` allows the use of the `?` operator on `Result` values

```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

The `Box<dyn Error>` type is a trait object, that means "any kind of error". 
Using `?` on a `Result` value in a `main` function with the error type `Box<dyn Error>` is allowed, because it allows any `Err` value to be returned early. 

When a `main` function returns a `Result<(), E>`, the executable will exit with a value of `0` if main returns `Ok(())` and will exit with a nonzero value if `main` returns an `Err` value. 

<ins>Note: Executables written in C return integers when they exit: programs that exit successfully return the integer 0, and programs that error return some integer other than `0`. Rust also returns integers from executables to be compatible with this convention.</ins>

The `main` function may return any types that implement the `std::process::Termination` trait.