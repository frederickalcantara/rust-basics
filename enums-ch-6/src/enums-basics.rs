enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
// Using an enum is a way more concise to display the information. 
// There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
// There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. 

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: four,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: six,
    //     address: String::from("::1"),
    // };

    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    // The write method is being used here
    m.call();
}

fn route(ip_kind: IpAddrKind) {}

// Enums are a way of defining custom data types in a different way than you do with structs
// An enum value can only be one of its variants