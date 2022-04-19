fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    let strings = some_string.push_str(", world");
}

// You can't have a multiple immutable references and then a mutable reference
// Using multiple mutable references will lead to data races
// Using multiple immutable references is perfectly fine

// Users of an immutable reference don’t expect the value to suddenly change 
// However, multiple immutable references are allowed because no one who is just 
// reading the data has the ability to affect anyone else’s reading of the data.

// Note that a reference’s scope starts from where it is introduced 
// and continues through the last time that reference is used