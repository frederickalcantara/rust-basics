#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for self: &Self, in which the type Self is an alias for the type that the impl block is for.

    // Methods must have a parameter named self of type Self for their first parameter, 
    // so Rust lets you abbreviate this with only the name self in the first parameter spot.

    // Note: We still need to use the & in front of the self shorthand to indicate this method borrows the Self instance.

    // Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Rust automatically adds in &, &mut, or * so object matches the signature of the method
    // The following are the same:

        // p1.distance(&p2);
        // (&p1).distance(&p2);

}
// The impl block (implementation) block for Rectangle. 
// Everything in the impl block will be associated with the Rectangle struct.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// Methods are similar to functions: we declare them with the fn keyword and a name, 
// they can have parameters and a return value, 
// and they contain some code that’s run when the method is called from somewhere else. 

// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object) and their first parameter is always self
// which represents the instance of the struct the method is being called on. 

// The main reason for using methods instead of functions, 
// in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization.