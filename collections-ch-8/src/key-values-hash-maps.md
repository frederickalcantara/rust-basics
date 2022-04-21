# Storing Keys with Associated Values in Hash Maps

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a **hashing function**, which determines how it places these keys and values into memory. 

<ins>Note: Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.</ins>

Hash maps are useful whenever we want to look up data not by using an index, as we can with vectors, but by using a key that can be of any type. 

## Creating a New Hash Map