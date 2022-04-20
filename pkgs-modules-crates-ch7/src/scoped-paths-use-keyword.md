# Bringing Paths into Scope with the `use` Keyword

We can shorten paths by calling the parent path once instead of including the absolute or relative path each time. 

We can bring a path into a scope once and then call the items in that path as if they’re local items with the `use` keyword.

File: `src/lib.rs`

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
Idiomatic way to bring a function into scope with `use`

Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope, just as though the `hosting` module had been defined in the crate root. Paths brought into scope with `use` also check privacy, like any other paths.

We can also bring an item into scope with `use` and a relative path with the `self` keyword.

File: `src/lib.rs`

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

## Idiomatic `use` Paths

Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. 

On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path. 

File: `src/main.rs`

```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

There's no real reason outside of the fact that ppl just do it this way. 

The **exception** to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that. 

File: `src/lib.rs`

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

Using the parent modules distinguishes the two Result types. If instead we specified use std::fmt::Result and use std::io::Result, we’d have two Result types in the same scope and Rust wouldn’t know which one we meant when we used Result.

## Providing New Names with the `as` Keyword

There’s another solution to the problem of bringing two types of the same name into the same scope with use: after the path, we can specify as and a new local name, or alias, for the type.

File: `src/lib.rs`

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

In the second `use` statement, we chose the alias name `IoResult` for the `std::io::Result` type, which won’t conflict with the Result from std::fmt that we’ve also brought into scope. 

## Re-exporting Names with `pub` `use`

When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.

This technique is called **re-exporting** because we’re bringing an item into scope but also making that item available for others to bring into their scope.

File: `src/lib.rs`

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

By using `pub use`, external code can now call on the `add_to_waitlist` function using `hosting::add_to_waitlist()`. 

If we hadn't specified `pub use`, the `eat_at_restaurant` function could call `hosting::add_to_waitlist` in its scope, but external code couldn’t take advantage of this new path.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. 

## Using External Packages

To use an external package, we simply add the dependency name and version in the **Cargo.toml** file. 

File: `Cargo.toml` 

```
rand = "0.8.3"
```

Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.

When we need to bring `rand` definitions into the scope of our package, we add the `use` line staring with the name of the crate. 

Example: Calling the Rng trait into scope to call

```
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

The standard library (std) is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std. However, we do need to refer to it with use to bring items from there into our package’s scope. 

Example, with HashMap we would use this line:

```
use std::collections::HashMap;
```

This is an absolute path starting with std, the name of the standard library crate.

## Using Nested Paths to Clean Up Large `use` Lists

If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files.

To reduce repetition, we can use nested paths to being same items into scope in one line. 

We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ.


Example File: `src/main.rs` with nested paths

```
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

Example File: `src/main.rs` without nested paths

```
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

In bigger programs, bringing many items into scope from the same crate or module using nested paths can reduce the number of separate use statements needed by a lot!

We can use a nested path at any level in a path, which is useful when combining two `use` statements that share a subpath.

Example File: `src/lib.rs`

```
use std::io;
use std::io::Write;
```

The common part of these two paths is std::io, and that’s the complete first path. To merge these two paths into one use statement, we can use self in the nested path.

Example File: `src/lib.rs`

```
use std::io::{self, Write};
```

The line above bring `std::io` and `std::io:Write` into scope. 

## The Glob Operator

If we want to bring all public items defined in a path into scope, we can specify that path followed by *, the glob operator:

```
use std::collections::*;
```

This use statement brings all public items defined in std::collections into the current scope.

<ins>Note: Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.</ins>