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

