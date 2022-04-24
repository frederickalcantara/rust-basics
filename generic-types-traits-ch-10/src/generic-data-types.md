# Generic Data Types

We can use generics to create definitions for items like function signatures or structs, which we can use with many different concrete data types. 

## In Functions Definitions

When defining a function that uses generics, we place the generics in the signature of the function where we would specify the data types of the parameters and return value. 

Example: 2 Functions that differ only in their names and the types in their signatures 

```
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

The function bodies have the same code, 