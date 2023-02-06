use crossterm::style::Stylize;

pub mod compile;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("\n{}\n", "Usage".bold());
        println!("javai <command>");
        println!("\n{}\n", "Commands".bold());
        println!("compile (c):    Compile the java files in the src folder\n                (or otherwise specified folder)\n");
        println!("run (r):        Run the java class containing the main function\n                in the bin folder (or otherwise specified folder)\n");
    };

    let command: &String = &args[1];

    if command == "compile" || command == "c" {
        compile::get_source_files();
    };
    if command == "run" || command == "r" {};
}
