// mod file used to declare directory level module
use crate::module_one::encapsulate::public_func;

pub mod nested_module; // declaring the nested module public so that it can be used 
                       // be imported into super modules 
mod encapsulate; // this is invisible to the super modules
                 // however its public members can be used here 

pub fn public_function(){
    println!("module_one: this is a public function in module_one");

    public_func();
}
