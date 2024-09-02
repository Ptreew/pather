use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

const MAX_PATH_LENGTH: usize = 1024;
const VERSION: &str = "1.2.1";

fn main() {
    let mut path_array = parse_path_env();
    let home_path = determine_shell_config_path();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => display_help(),
            "-v" | "--version" => println!("Version: {}", VERSION),
            _ => process_arguments(&args[1..], &mut path_array, &home_path),
        }
    } else {
        add_or_remove_current_directory(&mut path_array, &home_path);
    }

    println!("\nTo update the current session, run the command:\n");
    println!("\tsource {}\n", home_path.display());
}

fn parse_path_env() -> Vec<String> {
    env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .map(|s| s.to_string())
        .collect()
}

fn determine_shell_config_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_default();
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.contains("bash") {
        Path::new(&home).join(".bashrc")
    } else if shell.contains("zsh") {
        Path::new(&home).join(".zshrc")
    } else if shell.contains("fish") {
        Path::new(&home).join(".config/fish/config.fish")
    } else {
        eprintln!("Unsupported shell: {}", shell);
        std::process::exit(1);
    }
}

fn path_prepend(arg: &str, path_array: &mut Vec<String>, home_path: &Path) {
    path_array.retain(|p| p != arg);
    path_array.insert(0, arg.to_string());
    update_shell_config(path_array, home_path);
}

fn path_remove(arg: &str, path_array: &mut Vec<String>, home_path: &Path) {
    path_array.retain(|p| p != arg);
    update_shell_config(path_array, home_path);
}

fn update_shell_config(path_array: &[String], config_file: &Path) {
    let new_path = path_array.join(":");

    let temp_file = config_file.with_extension("tmp");
    let mut temp_config = File::create(&temp_file).unwrap_or_else(|e| {
        eprintln!("fopen() error: {}", e);
        std::process::exit(1);
    });

    let config = File::open(config_file).unwrap_or_else(|e| {
        eprintln!("fopen() error: {}", e);
        std::process::exit(1);
    });

    let mut path_line_found = false;

    for line in io::BufReader::new(config).lines() {
        let line = line.unwrap();
        if line.starts_with("export PATH=") {
            writeln!(temp_config, "export PATH=\"{}\"", new_path).unwrap();
            path_line_found = true;
        } else {
            writeln!(temp_config, "{}", line).unwrap();
        }
    }

    if !path_line_found {
        writeln!(temp_config, "export PATH=\"{}\"", new_path).unwrap();
    }

    // Copy the temporary file back to the original configuration file
    fs::copy(&temp_file, config_file).unwrap();

    // Remove the temporary file
    fs::remove_file(&temp_file).unwrap();
}


fn add_or_remove_current_directory(path_array: &mut Vec<String>, home_path: &Path) {
    if let Ok(cwd) = env::current_dir() {
        let absolute_path = cwd.canonicalize().unwrap_or(cwd);
        let absolute_path_str = absolute_path.to_str().unwrap_or_default();

        if path_array.contains(&absolute_path_str.to_string()) {
            path_remove(absolute_path_str, path_array, home_path);
            println!("Removed {} from PATH", absolute_path_str);
        } else {
            path_prepend(absolute_path_str, path_array, home_path);
            println!("Added {} to PATH", absolute_path_str);
        }
    } else {
        eprintln!("getcwd() error");
    }
}

fn process_arguments(args: &[String], path_array: &mut Vec<String>, home_path: &Path) {
    for arg in args {
        if let Ok(metadata) = fs::metadata(arg) {
            if metadata.is_dir() {
                let absolute_path = Path::new(arg).canonicalize().unwrap_or_else(|_| PathBuf::from(arg));
                let absolute_path_str = absolute_path.to_str().unwrap_or_default();

                if path_array.contains(&absolute_path_str.to_string()) {
                    path_remove(absolute_path_str, path_array, home_path);
                    println!("Removed {} from PATH", absolute_path_str);
                } else {
                    path_prepend(absolute_path_str, path_array, home_path);
                    println!("Added {} to PATH", absolute_path_str);
                }
            } else {
                println!("Did not add {} to PATH, path does not exist or is not a directory.", arg);
            }
        } else {
            println!("Did not add {} to PATH, path does not exist or is not a directory.", arg);
        }
    }
}

fn display_help() {
    println!("Usage: add2path [DIRECTORY1 DIRECTORY2 ...]\n");
    println!("Add or remove directories from the PATH variable.\n");
    println!("If run without any arguments, adds the current directory to PATH if it's not already in PATH,");
    println!("otherwise removes it from PATH.\n");
    println!("Options:");
    println!("  -h, --help\t\tDisplay this help message and exit.");
    println!("  -v, --version\t\tDisplay current version and exit.");
}

