# Validating References with Lifetimes

Every reference in Rust has a **lifetime**, which is the scope for which the reference is valid. 

Most of the time, lifetimes are implicit and inferred, similar to how types are inferred. 

We must annotate types when multiple types are possible. 
Similarly, we must annotate lifetimes when the lifetime of references could be related in different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will be valid. 


## Preventing Dangling References with Lifetimes

The main goal of lifetimes is to prevent dangling references, which causes a program to reference data other than the data it's intended to reference. 

Example: An attempt to use a reference whose value has gone out of scope

```
    {
        let r;

        { 
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
```

<ins>Note: Variables will be declared without an initial value, in which the variable name exists in the outer scope. Initially, this might appear to be in conflict with Rust's having no null values. However, if we try to use a variable before giving it a value, we'll get a compile-time error, which shows that Rust indeed doesn't allow null values.</ins>

The outer scope declares a variable named `r` with no initial value, and the inner scope declares a variable named `x` with the initial value of 5. Inside the inner scope, we attempt to set the value of `r` as a reference of `x`. Then the inner scope ends, and we attempt to print the value in `r`. 

This code won't compile because the value `r` is referring to has gone out of scope before we try to use. 
The error message that we get is a compile-time error. 

The variable `x` doesn't "live long enough". The reason is that `x` will be out of scope when the inner scope ends. `r` is still valid for the outer scope; because its scope is larger, we that it "lives longer". 
If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to with `r` wouldn't work correctly. 

How does Rust determine that code is invalid? It uses a borrow checker. 

## The Borrow Checker

The rust compiler has a **borrow checker** that compares scopes to determine whether all borrows are valid. 

Example: Same code as above but with annotations showing the lifetimes of the variables

```
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
```

The lifetime of each variable is annotated with `'a` for `r` and `'b` for `x`. The `'b` block is shorter than the `'a` block. 

At compile time, Rust compares the size of the 2 lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with a lifetime of `'b`. The programs rejects the runtime because `'b` is shorter than `'a`: The subject of the reference doesn't live as long as the reference. 


Example: Fixing the code so that it doesn't have a dangling reference and compiles without any errors

```
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
```

The code compiles because `x` has the lifetime `'b`, which in this case is larger than `'a`. This means that `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid. 


## Generic Lifetimes in Functions

Example: A function will return the longest of 2 strings and return a string slice. 

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

The function will take string slices, which are references, because we don't want the `longest` function to take ownership of its parameters. 


Example: `longest` Function 

```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The code won't compile, when we run the error we will get a text stating that the return type needs a generic lifetime parameter on it because Rust can't tell whether the reference being returned refers to `x` or `y`. 

When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the `if` case or the `else` case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did to determine whether the reference we return will always be valid. 

The borrow checker can't determine this either, because it doesn't know how the lifetimes of `x` and `y` relate to the lifetime of the return value. To fix this, we'll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis. 


## Lifetime Annotation Syntax

Lifetime annotations don't change how long any of the references live. 

In the same way that functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. 

Lifetime annotations decribe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. 

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types. Most people use the name `'a`. We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference's type. 

Examples: 

- A reference to `i32` without a lifetime parameter 
- A reference to `i32` that has the lifetime parameter named `'a` 
- A mutable reference to an `i32` that also has the lifetime `'a`

```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. 

Example: There's a function with the parameter `first` that's a reference to an `i32` with lifetime `'a`. The function also has another parameter named `second` that's another reference to an `i32` that also has the lifetime `'a`. 
The lifetime annotations indicate that the references `first` and `second` must both live as long as that generic lifetime. 


## Lifetime Annotations in Function Signatures

As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list. 
The constraint we want to express in this signature is that the lifetime of both of the parameters and the lifetime of the returned reference are related such that the returned reference will be valid as long as both the parameters are. 

Example: The `longest` function definition specifying that all of the references in the signature must have the same lifetime `'a` 

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code will work since we gave all of the references the same lifetime `'a`. 

The function signature now tells Rust that for some lifetime `'a`, the function takes 2 parameters, both of which are string slices that live at least as long as lifetime `'a`. 
The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`. In practice, this means that the lifetime of the reference returned by a function is the same as the smaller of the lifetimes of the references passed in. 

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. The lifetime annotations become part of the contract of the function, much like the types in the signature are.
Having function signatures contain the lifetime contract means the analysis the Rust compiler does can be simpler. If there’s a problem with the way a function is annotated or the way it is called, the compiler errors can point to the part of our code and the constraints more precisely. 
Instead, the Rust compiler made more inferences about what we intended the relationships of the lifetimes to be, the compiler might only be able to point to a use of our code many steps away from the cause of the problem.

Example: Using the `longest` function with references to `String` values that have differnet concrete lifetimes

```
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println("The longest string is {}", result);
    }
}
```

In this example, `string1` is valid until the end of the outer scope, `string2` is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves of this code; it will compile and print `The longest string is long string is long`.

Example: Code where the lifetime of the reference in `result` must be the smaller lifetime of the 2 arguments.

We'll move the declaration of the `result` variable outside the inner scope but leave the assignment of the value to the `result` variable inside the scope with `string2`. Then we'll move the `println!` that uses `result` outside the inner scope, after the inner scope has ended

```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

When we compile the code, it won't work because `string2` needs to be valid until the end of the outer scope. In this case, it isn't
Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter of `'a`.


## Thinking in Terms of Lifetimes

The way in which we need to specify lifetime parameters depends on what our function is doing. 

Example: If we changed the implementation of the `longest` function to always return the first parameter rather than the longest string slice, we wouldn't need to specify a lifetime on the `y` parameter. 

The code below compiles: 

```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

In the example above, we've specified a lifetime parameter `'a` for the parameter `x` and the return type, but not for the parameter `y`, because the lifetime of `y` doesn't have any relationship with the lifetime of `x` or the return value. 

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference return **doesn't** refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function. 

Example: Compiling code when the reference return doesn't refer to one of the function parameters.

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Code that won't compile because the return value lifetime isn't related to the lifetime of the parameters at all. 

The problem is that `result` goes out of scope and gets cleaned up at the end of the `longest` function. We're also trying to return a reference to `result` from the function. 
There's no way we can specify lifetime parameters that would change the dangling reference, and Rust won't let us create a dangling reference.  
The best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value. 

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.


## Lifetime Annotations in Struct Definitions

Defined struct can not only hold owned types, they can also hold references. In the case of structs, there needs to be a lifetime annotation on every reference in the struct's definition. 

Example: A struct that holds a reference, so its definition needs a lifetime annotation

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...")l
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    }
}
```

The struct has one field, `part`, that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. 
This annotation means an instance of `ImportantExcept` can't outlive the reference it holds in its `part` field. 

The `main` function here creates an instance of the `ImportantExcept` struct that holds a reference to the first sentence of the `String` owned by the variable `novel`. The data in novel exists before the `ImportantExcept` instance is created. Additionally, `novel` doesn't go out of scope until after the `ImportantExcept` goes out of scope, so the reference in the `ImportantExcept` instance is valid. 


## Lifetime Elision

Every reference has a lifetime and that we need to specify lifetime parameters for functions or structs that use references.

```
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime.

At the time, the function signature would have been written like this: 

```
fn first_word<'a>(s: &'a str) -> &'a str {}
```

Automatically accounting for lifetime annotations into the compiler's code would allow the borrow to infer the lifetimes in these situations and wouldn't need explicit annotations. 

This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The patterns programmed into Rust's analysis of reference are called the **lifetime elision rules**. 
These aren't rules per se, rather a set of edge cases that the compiler will consider. 
If our code fits these cases, then we don't need to write the lifetimes explicitly. 

The elision rules don't provide full inference. 

If Rust deterministically applies the rules but there's still ambiguity as to what lifetimes those references have, the compiler won't guess what the lifetime of the remaining references should be. 
In this case, instead of guessing, the compiler will give us an error that we can resolve by adding the lifetime annotations that specify how the references relate to each other. 

Lifetimes on function or method parameters are called **input lifetimes** 
Lifetimes on return values are called **output lifetimes**. 

The compiler uses 3 rules to figure out what lifetimes references have when there aren't explicit: 

1. Input lifetimes
2. Output lifetimes
3. Output lifetimes

If the compiler gets to the end of the 3 rules and there are still references for which it can't figure out lifetimes, the compiler will stop with an error. 

These rules apply to `fn` definitions as well as `impl` blocks. 

The **first** rule is that each parameter that's a reference gets its own lifetime parameter. 

Code Examples: 

A function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`
A function with 2 parameters gets 2 paramteres: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`

The **second** rule is if there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`

The **third** rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. 
This **third** rule makes methods nicer to read and write because fewer symbols are necessary. 

Compiler Examples with Lifetime elisions: 

The code example is lifetimes of the references in the signature of the `first_word` function. 

The signature will start without any lifetimes associated with the references: 

```
fn first_word(s: &str) -> &str {}
```

The compiler applies the **first** rule, which specifies that each parameter gets its own lifetime. 

It will use some form of lifetime parameter, in this case `&'a`. 

```
fn first_word<'a>(s: &'a str) -> &str {}
```

The **second rule** applies because there's exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assign to the output lifetime. 

```
fn first_word<'a>(s: &'a str) -> &'a str {}
```

All the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature. 

Code Example: lifetime elision reveiw with the `longest` function that has 2 parameters

The signature will start without any lifetimes associated with the references: 

```
fn longest(x: &str, y: &str) -> &str {}
```

The **first** rule will be applied: each parameter gets its own lifetime. Now, we have 2 parameters instead of one, so there are 2 lifetimes; 

```
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```

The second rule doesn't apply for our code because there is more than one input lifetime. 
The third rule doesn't apply either because `longest` is a function rather than a method, so none of the parameters are `self`. 

After working through all of the 3 rules, we still haven't figured out what the return's type lifetime is. 


## Lifetime Annotations in Method Definitions

When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters. 
Where we declare and use the lifetime parameters depends on whether they're related to the struct fields or the method parameters and return values. 

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name, because those lifetimes are part of the struct's type. 

In method signatures inside the `impl` block, references might be tied to the lifetime of references in the struct's fields, or they might be independent. Additionally, the lifetime elision rules often make it so that lifetime annotations aren't necessary in method signatures. 

Example: `ImportantExcerpt` struct with a lifetime

A method named `level` whose only parameter is a reference to `self` and whose return value is `i32`, which isn't a reference to anything: 

```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after `impl` and its user after the type name are required, but we aren't required to annotate the lifetime of the reference to `self` because of the first elision rule. 

Example: The 3rd lifetime elision rule applies

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are 2 input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self` and `announcement` their own lifetimes. Then, because one of the parameters is `&self`, the return type gets the lifetime of `&self`, and all lifetimes have been accounted for. 


## The Static Lifetime

One special lifetime is `'static`, which means that this reference **can live** for the entire duration of the program. 
All string literals have the `'static` lifetime, which we can annotate as follows: 

```
let s: &'static str = "I have a static lifetime.";
```

The text of the string above is stored in the program's binary, which is always available. Therefore, the lifetime of all string literals is `'static`. 

We might see suggestions to use the `'static` lifetime in error messages. But before specifying `'static` as the lifetime for a reference, think about whether the reference we have actually lives the entire lifetime of our program or not. We might consider whether we want it to live that long, even if it could. 

<ins>Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. In these cases, the solution is fixing those problems, not specifying the `'static` lifetime.</ins>

