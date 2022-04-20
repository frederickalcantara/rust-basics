# Paths for item reference in the module tree

To show rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

A patch can take 2 forms: 

- An **absolute** path starts from a create by using a crate name or a literal `crate`
- A **relative** path starts from the current module and uses `self`, `super`, or an identifier in the current module

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

How do we call a function inside a module? 

Example: Calling the add_to_waitlist function (refer to code from lib.rs)

`src/lib.rs` file

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The code in the example won't compile because it says that the module `hosting` is private. The path is correct, but Rust won't let us use them because it doesn't have access to the private sections. 

Choosing whether to use a relative or absolute path is a decision you’ll make based on your project. The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item.

Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.

Module not only organize code, they also define Rust's **privacy boundary**: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.

The way privacy works in Rust is that all of the items (functions, methods, structs, enums, modules, and constants) are private by default. 

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. 

The reason is that **child modules wrap and hide their implementation details**, but the **child modules can see the context in which they’re defined.**

We can expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.

## Exposing Paths Publicly with the `pub` Keyword

Example: Calling the add_to_waitlist function (refer to code from lib.rs) with the hosting module now public

`src/lib.rs` file

```
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The code still won't compile because the module `hosting` is public but not the child function `add_to_waitlist`. Parent modules cannot access the contents of their children modules. However, in this case the child function `add_to_waitlist` can access the contents of the parent module `hosting`. 

The `pub` keyword on a module only lets code in its ancestor modules refer to it. 
The privacy rule also applies to structs, enums, functions, and methods as well as modules.

Example: Calling the `add_to_waitlist` function with the function also being publicly called

`src/lib.rs` file

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The code compiles because we made the add_to_waitlist function public by using the `pub` keyword. 

**Modules can access content from a sibling module**


## Starting Relative Paths with `super`

We can also construct relative paths that begin in the parent module by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax.

Example: Calling a function using `super`

`src/lib.rs` file

```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        
        cook_order();
        // Acesses sibling function relative to the fix_incorrect_order function

        super::serve_order();
        // See explanation below
    }

    fn cook_order() {}
}
```

How does this work exactly? The `fix_incorrect_order` **function** is in the `back_of_house` **module**, so we can use `super` to go to the parent **module** of `back_of_house`, which in this case is crate, the root. From there, we look for serve_order and find it.

## Making Structs and Enums Public

We can also use pub to designate structs and enums as public. 
If we use pub before a struct definition, we make the struct public, but the structs fields will still be private. 
We can make each field public or not on a case-by-case basis.

Example: Access some public fields but unable to access private fields

`src/lib.rs` file

```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

The `toast` field in the struct is public. In the `eat_at_restaurant` function, we can read and write to the `toast` field using the dot notation. We don't be able to use the `seasonal_fruit` field because it's private. 

Take note that the Breakfast has a private field, the struct needs to provide a public function that constructs an instance of `Breakfast` (the instance is named `summer` here). If `Breakfast` didn’t have such a function, we couldn’t create an instance of `Breakfast` in `eat_at_restaurant` because we couldn’t set the value of the private `seasonal_fruit` **field** in `eat_at_restaurant`.

In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.

Example: Public enum

`src/lib.rs` file Example

```
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant. 

Enums aren’t very useful unless their variants are public; enums are set to public by default.

Structs are useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with `pub`. 

