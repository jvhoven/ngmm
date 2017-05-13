use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate clap;
use clap::App;

#[derive(Debug)]
struct Module {
    name: std::ffi::OsString,
    path: PathBuf,
    information: String
}

impl Module {
    pub fn new(name: std::ffi::OsString, path: PathBuf, information: String) -> Module {
        Module { name: name, path: path, information: information }
    }
}

fn main() {
    //let modules = init();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    match matches.subcommand_name() {
        Some("readme") => {
            println!("{}", "test");
        },
        Some("list") => {}, //println!("{:?}", modules),
        None => println!("No subcommand"),
        _ => println!("Other subcommand used")
    }
}

fn init() -> Vec<Module> {
    let yarn = Command::new("yarn")
        .args(&["global", "bin"])
        .output()
        .expect("failed to execute `yarn global bin`, do you have Yarn installed?");
    
    let mut bin_folder = String::from_utf8(yarn.stdout).unwrap();

    // Remove newline
    bin_folder.pop();

    let binary_path = Path::new(&bin_folder);
    get_modules(&binary_path)
}

fn get_modules(path: &Path) -> Vec<Module> {
    let mut modules = Vec::new();

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let module_info = module_information(&entry.file_name(), &String::from("description"));
            modules.push(Module::new(entry.file_name(), entry.path(), module_info));
        }
    }

    modules
}

fn module_information(module: &std::ffi::OsString, subject: &String) -> String {
    let info = Command::new("yarn")
        .args(&["info", module.to_str().unwrap(), subject])
        .output()
        .expect("failed to execute `yarn global bin`, do you have Yarn installed?");

    String::from_utf8(info.stdout).unwrap()
}

// TODO: If markdown file exists
fn open_readme(module: std::ffi::OsString) {
    let readme = module_information(&module, &String::from("readme"));
    let path = create_temp_file(format!("{}.md", module.into_string().unwrap()), readme);    

    Command::new("open")
        .arg(path.to_str().unwrap())
        .output()
        .expect("failed to open README");
}

fn create_temp_file(filename: String, content: String) -> std::path::PathBuf {
    let mut dir = env::temp_dir();
    dir.push(filename);

    let mut file = File::create(&dir).unwrap();
    file.write_all(content.into_bytes().as_slice()).unwrap();
    
    dir
}
