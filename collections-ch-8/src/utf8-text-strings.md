# Storing UTF-8 Encoded Text with Strings

Strings are tricky for 3 reasons: 

1. Rust’s propensity for exposing possible errors
2. Strings being a more complicated data structures than many programmers give them credit for
3. UTF-8

Strings will be discussed in the context of collections because strings are implemented as a collection of bytes. 

## What is a String? 

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

**String slices** are references to some UTF-8 encoded string data stored elsewhere. 

String literals, are stored in the program's binary and are therefore string slices. 

The `String` type, is provided by Rust's standard library rather than coded into the code language, is a growable, mutable, owned, UTF-8 encoded string type.

<ins>When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.</ins>

Rust also has number of other string types: `OsString`, `OsStr`, `CString`, and `CStr`. 

These string types can store text in different encodings or be represented in memory in a different way. 

## Creating a new String

Many of the same operations available with `Vec<T>` are available with `String` as well, starting with the new function to create a string.

Example: Creating a new empty string.

```
let mut s = String::new();
```

We usually have some data that we want to start a string with. 

In this case, we use the `to_string` function, which is available on any type that implements the `Display` trait, as string literals do.

Example: Creating a string that contains "initial contents"

```
let data = "initial contents";

let s = data.to_string();

// The method also works on a literal directly: 
let s = "initial contents".to_strings();
```

We can also use the function `String::from` to create a `String` from a string literal.

Example below

```
let s = String::from("initial contents");
```

Strings are used for so many things, we can use many different generic APIs for strings, providing us with a lot of options. Some of them are redundant, but they all have their place!

<ins>String::from and to_string do the same thing, so which you choose is a matter of style and readability.</ins>

Strings are UTF-8 encoded, which allows us to include any properly encoded data in them. 

Example: Different language as strings

```
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

All of the above are valid `String` values.

## Updating a String

A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`, if we push more data into it. 
Additionally, we can use the `+` operator or the `format!` macro to concatenate `String` values. 

### Appending to a String with `push_str` and `push`


#### `push_str`

We can grow a `String` by using the `push_str` method to append a string slice. 

```
let mut s = String::from("foo");
s.push_str("bar");
```

The `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter. 

Example: Using a string after appends its content to a `String`

```
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```

If the `push_str` method took ownership of `s2`, we wouldn't be able to print its value on the last line. 
The code above works and prints "bar".

#### `push`

The `push` method takes a single character as a parameter and adds it to the `String`. 

Example: Adding one character to a `String` value using `push`

```
let mut s = String::from("lo");
s.push("l");
```

The `s` variable will print "lol".

## Concatenation with the `+` Operator or the `format!` Macro

There are times in which we want to combine 2 existing strings. 

```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The string `s3` will contain `Hello, world!`. The reason `s1` is no longer valid after the addition, and the reason we used a reference to `s2`, has to do with the signature of the method that's called when we use the `+` operator. 

The `+` operator uses the `add` method, whose signature looks somthing like this: 

```
fn add(self, s: &str) -> String {}
```

The reason the code example compiled is because he add function can only add 1 `&str` (string reference) to a `String`. We can't add 2 `String` values together. 

We can use `&s2` in the call to `add`, which allows the compiler to <ins>coerce</ins> the `&String` argument into a `&str`. 
When we call the `add` method, Rust uses a <ins>deref coercion</ins>, which turns `&s2` into `&s2[..]`. 
`add` also doesn't take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation. 

In the `add` function, we can see that the signature takes ownership of `self`, because `self` doesn't have an `&`. 
`s1` will be moved in to the `add` call and will no longer be valid afterwards. 

This statement `let s3 = s1 + &s2` actually takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. 

An easier way to copy strings is to use the `format!` macro: 

```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

This makes it easier to read than adding multiple `+`s. 

Any code generated by the `format!` macro uses references so that this call doesn't take ownership of any of its parameters. 

## Indexing into Strings

In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. 
However, if we try to access parts of a `String` using the indexing syntax in Rust, we get an error. 

```
let s1 = String::from("hello");
let h = s1[0];
```
This code won't compile because Rust strings don't support indexing. 

### Internal Representation

A String is a wrapper over a `Vec<u8>`.

Example: UTF-8 String

```
let hello = String::from("Hola");
```

In this example, `len` will be 4, which means the vector storing the string “Hola” is 4 bytes long.

Each of the letters above take 1 byte when encoded in UTF-8. 

The line below will take up more bytes because each Unicode scalar value in that string takes 2 bytes of storage. 

```
let hello = String::from("Здравствуйте");
```

An index into the string's bytes won't always correlate to a valid Unicode scalar value. 

When it comes to strings, its best to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

### Bytes and Scalar Values and Grapheme Clusters! Oh My!

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

The code above results in 18 bytes and this is how computers store the data. 

If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:

['न', 'म', 'स', '्', 'त', 'े']

There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

["न", "म", "स्", "ते"]

Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

## Slicing Strings

Indexing into a string is generally a bad idea because it's not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. 
If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.

<ins>Note: We should use ranges to create string slices with caution, because doing so can crashes to our program</ins>

## Methods for Iterating Over Strings

**The best way to operate on pieces of strings is to be explicit about whether we want characters or bytes.**

or individual Unicode scalar values, use the chars method. Calling chars on “नमस्ते” separates out and returns six values of type char

We can iterate over the result to access each element: 

```
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

The code will print this out: 

```
न
म
स
्
त
े
```

Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:

```
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

This code will print the 18 bytes that make up the `String`: 

```
224
164
// --snip--
165
135
```

Take note that valid Unicode scalar values may be made up of more than 1 byte. 

Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.

Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront.