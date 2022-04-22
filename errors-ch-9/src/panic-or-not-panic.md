# to `panic!` or Not to `panic!`

How do we decide when we should call `panic!` and when we should return `Result`? When code panics, there's no way to recover. 

<ins>Note: We could call `panic!` for any error situation, whether there’s a possible way to recover or not, but then we’re making the decision that a situation is unrecoverable on behalf of the calling code.</ins>

When we choose to return a `Result` value, we givd the calling code options. The calling code could choose to attempt to recover in a way that's appropriate for its situation, or it could decide that an `Err` value in this case is unrecoverable, so it can call `panic!` and turn our recoverable error into an unrecoverable error. 
Returning `Result` is a good default choice when we're defining a function that might fail. 

In situations in which we're dealing with examples, prototype code, or tests; it's more appropriate to write code that panics instead of returning a `Result`. 

## Examples, Prototype Code, and Tests

When we're writing an example to illustrate some concept, also including robust error-handling code can make the example less clear.
In examples, it's understood that a call to a method like `unwrap` that could panix is meant as a placeholder for the way we'd want our application to handle errors, which can differ based on what the rest of our code is doing. 

Similarly, the `unwrap` and `expect` methods are very handy when prototyping, before we're ready to decide how to handle errors. 
They leave clear markers in our code for when we're ready to make our program more robust. 

If a method call fails in a test, we’d want the whole test to fail, even if that method isn’t the functionality under test. 
Because `panic!` is how a test is marked as a failure, calling `unwrap` or `expect` is exactly what should happen.

## Cases in Which we have more Information than the Compiler

It's also appropriate to call `unwrap` when we have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn’t something the compiler understands.
We'll still have a `Result` value that we need to handle: whatever operation we're calling still has the possibility of failing in general, even though it’s logically impossible in our particular situation.
If we can ensure by manually inspecting the code that we’ll never have an `Err` variant, it’s perfectly acceptable to call `unwrap`.

Example: 

```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

We're creating an `IpAddr` instance by parsing a hardcoded string. We can see that `127.0.0.1` is a valid IP address, so it’s acceptable to use `unwrap` here. 
However, having a hardcoded, valid string doesn't change the return type of the `parse` method: we still get the `Result` value, and the compiler will still
make us handle the `Result` as if the `Err` variant is a possibility because the compiler isn't smart enough to see that this string is always a valid IP address. 

<ins>Note: If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure, we’d definitely want to handle the Result in a more robust way instead.</ins>

## Guidelines for Error Handling

It’s advisable to have our code panic when it’s possible that our code could end up in a bad state.

A **bad state** is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to our code.
It can also result in one or more of the following:

- The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
- Our code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
- There’s not a good way to encode this information in the types you use. 

If someone calls our code and passes in values that don’t make sense, the best choice might be to call `panic!` and alert the person using our library to the bug in their code so they can fix it during development. 
Similarly, `panic!` is often appropriate if we’re calling external code that is out of our control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call.

Example: 

- A parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit.

In this case, returning a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle. 

When our code performs operations on values, our code should verify the values are valid first and panic if the values aren’t valid.
This is mostly for safety reasons: attempting to operate on invalid data can expose our code to vulnerabilities. 
This is the main reason the standard library will call panic! if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem.

<ins>However, having lots of error checks in all of our functions would be verbose and annoying. Fortunately, you can use Rust’s type system (and thus the type checking done by the compiler) to do many of the checks for you.</ins>

If our function has a particular type as a parameter, you can proceed with our code’s logic knowing that the compiler has already ensured you have a valid value. 

Example: 

- If we have a type rather than an `Option`, our program expects to have **something** rather than **nothing**. Our code then doesn’t have to handle two cases for the `Some` and `None` variants: it will only have one case for definitely having a value. Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime.

- Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime.

## Creating Custom Types for Validation

We can use Rust's type system to ensure we have a valid value one step further and look at creating a custom type for validation. 

Example: The guessing game in Chapter 2 in which our code asked the user to guess a number between 1 and 100.

Problem: We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive.

Solution: It would be a useful enhancement to guide the user toward valid guesses and have different behavior when a user guesses a number that’s out of range versus when a user types, for example, letters instead.

Code: 

```
...
loop {
    // ...

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // ...
    }
}
...
```

This is one approach, however it's not an ideal solution. This check above would be tedious to use. 


We can make a new type and put the validations in a function to create an instance of the type rather than repeating validations everywhere. 
This way, it's safe for functions to use the new type in their signatures and confidently use the values they receive. 

Example: Defining a `Guess` type (struct) that will only create an instance of `Guess` if the `new` function receives a value between 1 and 100. 

```
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1|| value > 100 {
            panic!("Guess must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

