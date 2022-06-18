
fn private_function(){
    println!("calling is a private functions");
}

pub fn public_func(){
    private_function();
}
