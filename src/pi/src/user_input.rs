use colored::{Colorize};
use std::{fs, io};

fn get_user_choice_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn get_user_app_choice_pretty(stdin_string: String) -> Result<u16, &'static str> {
    let cleaned = stdin_string.trim();

    if let Some(hex_str) = cleaned.strip_prefix("0x") {
        return u16::from_str_radix(hex_str, 16)
            .map_err(|_| "Error with converting from stdin string to int!");
    }

    cleaned.parse::<u16>().map_err(|_| "The conversion from stdin string to int failed!")
}

fn print_applications() {
    
    let entries = match fs::read_dir("/home/mala/Documents/skeletonKey/apps") {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("The folder with apps does not exist! Err: {err}");
            return;
        }
    };

    for (index, entry) in entries.enumerate() {
        if let Ok(entry) = entry {
            println!("Option {:#x}: {}", index, entry.file_name().to_string_lossy());
        }
    }

}

pub fn print_main_menu() {
    
    let logo = "
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣧⣶⣶⣶⣦⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣠⣾⢿⣿⣿⣿⣏⠉⠉⠛⠛⠿⣷⣕⠀⠀⠀⠀⠀⠀⢀⡀
⠀⠀⠀⠀⣠⣾⢝⠄⢀⣿⡿⠻⣿⣄⠀⠀⠀⠀⠈⢿⣧⡀⣀⣤⡾⠀⠀⠀
⠀⠀⠀⢰⣿⡡⠁⠀⠀⣿⡇⠀⠸⣿⣾⡆⠀⠀⣀⣤⣿⣿⠋⠁⠀⠀⠀⠀
⠀⠀⢀⣷⣿⠃⠀⠀⢸⣿⡇⠀⠀⠹⣿⣷⣴⡾⠟⠉⠸⣿⡇⠀⠀⠀⠀⠀
⠀⠀⢸⣿⠗⡀⠀⠀⢸⣿⠃⣠⣶⣿⠿⢿⣿⡀⠀⠀⢀⣿⡇⠀⠀⠀⠀⠀
⠀⠀⠘⡿⡄⣇⠀⣀⣾⣿⡿⠟⠋⠁⠀⠈⢻⣷⣆⡄⢸⣿⡇⠀⠀⠀⠀⠀
⠀⠀⠀⢻⣷⣿⣿⠿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠻⣿⣷⣿⡟⠀⠀⠀⠀⠀⠀
⢀⣰⣾⣿⠿⣿⣿⣾⣿⠇⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣅⠀⠀⠀⠀⠀⠀
⠀⠰⠊⠁⠀⠙⠪⣿⣿⣶⣤⣄⣀⣀⣀⣤⣶⣿⠟⠋⠙⢿⣷⡄⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣿⡟⠺⠭⠭⠿⠿⠿⠟⠋⠁⠀⠀⠀⠀⠙⠏⣦⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢸⡟⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ".red();

    println!("{}", logo);
    println!("Welcome to your system sir");

    print_applications();
    
    match get_user_choice_stdin() {
        Ok(raw_input) => match get_user_app_choice_pretty(raw_input) {
            Ok(choice) => println!("Selected: {choice:#x}"),
            Err(e) => eprintln!("Invalid selection: {e}"),
        },
        Err(e) => eprintln!("Failed to read input: {e}"),
    }

}