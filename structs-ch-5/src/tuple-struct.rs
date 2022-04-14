struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); 

    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; 
    // rather, they just have the types of the fields. 
    
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, 
    // and when naming each field as in a regular struct would be verbose or redundant.

    // tuple struct instances behave like tuples: you can destructure them into their individual pieces, 
    // you can use a . followed by the index to access an individual value, and so on.
}