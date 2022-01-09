# RUST MODULE SYSTEM

- A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

- In rust, modules are not mapped to file system!!!

## USE AND MOD 

- Using use brings a Rust item into the current scope. 
- In most cases, using use is optional, and can be avoided by referring to the full path of the item you 
want to access. Use is primarily used to make code cleaner and less verbose.
- Using use simply brings another item into the current namespace by following another path.
An item is some general object such as a function, struct or trait that you need to access.
- The current namespace means bringing an item into the current file so you can access it as if it were local.

- Using mod defines a module which is a collection of Rust items.
- In Rust a module is simply a container for zero or more items. It’s a way of grouping items together in a logical way so that your module is easy to navigate.
- When you define a module, you can refer to any item inside it either by its full path, 
- or by using use to bring the module into scope.


## Visibility
- By default, the items in a module have private visibility, but this can be overridden with the pub modifier. 
Only the public items of a module can be accessed from outside the module scope.

- Structs have an extra level of visibility with their fields.
- Public structs with private fields cannot be constructed using field names.

- The use declaration can be used to bind a full path to a new name, for easier access. 
It is often used like this:
- A use declaration creates one or more local name bindings synonymous with some other path.
- Like items, use declarations are private to the containing module, by default.
- Usually a use declaration is used to shorten the path required to refer to a module item
- The `self` keyword refers to the current module scope
------------------------------------------------------------------------------------------
mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called `deeply::nested::function()`");
    }
  }
}

- use crate::deeply::nested::function;
  function();
------------------------------------------------------------------------------------------

## Crates

- A crate is a compilation unit in Rust.
- A crate is a binary or library.
- The crate root is a source file that the Rust compiler starts from
- Whenever rustc some_file.rs is called, some_file.rs is treated as the crate file.

- A package is one or more crates that provide a set of functionality. 
- A package contains a Cargo.toml file that describes how to build those crates.

- cargo-modules: A cargo plugin for showing an overview of a crate's modules.
cargo modules generate tree --with-types
