# Separating Modules into Different Files

When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

We can change modules from the crate root file to a separate file. 

This procedure also works with binary crates whose crate root file is src/main.rs.

Example File: `src/lib.rs`

```
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

We can add a file named `src/front_of_house.rs` in which we can get the definition from teh body of the `front_of_house` module. 

Example File: `src/front_of_house.rs` 

```
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

Using a semicolon after mod `front_of_house` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.

We can also extract the `hosting` module to its own file as well. 

File: `src/front_of_house.rs` 

```
pub mod hosting;
```

We create a `src/front_of_house` directory and a file `src/front_of_house/hosting.rs` to contain the definitions made in the hosting module:

File: `src/front_of_house/hosting.rs`

```
pub fn add_to_waitlist() {}
```

The module tree remains the same, and the function calls `eat_at_restaurant` will still work without any modification, even thought eh definitions live in different files. We can move to new files as they grow in size. 

The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.


## Summary

Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the `pub` keyword.