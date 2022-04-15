#[derive(Debug)]
// Adding #[derive(Debug)] allows you to print debugging information for structs 

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    
    // To print a standard output, use {:?}
    
    // To pretty print a debug output, use {:#?}

    dbg!(&rect1);
    // dbg! macro prints to the standard error console stream (stderr), as opposed to println! which prints to the standard output console stream (stdout) 
}