/*
 * module lib is defined in the cargo.toml file with a path to the lib root
 */
use module_lib::lib_func;

fn main() {
    println!("Hello, world!");

    lib_func();
}
