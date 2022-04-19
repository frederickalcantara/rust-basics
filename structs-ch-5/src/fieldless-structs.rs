struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

// You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself