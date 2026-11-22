use std::{env, fmt::Display,};


pub mod managers;


trait OrExit<T> {
    fn or_exit(self, message: &str) -> T;
}

impl<T> OrExit<T> for Option<T> {
    fn or_exit(self, message: &str) -> T {
        self.unwrap_or_else(||{
            eprintln!("Error: {message}");
            std::process::exit(0)
        })
    }
}

impl<T, E: Display> OrExit<T> for Result<T, E>  {
    fn or_exit(self, message: &str) -> T {
        self.unwrap_or_else(|error|{
            eprintln!("Error: {message} | Details: {error}");
            std::process::exit(0)
        })
    }
}

fn main() {


    let args: Vec<String> = env::args().collect();

    managers::uninstall(args.get(1).or_exit("se debe tener un nomber"));








    
}
