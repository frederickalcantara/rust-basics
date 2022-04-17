// The Option type encodes the very common scenario in which a value could be something or it could be nothing.

// Example If you request the first of a list containing items, you would get a value. If you request the first item of an empty list, you would get nothing.
// This concept helps in preventing bugs that are common in other programming languages.


fn main() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_numer: Option<i32> = None;

}