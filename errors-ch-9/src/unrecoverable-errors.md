# Unrecoverable Errors with `panic!`

Rust has the `panic!` macro whenever bad things happen in our code. 
When the `panic!` macro executes, our program will print a failure message, unwind and clean up the stack, and then quit. 

We invoke a panic when a bug of some kind has been detected and it's not clear how to handle the problem at the time we're writing our program. 

## Unwinding the Stack or Aborting in Response to a Panic

By default, whenever a panic occurs, the program starts **unwinding**, or Rust walks back up the stack and cleans up the data from each function it encounters. 
Walking back and doing clean up is a lot of work. 

Rust allows us to choose the alternative of immediately **aborting**, in which the program ends without doing any cleanup. 

Memory that the program was using will then need to be cleaned up by the OS. 

If we make the binary as small as possible, we can switch from unwinding to aborting upong a panic by adding a `panic = 'abort'` to the appropriate `[profile]` sections in our **Cargo.toml** file. 

Example: If we want to abort on panic in release mode, we can do this

```
[profile.release]
panic = 'abort'
```

Example: Calling `panic!` in a program 

File `src/main.rs` 

```
fn main() {
    panic!("crash and burn");
}
```

When we run the program we will get a panic message. 

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

The call to `panic!` causes the error message contained in the last 2 lines. 
The first line shows our panic message and the place in our source code where the panic occurred. 

In our example, the line indicated is part of our code, and if we go to that line, we see the `panic!` macro call. 

In other cases, the `panic!` call might be in code that our code calls, and the filename and line number reported by the error message will be someone else’s code where the `panic!` macro is called, not the line of our code that eventually led to the `panic!` call.
We can use the backtrace of the functions the `panic!` call came from to figure out the part of our code that's causing the problem. 

## Using a `panic!` Backtrace

Example: Looking at an example when a `panic!` call comes from a library because of a bug in our code instead of from our code calling the macro directly. 

File `src/main.rs`

```
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

This code will result in a panic because we're trying to access the 100th element of our vector when the vector only has 3 elements. 
Rust will naturally panic 

In C, attempting to read beyond the end of a data structure is undefined behavior. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a **buffer overread** and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.

To protect our program from a buffer related vulnerability, Rust will the stop the execution and refuse to continue. 

We can get a backtrace of the runtime compilation if set the `RUST_BACKTRACE` environment variable to get a backtrace of what happened to cause the error. 
A **backtrace** is a list of all functions that were called to get to that point. 

Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. 
That’s the spot where the problem originated. The lines above that spot are code that your code has called; the lines below are code that called your code. 
These before-and-after lines might include core Rust code, standard library code, or crates that you’re using. 

Example: Running a backtrace 

```
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

The output will differ depending on the OS and Rust version. In order to get debug information, debug symbols must be enabled. 
Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag. 

If we don’t want our program to panic, we should start our investigation at the location pointed to by the first line mentioning a file we wrote.

When our code panics in the future, you’ll need to figure out what action the code is taking with what values to cause the panic and what the code should do instead.