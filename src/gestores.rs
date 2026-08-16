use std::{io, process::Command, rc::Rc, vec};

use regex::RegexBuilder;


struct PackageManager{
     name: String,
    // comando que se utiliza para obtener el ID necesario para eliminar
     list_command: String,

     column_num: i8, // Se inicia desde 0
    
    // Comando para junto con el ID eliminar el paquete
     delete_comand: String,

    need_delete_root: bool

}

 struct SelectedPackages{
    name: String,
    package_manager: Rc<PackageManager>
}

impl SelectedPackages{

     fn delete(&self){
        self.package_manager.execute_delete_command(&self.name);
    }
}


pub fn uninstall(package_name: &str){

    let mut found_packages: Vec<SelectedPackages> = Vec::new();
    let package_managers = InstalledPackageManager::new();

    let regex = RegexBuilder::new(package_name).case_insensitive(true).build().unwrap();


    package_managers.instalado.iter().for_each(|manager|{

        let list_packages = manager.execute_list_command();


        list_packages.lines().for_each(|line|{
            if regex.is_match(line){
                let select_packa = SelectedPackages{
                    name: line.to_string(),
                    package_manager: Rc::clone(manager),
                };

                found_packages.push(select_packa);

            }
            

        });


    });

    found_packages.iter().enumerate().for_each(|package_actual| {
        println!("({}) {} {}", package_actual.0, package_actual.1.name, package_actual.1.package_manager.name)
    });

    // Eliminar si solo hay uno
    if found_packages.len() == 1{
        found_packages.get(0).unwrap().delete();
    }
    else if found_packages.len() == 0 {println!("no se encontro ninguna coincidencia");
            return;
    }
    else{
        println!("elije la opcion");
        let mut opcion = String::new();

        io::stdin().read_line(&mut opcion);

        let opciones:i32 = opcion.trim().parse().unwrap();

        if let Some(package) = found_packages.get(opciones as usize) {
            package.delete();
        }
    }


}




impl PackageManager{

    /// retorna la salida del comando
     fn execute_list_command(&self) -> String{
        let salida = Command::new(&self.name)
        .args(self.list_command.split_whitespace()).output();

        if let Err(_) = salida {return String::new() }

        let salida = salida.unwrap().stdout;



        let salida = str::from_utf8(&salida).expect("errores");


        // Se debe solo obtener la columna que tiene los nombres de paquetes
        // hay paquetes que agregan mas cosas pero solo nos interesa el nombre porque se usa para desinstalar
        
        let packages: Vec<&str> = salida.lines()
        .filter_map(|linea| {
            linea.split_whitespace().nth(self.column_num as usize)
        })
        .collect();

        packages.join("\n")


        

        
        
    }

     fn execute_delete_command(&self, name: &str){
        if self.need_delete_root {
        let full_command = format!("{} {} {}", self.name, self.delete_comand, name);
        let _= Command::new("sudo").args(full_command.split_whitespace()).status();
        }
        else {
        let full_command = format!("{} {}",self.delete_comand, name);
        let _= Command::new(&self.name).args(full_command.split_whitespace()).status();

        }

    }

    pub fn new(name: &str, list_command: &str, delete_command: &str, column_num: i8, need_delete_root: bool) -> Self{
        PackageManager { name: name.to_string(), list_command: list_command.to_string(), delete_comand: delete_command.to_string(),
            column_num, need_delete_root
         }
    }
}


struct InstalledPackageManager{

    instalado: Vec<Rc<PackageManager>>
    
}

impl InstalledPackageManager {
    
    /// Crear el struct con todos los manejadores de paquetes soportados
    /// Para agregar un nuevo manejador solo se tiene que colocar los atributos de cada manejador
     fn new() -> Self{
        let vec = vec![


        Rc::new(PackageManager::new("dnf", 
        "list --installed", 
        "rm", 0, true)),

        Rc::new(PackageManager::new("flatpak", 
        "list --app --columns=application", "uninstall", 
        0, false)),


        Rc::new(PackageManager::new("pacman", "-Q", "-R", 0, true)),

           






        






        ];


        Self { instalado: vec }
        






    }
}







