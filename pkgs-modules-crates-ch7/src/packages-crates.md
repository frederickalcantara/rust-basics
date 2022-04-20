# Packages & Crates

Crates are binaries or libraries. The crate root is a source file that the Rust compiler starts from and makes up the root module of the crate. 

A package is one or more crates that provides a set of functionalities. A package provides a Cargo.toml file that describes how to build those crates. 
Several rules determine what a package can contain. A package can contain at most one library crate. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).


## Package Creation Process

We create a package with `cargo new new-project`

1. Cargo created a Cargo.toml file, giving us a package.
    - Looking at the contents of Cargo.toml, there’s no mention of `src/main.rs` because Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.
    - Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.
2. Cargo will pass the crate root file to `rustc` to build the library or binary. 

If a package contains `src/lib.rs` and `src/main.rs`, then it will 2 crates: a binary and a library, both with the same name as the package. 

A package can have multiple binary crates by packing files in the `src/bin` directory: Each file will be a separate binary crate. 

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects. 
We can use that functionality in our own projects by bringing the rand crate into our project’s scope. All the functionality provided by the rand crate is accessible through the crate’s name, rand.

Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the rand crate and prevents potential conflicts. For example, the rand crate provides a trait named Rng. We can also define a struct named Rng in our own crate. Because a crate’s functionality is namespaced in its own scope, when we add rand as a dependency, the compiler isn’t confused about what the name Rng refers to. In our crate, it refers to the struct Rng that we defined. We would access the Rng trait from the rand crate as rand::Rng.