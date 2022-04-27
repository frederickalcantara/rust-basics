# Error Handling

In Rust, we're required to acknowledge the possibility of an error and take some action before our code compiles. 
This requirement makes our program more robust by ensuring that we'll discover errors and handle them appropriately before we've deployed our code to production.  

Rust groups errors into 2 major categories: 

- Recoverable 
    - For recoverable errors, such as a **file not found** error, we most likely just want to report the problem to the user and retry the operation. 

- Unrecoverable
    - These types of errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program. 

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions.

Rust doesn't have exceptions, it has a types and macros for errors. 

- Recoverable
    - Type `Result<T, E>`

- Unrecoverable
    - Macro `panic!` that stops execution when the program encounters an unrecoverable error. 

