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

The function bodies have the same code, we can eliminate the duplication by introducing a generic type parameter in a single function. 

In order to parameterize the types in the new function that we'll create, we need to name the type parameter, similar to what we do for the value parameters to a function. 
We can use any identifier as a type parameter name. 

`T` is usually used, by convention, since parameter names in Rust are short. It's often just a letter and used with Rust's type-naming convention in CamelCase. Short for "type", `T` is the default choice for most Rust programmers. 

When we use a parameter in the body of a function, we have to declare the parameter name in the signature so the compiler knows what that name means. On a similar note, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it. To define a generic in a function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameters, like this: 

```
fn largest<T>(list: &[T]) -> T {
    ....
}
```

The `largest` function has a generic over some type `T`. This function has one parameter named `list`, which is a slice of values of type `T`. The `largest` function will return a type of value of the same type `T`. 


Example: A defintion of the `largest` function that uses generic type parameters but doesn't compile yet

```
fn largest<T>(list: &[T]) -> T {
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

The code won't compile because we want to compare types of value `T` in the body, we can only use types whose values can be ordered. 
If we want to enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that we implement on types. 


## In Struct Definitions

We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax. 

Example: A `Point<T>` struct that holds `x` and `y` values of type `T` 

```
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

The syntax for using generics in struct definitions is similar to that used in function definitions. First, we declare the name of the type parameter inside angle brackets just after the name of the struct. Then we can use the generic type in the struct definition where we would otherwise specify concrete data types.

<ins>Note: Because we've only used one generic type to define `Point<T>`, the definition says that the `Point<T>` struct is generic over some type `T`, and the fields `x` and `y` are **both** that same type, whatever that type may be.</ins> 

If we create an instance of a `Point<T>` that has values of different types, our code won't compile. 

Example: The fields `x` and `y` must be the same type because both have the same generic data type `T`. 

```
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

In the example above, when we assign the integer value 5 to `x`, we let the compiler know that generic type `T` will be an integer for this instance of `Point<T>`. When we pass 4.0 for `y`, we will get a type mismatch error. 

If we want to define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters. 

We can change the defintion of `Point` to be generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`. 

Example: A `Point<T, U>` generic over 2 types so that `x` and `y` can be values of different types  

```
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point {x: 5, y: 10 };
    let both_float = Point {x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

All of the instances of `Point` shown are allowed! We can use an many generic type parameters in a definition as we want. 

When we need a lot of generic types in our code, this could be indicative of the fact that our code needs restructuring into smaller pieces. 


## In Enum definitions

We can also define enums to hold generic data types in their variants. `Option<T>` is an enum provided by the standard library. 

```
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` is an enum that's generic over type `T` and has 2 variants: `Some`, which holds one value of type `T`, and a `None` variant that doesn't hold any value. By using the `Option<T>` enum, we can express the abstract concept of having an optional value, and because `Option<T>` is generic, we can use this abstraction no matter what the type of the optional value is. 

Enums can also use multiple generic types as well. The definition of the `Result` enum is one example. 

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over 2 types: `T` and `E`, and 2 variants: `Ok`, which holds a value of type `T`, and `Err`, which holds a value of type `E`. This definition makes it convenient to use the `Result` enum anywhere we have an operation that might succeed (return a value of some type `T`) or fail (return an error of some type `E`). 

Example use case: When we open a file using `std::fs::File`, `T` was filled in with the type `std::fs::File` when the file was opened successfully and `E` was filled in with the type `std::io::Error` when there were problems opening the file. 

When we recognize situations in which our code with multiple struct or enum definitions that differ only in the type of the values they hold, we can avoid duplication by using generic types instead. 


## In Method Definitions

We can use generic types in methods as well. 

Example: Implementing a method named `x` on the `Point<T>` struct that will return a reference to the `x` field of type `T`

```
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

In the example above, we created a method named `x` on `Point<T>` that returns a reference to the data in the field `x`. 

<ins>Note: We have to declare `T` just after `impl` so we can use it to specify that we're implementing methods on type `Point<T>`.</ins> 
By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type. 

Because we are declaring the generic again, we could have chosen a different name for the generic parameter than the generic parameter declated in the struct definition, but using the same name is conventional. 
Methods written within an `impl` that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up subsituting for the generic type. 

The other option we have is defining methods on the type with some constraint on the generic type. 

Example: Implementing methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type. 

We can implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type. 

```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

The code above means that the type `Point<f32>` will have a method named `distance_from_origin` and other instances of `Point<T>` where `T` is not of type `f32` will not have this method defined. 

Generic type parameters in a struct definition aren't always the same as those we use in that struct's method signature. 

Example: A method that uses different generic types from its struct's definition

```
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: "c" };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

The code above uses the generic types `X1` and `Y1` for the `Point` struct and `X2` `Y2` for the `mixup` method signature. The method creates a new `Point` instance with the `x` value from the `self` `Point` (of type `X1`) and the `y` value from the passed-in `Point` (of type `Y2`).

This example shows how some generic parameters are declared with `impl` and some are declared with the method definition. The generic parameters `X1` and `Y1` are declared after `impl` because they go with the struct definition. The generic parameters `X2` and `Y2` are declared after `fn mixup`, because they're only relevant to the method. 


## Performance of Code Using Generics

Rust can maintain optimal performance with generics using a process called monomorphization. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. 

In the process, the compiler does the opposite of the steps we used to create the generic function; the compiler looks at all of the places where generic code is called and generates code for the concrete types the generic code is called with. 

Example: Looking at generics compile using the std library `Option<T>` enum: 

```
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization. During this process, the compiler reads the values that have been used in `Option<T>` isntances and identifies 2 kinds of `Option<T>`: 

1. `i32`
2. `f64`

With both of these examples, the compiler expands the generic definition of `Option<T>` into `Option_i32` and `Option_f64`, thereby replacing the generic definition with the specifc ones. 

The monomorphized version of the code looks like the following: The generic `Option<T>` is replaced with the specific definitions created by the compiler. 

```
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None, 
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Because Rust compiles generic code into code that specifies the type in each instance, we don't pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.