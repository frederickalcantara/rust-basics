# Traits: Defining Shared Behavior

A **trait** tells the Rust compiler about functionality that a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior. 

<ins>Note: Traits are similar to a feature called **interfaces** in other languages, although with some differences.</ins>


## Defining a trait

A type's behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose. 


Code Example: A `Summary` trait that consists of the behavior provided by a `summarize` method

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

In the example above, we declare a trait using the `trait` keyword and then the trait's name, which is `Summary`. We've also declared the trait as `pub` so that crates depending on this crate can make use of this trait sas well. 

Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait, which is `fn summarize(&self) -> String`

After the method signature, instead of providing an implementation within curly brackets, we use a semicolon. Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly. 

A trait can have multiple methods in its body: 

- The method signatures that are listed one per line
- Each line that ends in a semicolon


## Implementing a Trait on a Type

We can implement the trait's method on the types in our media aggregator. 

Example: Implementing the `Summary` trait on the `NewsArticle` and `Tweet` types

File: `src/lib.rs`

```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Implementing a trait on a type is similar to implementing regular methods. The difference is that after `impl`, we put the trait name that we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for. Within the `impl` block, we put the method signatures that the trait defintion has defined. 

The library has implemented the `Summary` trait on `NewsArticle` and `Tweet`, users of the crate can call the trait methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods. The only difference is that the trait has to be brought into scope as well as the types to get the additional trait methods. 

Example: How a binary crate could use our `aggregator` library crate

```
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

The code prints `1 new tweet: horse_ebooks: of course, as you probably know, people`.

Other crates that depend on the `aggregator` crate can also bring the `Summmary` trait into scope to implement the trait on their own types. One restriction to note with trait implementations is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. 

Exammple use case with traits: We can implement standard library traits like `Display` on a custom type like `Tweet` as part of our `aggregator` crate functionality, because the type `Tweet` is local to our `aggregator` crate. We can also implement `Summary` on `Vec<T>` in our `aggregator` crate, because the trait `Summary` is local to our `aggregator` crate.

We can't implement external traits on external types. 

Example: We can't implement the `Display` trait on `Vec<T>` within our `aggregator` crate, because `Display` and `Vec<T>` are defined in the standard library and aren't local to our `aggregator` crate. 

This restriction is part of a property of programs called **coherence**, and more specifically the **orphan rule**, so named because the parent type isn't present. This rule ensures that other people's code can't break our code and vice versa. Without the rule, 2 crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.


## Default Implementations

It's sometimes useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method's default behavior. 

Example: How to specify a default string implementation for the `summarize` method of the `Summary` trait

```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

If we want to use default implementations of a trait instead of defining a custom implementation, we specify an empty `impl` block with `impl Summary` for `NewsArticle {}`. 

Even if we're not defining a mehod on a struct directly, there's a default implementation and it specified that the `NewsArticle` struct implements the `Summary` trait. 

Code Example: As a result, we can still call the `summarize` method on an instance of the `NewsArticle` struct. 

```
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
    ),
};

println!("New article available! {}", article.summarize());
```

The code above prints out `New article available! (Read more...)`.

Creating a default implementation for `summarize` doesn't require us to change anythign about the implementation of `Summary` on `Tweet`. This is because the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn't have a default implementation. 

Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation. This way, a trait can provide some useful functionality and only require implementors to specify a small part of it. 

Example: We could define the `Summary` trait to have a `summarize_author` method whose implementation is required, and then define a summarize method that has a default implementation that calls the `summarize_author` method:

```
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

If we want to the version of `Summary` above, then we only need to define `summarize_author` when we implement the trait on a type: 

```
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

After we define the `summarize_author` method, we can call on the `summarize` function on instances of the `Tweet` struct, and the default implementation of the `summarize` function will call the definition of the `summarize_author` method. Because we’ve implemented `summarize_author`, the `Summary` trait has given us the behavior of the `summarize` method without requiring us to write any more code.

```

```

The code print out `1 new tweet: (Read more from @horse_ebooks...)`. 

<ins>Note: This isn't possible to call the default implementation from an overriding implementation of the same method.</ins>


## Traits as Parameters

We can use traits to define functions that accept many different types. 

Example: We implemented the `Summary` trait on the `NewsArticle` and `Tweet` types. 

We can define a `notify` function that calls the `summarize` method on its `item` parameter, which is of some type that implements the `Summary` trait. To do this, we can use the `impl Trait` syntax like below: 

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait. 

In the `notify` body, we can call any methods on `item` that come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. 

Code that calls the function with any other type, such as a `String` or an `i32`, won't compile because those types don't implement `Summary`. 


### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but it's actually syntactical sugar for a longer form, which is called a **trait bound**: 

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form works the same way but it's more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside single brackets. 


The `impl Trait` syntax is convenient and makes for concise code in simple cases. The trait bound syntax can express more complexity in other cases. 

Example: We can have 2 parameters that implement `Summary`. 

Using the `impl Trait` looks like this: 

```
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```

If we wanted to function allow for `item1` and `item2` to have different types, using `impl Trait` would be appropriate (as long as both types implement `Summary`). 

If we want to force both parameters to have the same type, that's only possible to express using a trait bound: 

```
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

The generic type `T` specified as the type of the `item1` and `item2` parameters constrains the function such that the concrete type of the value passed as an argument for `item1` and `item2` must be the same. 


### Specifying Multiple Trait Bounds with the `+` Syntax

We can also specify more than one trait bound. 

Example: We want `notify` to use display formatting on `item` as well as the `summarize` method: We specify in the `notify` definition that `item` must implement both `Display` and `Summary`. 

We can specify more than one trait bound using the `+` syntax: 

```
pub fn notify(item: &(impl Summary + Display)) {}
```

The `+` syntax is also valid with trait bounds on generic types: 

```
pub fn notify<T: Summary + Display>(item: &T) {}
```

With the 2 trait bounds specified, the body of `notify` can call `summarize` and use `{}` to format `item`.


### Clearer Trait Bound with `where` Clauses

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function's name and its parameter list, making the function signature hard to read. 
For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature. 

Instead of writing this: 

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

We can use a `where` clause to better explain multiple trait bounds: 

```
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug 
{}
```

The function's signature is also less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds. 


## Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait, such as the code below: 

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

By using `impl Summary` for the return type, we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type. 

The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators.

Closures and iterators create types that only the compiler knows or types that are very long to specify. The `impl Trait` syntax lets you concisely specify that a function returns some type that implements the `Iterator` trait without needing to write out a very long type.

The caveat is that we can only use `impl Trait` if we're returning a single type. 

Example: This code that returns either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary` wouldn't work. 

```
fn return_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

Returning a `NewsArticle` or `Tweet` isn't allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler.


## Fixing the `largest` Function with Trait Bounds

Traits allow us specify the behavior that we want to use using the generic type parameter's bounds. 

Example: Going back to this example, a defintion of the `largest` function that uses generic type parameters but doesn't compile yet

```
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

The code won't compile because we want to compare types of value `T` in the body, we can only use types whose values can be ordered. 
If we want to enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that we implement on types. 

In the body of `largest`, we wanted to compare 2 values of type `T` using the greater than (`>`) operator. Because that operator is defined as a default method on the standard library trait `std::cmp::PartialOrd`, we need to specify the `PartialOrd` in the trait bounds for `T` so the `largest` function can work on slices of any type that can compare. We don't need to bring `PartialOrd` into scope because it's in the prelude. 

Change the signature of `largest` to look like this: 

```
fn largest<T: PartialOrd>(list: &[T]) => T {}
```

THe new error here is `cannot move out of type [T], a non-copy slice`. With the non-generic versions of the `largest` function, we were tryign to find the largest `i32` or `char`. Types like `i32` and `char` have a known size that's stored on the stack, so they implement the `Copy` trait. When we have made the `largest` function generic, it became possible for the `list` parameter to have types in it that don't implement the `Copy` trait. This results in us not being able to move the value out of `list[0]` and into the `largest` variable, resulting in an error. 

To call this code with only the types that implement the `Copy` trait, we can add `Copy` to the trait bounds of `T`! 

Example: A working definition of the `largest` function that works on any generic type that implements the `PartialOrd` and `Copy` traits. 

```
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!["y", "m", "a", "q"];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

The code will compile now that we added a trait. 

If we don't want to restrict the `largest` function to the types that implement the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead of `Copy`. 
Then we could clone each value in the slice when we want the `largest` function to have ownership. Using the `clone` function means we're potentially making more heap allocations in the case of types that own heap data like `String`, and heap allocations can be slow if we're working with large amounts of data. 

Another way we could implement `largest` is for the function to return a reference to a `T` value in the slice. If we change the return type to `&T` instead of `T`, thereby changing the body of the function to return a reference, we wouldn't need the `Clone` or `Copy` trait bounds and we could avoid heap allocations. 


## Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. 

Example: Conditionally implement methods on a generic type depending on trait bounds

```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_
}
```

The type `Pair<T>` always implement the `new` function to return a new instance of `Pair<T>` (<ins>Note: Recall that `Self` is a type alias for the type of the `impl` block, which in this case is `Pair<T>`</ins>). 
In the next `impl` block, `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd` trait that enabled comparison **and** the `Display` trait that enables printing. 

We can also conditionally implement a trait for any type that implements another trait.
Implementations of a trait on any type that satisfies the trait bounds are called **blanket implementations** and are extensively used in the Rust standard library. 

Example: The standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code: 

```
impl<T: Display> ToString for T {
    ...
}
```

Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait. 

Example: We can turn integers into their corresponding `String` values like this because integers implement `Display`

```
let s = 3.to_string();
```

We can also conditionally implement a trait for any type that implements another trait. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.

In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

Another kind of generic is called **lifetimes**. Rather than ensuring that a type has the behavior that we want, lifetimes ensure that references are valid as long as we need them to be. 