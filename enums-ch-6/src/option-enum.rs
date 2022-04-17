// The Option type encodes the very common scenario in which a value could be something or it could be nothing.

// Example If you request the first of a list containing items, you would get a value. If you request the first item of an empty list, you would get nothing.
// This concept helps in preventing bugs that are common in other programming languages.


fn main() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_numer: Option<i32> = None;

}

// Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code. 
// In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. 
// Then, when you use that value, you are required to explicitly handle the case when the value is null. 
// Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. 
// This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.