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

