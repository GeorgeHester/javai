//use regex::Regex;

// Need a recursive functions that gets the files in each sub folder

use std::env;
use std::fs;
use std::fs::DirEntry;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

pub fn read_directory(directory: PathBuf) -> Result<Vec<String>, String>
{
    let mut output_files: Vec<String> = Vec::new();
    let files: io::Result<ReadDir> = fs::read_dir(&directory);

    if files.is_err()
    {
        let directory_string: Option<&str> = directory.to_str();

        if directory_string.is_some()
        {
            panic!(
                "Error: Failed to read directory {}",
                directory.to_str().unwrap()
            );
        };

        panic!("Error: Failed to read directory");
    };

    let files_unwrapped: ReadDir = files.unwrap();

    for file in files_unwrapped
    {
        if file.is_err()
        {
            panic!("Error: Failed to read file");
        };

        let file_unwrapped: DirEntry = file.unwrap();
        let file_path: PathBuf = file_unwrapped.path();

        if file_path.is_dir()
        {
            let sub_files: Result<Vec<String>, String> = read_directory(file_unwrapped.path());

            if sub_files.is_err()
            {
                panic!("Error: Failed to read sub directory");
            };

            let mut sub_files_unwrapped: Vec<String> = sub_files.unwrap();

            output_files.append(&mut sub_files_unwrapped);
        };

        if file_path.is_file()
        {
            let file_path_string: Option<&str> = file_path.to_str();

            if file_path_string.is_some()
            {
                let file_path_string_unwrapped: &str = file_path_string.unwrap();

                output_files.push(file_path_string_unwrapped.to_string());
            };
        };
    }

    Ok(output_files)
}

pub fn get_source_files() -> Result<Vec<String>, String>
{
    let current_terminal_directory: io::Result<PathBuf> = env::current_dir();

    if current_terminal_directory.is_err()
    {
        panic!("Error: Could not access current directory");
    };

    let current_terminal_directory_unwrapped: PathBuf = current_terminal_directory.unwrap();

    let files: Result<Vec<String>, String> = read_directory(current_terminal_directory_unwrapped);

    if files.is_err()
    {
        panic!("Error: Failed to parse files");
    };

    let files_unwrapped: Vec<String> = files.unwrap();

    for i in 0..files_unwrapped.len()
    {
        println!("Path: {}", files_unwrapped[i]);
    }

    println!("Length: {}", files_unwrapped.len());

    Ok(files_unwrapped)
}
