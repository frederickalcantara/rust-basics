fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),

        _ => (),
        // This example also meets the exhaustive requirement because we’re explicitly ignoring all other values in the last arm; we haven’t forgotten anything.
        // We can also express that by using the unit value (the empty tuple type)
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

// Using enums, we can also take special actions for a few particular values, but for all other values take one default action

// We can use catch all patterns to account for matches to be exhaustive. 
// Rust also has a pattern we can use when we don’t want to use the value in the catch-all pattern: _, 
// which is a special pattern that matches any value and does not bind to that value. 
// This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.