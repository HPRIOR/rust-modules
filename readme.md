Modules are imported using the mod keyword, followed by the name of the module. 

```rust
mod module_name;
```

The members of this module will be accessable in the file its imported too. 

To make the imported module externally visiable (for example to areas of code that import the module importing the module), you can use the public keyword

```rust
pub mod module_name
```

If this was declared in a module called `super_module`, you could access `module_name`, like so:

```rust
mod super_module;

pub fn use_super_module(){
    super_module::module_name::member_of_module_name()
}
```

Using private and public modules allows for encapsulation at a module level.







