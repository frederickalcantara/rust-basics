# Modules

modules and other parts of the module system, namely paths that allow you to name items;

The `use` keyword that brings a path into scope, and the `pub` keyword to make items public

**Modules** organize code within a crate into groups for readability and easy reuse. 

Modules also control the *privacy* of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

We can create a new library named restaurant by running `cargo new --lib restaurant`. 

We define a module by starting with the mod keyword and then specify the name of the module and place curly brackets around the body of the module.

Inside modules, we can have other modules. Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions.

By using modules, we can group related definitions together and name why they’re related.

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.