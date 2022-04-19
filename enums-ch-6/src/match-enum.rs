// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 
// Patterns can be made up of literal values, variable names, wildcards, and many other things.

// The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

#[derive(Debug)] // allows us to inspect the state
enum UsState {
    Arizona,
    California,
    Colorado,
    Florida,
    Tennessee,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.
// This is where the option enum is useful with the usage of match
// Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.

fn main() {
    value_in_cents(Coin::Quarter(UsState::Florida));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// With a match keyword, there are match arms. An arm has 2 parts: a patterm and some code.  
// The first arm here has a pattern that is the value Coin::Penny
// The code is the code to be ran. If you want to run multiple lines of code in a match arm, you must use curly brackets.
// Each arm is separated from the next with a comma

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. 
// This is how we can extract values out of enum variants.
