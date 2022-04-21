# Storing Keys with Associated Values in Hash Maps

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a **hashing function**, which determines how it places these keys and values into memory. 

Hash maps are part of the standard library. 

<ins>Note: Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.</ins>

Hash maps are useful whenever we want to look up data not by using an index, as we can with vectors, but by using a key that can be of any type. 

## Creating a New Hash Map

One way to create an empty hash map is using `new` and adding elements with `insert`. 

Example: Creating a new hash map and inserting some keys and values using `new`

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

<ins>Note: We need to use `use` to bring the `HashMap` from the collections portion of the standard library.</ins> 

Out of the 3 common collections, Hash maps are the least commonly used, so it isn't included in the features that's brought into scope automatically in the prelude. Hash maps also have less support from the standard library; there's no built-in macro to construct them. 

Similar to vectors, hash maps store their data on the heap. Also like vectors, hash maps are homogenous: All of the keys must have the same type, and all of the values must have the same type. 

Another way to build a hash map is by using iterators and the `collect` method on a vector of tuples, where each tuple consists of a key and its value. The `collect` method gathers data into a number of collection types, including `HashMap`. 

Example: Creating a hash map from a list of teams and a list of scores
In the example below, we can use the `zip` method to create an iterator of tuples where a String is paired with a value and then we use the `collect` method to turn that iterator of tuples into a hash map. 

```
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

The type annotation `HashMap<_, _>` is needed here because it's possible to `collect` into many different data structures and Rust doesn't which one we want unless we specify. For parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

## Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values. 

```
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using and 
// see what compiler error we get!
```

We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.

If we insert references to values into the hash map, the values won't be moved into the hash map. 
The values that the references point to must be valid for at least as long as the hash map is valid.

## Accessing Values in a Hash Map 

We can get a value out of the hash map by providing its key to the `get` method

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

In this example, `score` will have the value that's associated with the Blue team, and the result will be `Some(&10)`. 
The result is wrapped in `Some` because `get` return an `Option<&V>`; if there's no value for that key n the hash map, `get` will return `None`. The program will need to handle the `Option`. 

We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a `for` loop: 

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

THe score will print the following: 

```
Yellow: 50
Blue: 10
```

## Updating a Hash Map 

Although the number of key and value pairs is growable, each key can only have one value associated with it at a time. When we want to change the data in a hash map, we have to decide how to handle the case when a key already has a value assigned. 

We could one of the following: 

- We could replace the old value with the new value, completely disregarding the old value.
- We could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value.
- We could combine the old value and the new value.

### Overwriting a Value

If we insert a key and value into a hash map and then insert that same key with a different value, then the value associated with that key will be replaced. 

Even if we use `insert` twice, if we use `insert` against the same key then the new `value` will overwrite the old value 

Example: Replacing a value stored with a particular key

```
use std::collections:HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

The code above will print `{"Blue": 25}`. The original value of `10` has been overwritten. 

### Only Inserting a Valu if the Key has no Value

It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it.

Hash maps have a special API for this called `entry` that takes the key we want to check as a parameter. The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist. 

Example: Using the `entry` method to only insert if the key doesn't already have a value
We want to check whether the key for the Yellow team has a value associated with it. If it doesn't, then we want to insert the value 50, and the same for the Blue team. 

```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if it doesn't, then it inserts the parameters as the new value for this key and returns a mutable reference to the new value. 
This technique is much cleaner than writing the logic ourselves and, in addition, plays nicely with the borrow checker. 

Running the code will print `{"Yellow": 50, "Blue": 10}`. The first call to `entry` will insert the key for the Yellow team with the value 50 because the Yellow team doesn't have a value already. The second call to `entry` won't change the hash map because the Blue team already has the value of 10. 

#### Updating a Value based on the Old Value

Another common use case for hash maps is to look up a key's value and then update it based on the old value. 

We can use a hash map with the words as keys and increment the value to keep track of how many times we've seen that word. 

Example: Counting occurrences of words using a hash map that stores words and counts

```
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

The code will print `{"world": 2, "hello": 1, "wonderful": 1}`. The `split_whitespace` method iterates over sub-slices, separated by whitespace, of the value in `text`. The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specific key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). 

The mutable reference goes out of the scope at the end for the `for` loop, so all of these changes are safe and allowed by the borrowing rules. 

## Hashing Functions

`HashMap` uses a hashing function called **SipHash** that can provide resistance to Denial of Service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.

If we profile our code and find that the default hash function is too slow for our purposes, then we can switch to another function by specifying a different hasher. A **hasher** is a type that implements the `BuildHasher` trait. 

There are libraries from crates.io that provide different hashers. 
