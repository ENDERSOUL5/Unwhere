use std::env;


pub mod gestores;


fn main() {


    let args: Vec<String> = env::args().collect();

    gestores::uninstall(args.get(1).expect("se necesita el nombre del paquete a eleminar"));








    
}
