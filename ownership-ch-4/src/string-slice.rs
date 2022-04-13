fn hello_world() {
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{hello}");
    // println!("{world}");
}

fn main() {
    let s = String::from("hello");
    let len = s.len();


    // Both do the same thing, if we want to start at index zero, we can use 0 .. 2 or drop the value before 2 periods. 
    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number
    let slice = &s[3..len];
    let slice = &s[3..];

    // We can also drop both values to take a slice of the entire string. 
    let slice = &s[0..len];
    let slice = &s[..];

    // Important Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. 
}