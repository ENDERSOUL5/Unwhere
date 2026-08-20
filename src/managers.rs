use std::{io, process::Command, rc::Rc, vec};

use regex::RegexBuilder;


struct PackageManager{
     name: String,
    // command used to obtain the ID needed for deletion
     list_command: String,

     column_num: i8, // Starts from 0
    
    // Command to delete the package along with the ID
     delete_command: String,

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

    found_packages.iter().enumerate().for_each(|package_actual| {
        println!("({}) {} {}", package_actual.0, package_actual.1.name, package_actual.1.package_manager.name)
    });

    // Delete if only one
    if found_packages.len() == 1{
        found_packages.get(0).unwrap().delete();
    }
    else if found_packages.len() == 0 {println!("no match found");
            return;
    }
    else{
        println!("choose the option");
        let mut option = String::new();

        io::stdin().read_line(&mut option);

        let options:i32 = option.trim().parse().unwrap();

        if let Some(package) = found_packages.get(options as usize) {
            package.delete();
        }
    }


}




impl PackageManager{

    /// Returns the command output
     fn execute_list_command(&self) -> String{
        let output = Command::new(&self.name)
        .args(self.list_command.split_whitespace()).output();

        if let Err(_) = output {return String::new() }

        let output = output.unwrap().stdout;



        let output = str::from_utf8(&output).expect("errors");


        // Only the column with package names should be obtained
        // Some packages add more info but we only care about the name since it's used for uninstalling
        
        let packages: Vec<&str> = output.lines()
        .filter_map(|line| {
            line.split_whitespace().nth(self.column_num as usize)
        })
        .collect();

        packages.join("\n")


        

        
        
    }

     fn execute_delete_command(&self, name: &str){
        if self.need_delete_root {
        let full_command = format!("{} {} {}", self.name, self.delete_command, name);
        let _= Command::new("sudo").args(full_command.split_whitespace()).status();
        }
        else {
        let full_command = format!("{} {}",self.delete_command, name);
        let _= Command::new(&self.name).args(full_command.split_whitespace()).status();

        }

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
        let vec = vec![


        Rc::new(PackageManager::new("dnf", 
        "list --installed", 
        "rm", 0, true)),

        Rc::new(PackageManager::new("flatpak", 
        "list --app --columns=application", "uninstall", 
        0, false)),


        Rc::new(PackageManager::new("pacman", "-Q", "-R", 0, true)),

           



        


        ];


        Self { installed: vec }
        


        



    }
}


