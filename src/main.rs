use model::bash_config_file;

mod model;
mod config;

fn main() {
   let file = bash_config_file::BashConfigFile::new(None);
   let file_aliases = file.read_aliases_file();
   if file_aliases.is_ok() {
    println!("Aliases:");
    for (alias_name, alias_command) in file_aliases.unwrap().iter() {
        println!("{alias_name}, {alias_command}");
    } 
   }
}
