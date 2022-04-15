#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
    // All functions defined in the impl block are called association functions, 
    // because they're associated with the type named after the impl. 
    
    // We can define associated functions that don’t have self as their first parameter (and thus are not methods) 
    // because they don’t need an instance of the type to work with.

    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
}
fn main() {
    sq_size = Rectangle::square(3);

    println!("Square size: {}", sq_size);
}
