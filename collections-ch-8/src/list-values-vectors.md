# Storing Lists of Values with Vectors

Vectors allow us to store more than one value in a single data structure that puts all the values next to each other in memory. It's a dynamic array. 

Vectors store values of the same type. They're useful when we have a list of items, such as the lines of text in a file or the prices of items in a shopping cart. 

## Creating a New Vector

Example: Creating a new vector 

```
let v: Vec<i32> = Vec::new();
```

<ins>Note: We added a type annotation here</ins>

Since we aren't inserting any values into this vector, Rust doesn't know what kind of elements we intend to store. 

Vectors are implemented using generics. The `Vec<T>` type provided by the standard can hold any type. 

Most of them time, we can create a `Vec<T>` with initial values and Rust can imply the value type that we want to store. 

Rust also provides the `vec!` macro, which creates a new vector that holds the values that we give it. 


Example: Vector with the `vec!` macro

```
let v = vec![1, 2, 3];
```

In the example above, because we gave it `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and that the type annotation isn't necessary. 

## Updating a Vector

We can create a vector and add elements to it using the `push` method. 

Example: Updating a vector 

```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Like any variable, in order to change the value, we need to make it mutable using the `mut` keyword. 
The numbers that we placed inside it were all of type `i32`, and Rust infers this from the data, so we don't need the `Vec<i32>` annotation. 

## Dropping a Vector Drops its Elements

As with any other `struct`, a vector is freed when it goes out of scope. 

```
{
    let v = vec![1, 2, 3, 4];

    // Do stuff with v
} // <- v goes out of scope and is freed here
```

When the vector gets dropped, all of its contents are also dropped, in which the integers it holds will be cleaned up. 

## Reading Elements of Vectors

There are 2 ways to reference a value stored in a vector: 

1. Indexing
2. Using the `get` method

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element.);
}
```

2 Points: 

1. Vectors start indexing by the number, starting at 0.
2. We can get N element by either using `&` and `[]`, which gives us a reference, or using the `get` method with the index passed as an argument, which gives us an `Option<&T>`

The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements.

Example: Index out of range

```
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

Using the `[]` approach will cause the program to panic because it references a nonexistent element. This method is best used when we want our program to crash if there's an attempt to access an element pass the end of the vector. 

When the `get` method is passed an index that's outside the vector, it returns `None` without panicking. We would use this method if accessing an element beyond the range of the vector may happen oceassionally under normal circumstances. The code would then have logic to handle having either `Some(&element)` or `None`. 

Error Example: The index could be coming from a person entering a number. 
If they accidentally enter a number that’s too large and the program gets a None value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value. 


When a program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid. 
We can't have mutable and immutable references in the same scope. 

Example: Mutable and immutable reference in the same scope

```
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

This code won't compile, why is that? 

This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## Iterating over the Values in a Vector

To access each element in a vector, we would iterate through all of the elements rather than use indices to access one at a time. 

Example: Using a `for` loop to get immutable references to each element in a vector of `i32` values and print them. 

```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

We can also iterate over mutable references to each element in a mutable vector in order make changes to all of the elements. 

Example: Using a `for` loop to add 50 to each element. 

```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

To change the value that the mutable reference refers to, we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator. 

## Using an Enum to Store Multiple Types 

Vectors can only store values that are the same type. Inconvenient, however there are use cases for needing to store a list of items of different types. 

The variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types. 

Example: We want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings.

We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum.

Example: Defining an `enum` to store values of different types in one vector

```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
]
```

Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. 
We must also be explicit about what types are allowed in the vector. 
If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. 
Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.

If we don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work.

<ins>Note: In addition to `push`, a `pop` method removes and returns the last element.</ins>