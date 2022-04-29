# Generic Type Parameters, Trait Bounds, and Lifetimes Together

Example: Generic type parameters, trait bounds, and lifetimes in one function

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

The `longest` function now has an extra parameter named `ann` of the generic type `T`, which can be filled in by any type that implements the `Display` trait as specified by the `where` clause. 
This extra parameter will be printed using `{}`, which is why the `Display` trait bound is necessary. 
Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name. 


# Summary 

Generic type parameters let us apply the code to different types. 

Traits and trait bounds ensure that even though the types are generic, they'll have the behavior the code needs. 

Lifetime annotations allow flexible code to not have any dangling references. 

All of this analysis happens at compile time, none of which affects runtime performance. 


