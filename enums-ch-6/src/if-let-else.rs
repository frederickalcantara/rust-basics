// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

fn main() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    // The if let syntax allows us to explain the above in a concise way

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. 
    // Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

    // We can include an else with an if let.
    // The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else.

    // Original example
    
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // Concise if let and else example
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}