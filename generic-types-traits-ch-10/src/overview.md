# Generic Types, Traits, and Lifetimes 

**Generics** are abstract stand-ins for concrete types or other properties. 

<ins>Note: We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running code.</ins> 

Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values, functions can take parameters of some generic type instead of a concrete type, like `i32` or `String`. 

**Traits**: Ways in which we can define behavior in a generic way. We can combine traits with generic types to constrain a generic type to only those types that have a specific behavior, as opposed to any type. 

**Lifetimes**: A variety of generics that give the compiler information about how references relate to each other. Lifetimes allows us to borrow values in many situations while still enabling the compiler to check that the references are valid. 

## Removing Duplication by Extracting a Function

Example: Short program that finds the largest number in a list

File `src/main.rs`

```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

The code stores a list of integers in the variable `number_list` and places the first number in the list in a variable named `largest`. It will go through the list, do a comparison, and print out the largest number. 


Example: Find the largest number in 2 different lists of numbers. 

File `src/main.rs`

```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

This code work, but is redundant and error prone. 

To eliminate this duplication, we can create an abstraction by defining a function that operates on any list of integers given to it in a parameter. 


Example: Abstracted code to find the largest number of 2 lists

File `src/main.rs`

```
fn largest(list: &[i32]) -> i32 {
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

    let result = largest(&number_list);
    println!("The largest number is {}". result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}". result);
}
```

The `largest` function has a parameter called `list`, which represents any concrete slice of `i32` values that we might pass into the function. 

When we call the function, the code runs on the specific values that we pass in. 