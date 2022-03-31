fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. 
  // Danger! Danger!


// In languages with pointers, it’s easy to erroneously create a dangling pointer.
// A dangling pointer references a location in memory that may have been given 
// to someone else, which frees some memory while preserving a pointer to that memory



// To fix the dangler pointer problem, we can simply return the string directly.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}