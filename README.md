# Packages and Crates In Rust

A crate is the smallest amount of code that the Rust compiler considers at a time.

A crate can come in two forms: a binary crate and a library crate.

Binary crates are compiled into an executable and can be run from the command line. Each must have a function called main that defines what happens when the executable is run.

Library crates don't have a main function, and they don't compile into an executable. They are used to provide functionality to other crates.

The crate root is a source file that the Rust compiler starts from and makes up the root module of a crate.

A package is a bundle of one or more crates that provides a set of functionalities. It contains the Cargo.toml file, which describes the crates that make up the package.

Cargo is actually a package that contains the binary crates and library crates.

A package can contain as many binary crates as you like, but it must contain at least one crate, whether that's a library or a binary crate.

If a package contains both a src/main.rs and src/lib.rs, it has two crates: a binary and library, both with the same name as the package.

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.


# A Modules Cheat Sheet
- Start from the crate root: When you're compiling a crate, the compiler first looks in the crate root file, these can either be src/lib.rs or src/main.rs for the code to compile.

- Declaring modules: Modules are declared with the mod keyword. Modules are hierarchical: you can have modules inside other modules.
Imagine you declare a module with mod bread; The compiler will look for the module's code in these places:
- src/bread.rs
- src/lib.rs

- Declaring submodules: In any file other than the crate root, you can declare submodules.
- An example: declare mod rice; in src/bread.rs The compiler will look for the code within the directory named for the parent module in these places:
- Inline, directly following mod rice, within curly brackets instead of the semicolon
- In the file src/bread/rice.rs
- In the file src/bread/rice/mod.rs

- Paths to code in modules: Once a module is part of your crate, you can refer to the code in that module by using the path to the module.
- An example: use bread::rice::carrot as carrot;

- Private vs Public: Code within a module is private from its parent modules by default. To make it public, you need to use the pub mod instead of mod. To make items within a public module public as well, use pub before their declaration.
- An example: pub fn eat() {
- An example: pub mod rice {

- The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repeating code.
- An example: use std::io;
- In any scope that can refer to crate::bread::rice::carrot; , you can use a shortcut with use crate::bread::rice::carrot; and you only need to write carrot to make use of that type in the scope.


## Grouping Related Code In Modules
Modules let us organize code within a crate for easy readability and easy reuse. Modules allows us to control the privacy of items, because code within a module is private by default.

Private items are internal implementation details not available for outside use.

You can choose to make your module and the items within it public, this exposes them to allow external code to use and depend on them.

# Paths In Rust
To show Rust where to find an item in a module tree, you can use the path attribute. To call a function, you need to know its path in the module tree.

A path can take two forms:
- An absolute path: This is the path to the item in the root of the module tree. Absolute paths start with the crate root and end with the item you want to find.
- A relative path: This is the path to the item in the module tree. Relative paths start with the current module and end with the item you want to find and uses self, super or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by "::"


# Making Structs and Enums Public
If you use pub before a struct definition, you make the struct public, but the struct fields will still be private

# Enums 
Enums are not very useful unless their variants are public. The default for enum variants is private.

# The Glob Operator
This is needed when you want to bring all public items defined in a path into scope (*0
'' use std:: collections::'';

The use statement brings all public items defined in std::collections into the current scope. There's a downside of using the Glob Operator, it can make it harder to tell what names are in scope and where a name used in your program was defined

The glob operator is used when testing to bring everything under test into the tests module 
