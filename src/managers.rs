use std::{io, process::Command, rc::Rc, vec};

use comfy_table::Table;
use regex::RegexBuilder;

use crate::OrExit;



struct PackageManager{
     name: String,
    // command used to obtain the ID needed for deletion
     list_command: String,

     column_num: i8, 
    
    // Command to delete the package along with the ID
     delete_command: String,

    need_delete_root: bool

}

 struct SelectedPackages<>{
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


    package_managers.installed.iter().for_each(|manager|{

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

    let mut resut_table = Table::new();
    resut_table.set_header(vec!["index", "package_name", "package_manager"]);
    found_packages.iter().enumerate().for_each(|package_actual| {
        resut_table.add_row(vec![
            package_actual.0.to_string(), package_actual.1.name.clone(), package_actual.1.package_manager.name.clone()
        ]);
    });

    println!("{resut_table}");

    // Delete if only one
    if found_packages.len() == 1{
        found_packages[0].delete();
    }
    else if found_packages.len() == 0 {
        println!("no match found");
    }
    else{
        println!("choose the option");
        let mut option = String::new();

        let _= io::stdin().read_line(&mut option);

        let options: i32 = option.trim().parse().or_exit("La opcion elegida no es un numero");

        found_packages.get(options as usize).or_exit("opcion fuera de rango").delete();
    }


}




impl PackageManager{

    /// Returns the command output
     fn execute_list_command(&self) -> String{
        let output = Command::new(&self.name)
        .args(self.list_command.split_whitespace()).output();


        match output {

            Err(_) => {
                return String::new();
            }

            Ok(output) => {
                let output = output.stdout;
                let output = str::from_utf8(&output).expect("errors");

                // Only the column with package names should be obtained
                // Some packages add more info but we only care about the name since it's used for uninstalling
        
                let packages: Vec<&str> = output.lines()
                .filter_map(|line| {
                    line.split_whitespace().nth(self.column_num as usize)
                })
                .collect();

                return packages.join("\n")

            }
            
        }

        
    }

     fn execute_delete_command(&self, name: &str){
        if self.need_delete_root {
        let full_command = format!("{} {} {}", self.name, self.delete_command, name);
        let _= Command::new("sudo").args(full_command.split_whitespace()).status();
        return;
        }
        let full_command = format!("{} {}",self.delete_command, name);
        let _= Command::new(&self.name).args(full_command.split_whitespace()).status();

    }

    pub fn new(name: &str, list_command: &str, delete_command: &str, column_num: i8, need_delete_root: bool) -> Self{
        PackageManager { name: name.to_string(), list_command: list_command.to_string(), delete_command: delete_command.to_string(),
            column_num, need_delete_root
         }
    }
}


struct InstalledPackageManager{

    installed: Vec<Rc<PackageManager>>
    
}


impl InstalledPackageManager {
    
    /// Create the struct with all supported package managers
    /// To add a new manager you just need to add the attributes for each manager
     fn new() -> Self{

        const NEED_ROOT: bool = true;

        let all_package_managers = vec![


        Rc::new(PackageManager::new("dnf", 
        "list --installed", 
        "rm", 0, NEED_ROOT)),

        Rc::new(PackageManager::new("flatpak", 
        "list --app --columns=application", "uninstall", 
        0, !NEED_ROOT)),


        Rc::new(PackageManager::new("pacman", "-Q", "-R", 0, NEED_ROOT)),

           

        ];

        let installed_package_manager = 
        all_package_managers.into_iter().filter(|item|{
            which::which(item.name.as_str()).is_ok()
        }).collect();





        Self { installed: installed_package_manager }
        


        



    }
}


