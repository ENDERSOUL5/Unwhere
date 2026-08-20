use std::env;


pub mod managers;


fn main() {


    let args: Vec<String> = env::args().collect();

    managers::uninstall(args.get(1).expect("the package name to remove is required"));








    
}
