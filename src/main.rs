#[macro_use]
extern crate clap;
extern crate ansi_term;

use clap::App;
use ansi_term::Colour::Green;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Module {
    pub name: String,
    path: PathBuf
}

impl Module {
    pub fn new(name: std::ffi::OsString, path: PathBuf) -> Module {
        let name = name.into_string().unwrap();
        Module { name: name, path: path }
    }

    pub fn readme(&self) {
        let readme = self.get_information(&String::from("readme"));
        let path = create_temp_file(format!("{}.md", self.name), readme);    

        Command::new("open")
            .arg(path.to_str().unwrap())
            .output()
            .expect("failed to open README");
    }

    fn get_information(&self, subject: &String) -> String {
        let info = Command::new("yarn")
            .args(&["info", &self.name, subject])
            .output()
            .expect("failed to execute `yarn global bin`, do you have Yarn installed?");

        String::from_utf8(info.stdout).unwrap()
    }
}

fn create_temp_file(filename: String, content: String) -> PathBuf {
    let mut dir = env::temp_dir();
    dir.push(filename);

    let mut file = File::create(&dir).unwrap();
    file.write_all(content.into_bytes().as_slice()).unwrap();
    
    dir
}

fn main() {
    let modules = init();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("readme") => {
            if matches.is_present("module") {
                let module = matches.value_of("module").unwrap();
                let selected_module = modules.iter().find(|x| x.name == module);

                match selected_module {
                    Some(m) => m.readme(),
                    None => println!("Module `{}` currently not installed", module)
                }
            } 
        },
        Some("list") => {
            if modules.is_empty() {
                println!("Currently no installed global modules.");
            } else {
                println!("Currently installed global modules:");
                for module in modules {
                    println!("{} {}", Green.paint("*"), module.name);
                }
            }
        },
        None => println!("No command specified, please refer to nmm --help"),
        _ => println!("No command specified, please refer to nmm --help")
    }
}

fn init() -> Vec<Module> {
    let yarn = Command::new("yarn")
        .args(&["global", "bin"])
        .output()
        .expect("failed to execute `yarn global bin`, do you have Yarn installed?");
    
    let bin_folder = String::from_utf8(yarn.stdout).unwrap();
    let trimmed = bin_folder.trim_right();
    let binary_path = Path::new(&trimmed);

    get_modules(&binary_path)
}

fn get_modules(path: &Path) -> Vec<Module> {
    let mut modules = Vec::new();

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if &entry.file_name() != "node" {
                let module = Module::new(entry.file_name(), entry.path());
                modules.push(module);
            }
        }
    }

    modules
}
