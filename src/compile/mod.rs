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
    let mut output_files: Vec<String> = Vec::new();

    for file in files_unwrapped
    {
        if file.is_err()
        {
            panic!("Error: Failed to read file");
        };

        let file_unwrapped: DirEntry = file.unwrap();

        if file_unwrapped.path().is_dir()
        {
            let sub_files: Result<Vec<String>, String> = read_directory(file_unwrapped.path());

            if sub_files.is_err()
            {
                panic!("Error: Failed to read sub directory");
            };

            let mut sub_files_unwrapped: Vec<String> = sub_files.unwrap();

            output_files.append(&mut sub_files_unwrapped);
        };

        if file_unwrapped.path().is_file()
        {
            let file_path: PathBuf = file_unwrapped.path();
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

/*
pub fn get_source_files() -> Result<Vec<&'static str>, String>
{
    /* Get the path of the current directory of the terminal window */
    let current_directory_path: std::io::Result<std::path::PathBuf> = std::env::current_dir();

    /* Check if the program was able to access the path of the current terminal window */
    if current_directory_path.is_err()
    {
        panic!("Error: Could not access current directory");
    };

    /* Get the list of files currently in the terminal path */
    let current_directory_files: std::io::Result<std::fs::ReadDir> =
        std::fs::read_dir(current_directory_path.unwrap());

    /* Check if the program was able t access the list of files in the terminals path */
    if current_directory_files.is_err()
    {
        panic!("Error: Could not read the current directory");
    };

    //let java_regex: Regex = Regex::new(r"(?i)^.+\.java$").unwrap();
    //let mut source_files: Vec<&str> = Vec::new();

    let source_files: Vec<&str> = Vec::new();

    let files: std::fs::ReadDir = current_directory_files.unwrap();

    for file_wrapped in files
    {
        if file_wrapped.is_err()
        {
            continue;
        };

        let file: std::fs::DirEntry = file_wrapped.unwrap();
        let file_path: std::path::PathBuf = file.path();

        if file_path.extension().unwrap() == "java"
        {
            let file_path_string = file_path.
        }
    }

    /*for i in 0..files.count()
    {
        let file: std::option::Option<std::io::Result<std::fs::DirEntry>> = files.nth(i);
    }*/

    /*for file in  {
        let file_path: &str = file.unwrap().path().as_path().to_str().unwrap();

        if java_regex.is_match(file_path) {
            source_files.push(file_path);
        }

        println!("File: {}", file.unwrap().path().display());
    }*/

    Ok(source_files)
}
*/
