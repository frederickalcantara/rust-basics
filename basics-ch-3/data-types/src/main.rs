use std::io;

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    // chars
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuples example
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Another way to get tuple values
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Useful notes about arrays and vectors

    // Arrays must have the same type and a fixed size
    // Arrays are useful when you want your data allocated on the stack rather than the heap 
    // or when you want to ensure you always have a fixed number of elements. 
    // An array isn’t as flexible as the vector type, though. 
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. 
    // If you’re unsure whether to use an array or a vector, chances are you should use a vector. 
    
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // We can also declare the type and size of an array in one line
    let arr: [i32; 5] = [1, 2, 3, 4, 5];


    // We can create with a fixed and the same value 
    let concise_arr = [3; 5];
    // let a = [3, 3, 3, 3, 3];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
