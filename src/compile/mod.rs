use regex::Regex;

pub fn get_source_files() -> Result<Vec<&'static str>, String> {
    /* Get the path of the current directory of the terminal window */
    let current_directory_path: std::io::Result<std::path::PathBuf> = std::env::current_dir();

    /* Check if the program was able to access the path of the current terminal window */
    if current_directory_path.is_err() {
        Err("Error: Could not access current directory");
    };

    /* Get the list of files currently in the terminal path */
    let current_directory_files: std::io::Result<std::fs::ReadDir> =
        std::fs::read_dir(current_directory_path.unwrap());

    /* Check if the program was able t access the list of files in the terminals path */
    if current_directory_files.is_err() {
        Err("Error: Could not read the current directory");
    };

    let java_regex: Regex = Regex::new(r"(?i)^.+\.java$").unwrap();
    let mut source_files: Vec<&str> = Vec::new();

    for file in current_directory_files {
        let file_path: &str = file.unwrap().path().as_path().to_str().unwrap();

        if java_regex.is_match(file_path) {
            source_files.push(file_path);
        }

        println!("File: {}", file.unwrap().path().display());
    }

    Ok(source_files)
}
