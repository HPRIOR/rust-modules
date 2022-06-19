mod module_one; // importing module (declared with mod.rs)

pub fn lib_func() {
    println!("this can be called from main");
    module_one::public_function();
    module_one::nested_module::nested_function();
}

