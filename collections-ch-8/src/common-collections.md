# Common Collections

Rust has a standard library that include a number of useful data structures called **collections**. 

Most data types represent one specific value, but collections can contain multiple values. 

Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 

Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. 

There are 3 collections taht are used frequently in Rust programs: 

1. **Vector**: Allows us to store a variable number of values next to each other. 
2. **String**: Collection of characters. 
3. **Hash Map**: Allows us to associate a value with a particular key. It's a specific implementation of the more general data structure called a **map**