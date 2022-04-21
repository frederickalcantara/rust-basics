use std::collections::HashMap;

fn main() {

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    println!("{}", word);
    let count = map.entry(word).or_insert(0);
    println!("Before Count: {}", count);
    *count += 1;
    println!("After Count: {}", count);
}

println!("{:?}", map);
}
