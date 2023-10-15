use std::collections::HashMap;

pub trait AbstractConfigFileHandler {
    fn check_file_status(&self) -> bool;
    fn get_aliases(&self) -> HashMap<String, String>;
    fn add_new_alias(&mut self, alias_name: &str, command: &str) -> (); 
}