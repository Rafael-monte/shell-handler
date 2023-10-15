use std::{path::{PathBuf, Path}, str::FromStr, ffi::OsString, fs::{self, OpenOptions}, collections::HashMap, io::{ErrorKind, Write}};

use crate::config;

use super::abs_config_file_handler::{self, AbstractConfigFileHandler};

pub struct BashConfigFile {
    file_path: PathBuf,

}

impl BashConfigFile {
    pub fn new(bash_file_path: Option<&str>) -> Self {
        let mut file_path: OsString = OsString::from_str(config::DEFAULT_SHELL_LOCATION).unwrap();
        if bash_file_path.is_some() {
            file_path = OsString::from_str(bash_file_path.unwrap()).unwrap();
        }
        return Self {
            file_path: PathBuf::from(file_path)
        }
    }


    pub fn read_aliases_file(&self) -> Result<HashMap<String, String>, ErrorKind> {
        if !self.check_file_status() {
            println!("Não existe nenhum arquivo de configurações preparado!");
            return Err(ErrorKind::NotFound);
        }
       let aliases = self.get_aliases();
       return Ok(aliases)
    }

    fn cut_line_elements(&self, line: &str) -> (String, String) {
        let equal_sign_index = line.find("=").unwrap();
        let left_side = &line[..equal_sign_index];
        let alias_command = &line[equal_sign_index+1..];
        let alias_name_whitespace = left_side.find(" ").unwrap();
        let alias_name = &left_side[alias_name_whitespace+1..];
        return (alias_name.to_owned(), alias_command.to_owned());
    }
}

impl abs_config_file_handler::AbstractConfigFileHandler for BashConfigFile {
    fn check_file_status(&self) -> bool {
        return Path::new(self.file_path.as_os_str()).exists();
    }
    fn get_aliases(&self) -> HashMap<String, String> {
        let file_content: Result<String, std::io::Error> = fs::read_to_string(self.file_path.clone());
        if file_content.is_err() {
            eprintln!("Não foi possível carregar os conteúdos do arquivo de configuração");
            panic!();
        }
        let mut commands: HashMap<String, String> = HashMap::new();
        for line in file_content.unwrap().lines() {
            let (alias_name, alias_command) = self.cut_line_elements(line);
            commands.insert(alias_name, alias_command);
        }
        return commands;
    }
    fn add_new_alias(&mut self, alias_name: &str, command: &str) -> () {
        let opening_file = OpenOptions::new()
                    .append(true)
                    .open(self.file_path.as_os_str());
        if opening_file.is_err() {
            eprintln!("Ocorreu um erro ao abrir o arquivo de configurações");
            panic!();
        }
        let mut file = opening_file.unwrap();
        writeln!(file, "alias {alias_name}=\"{command}\"").unwrap()
    }
}