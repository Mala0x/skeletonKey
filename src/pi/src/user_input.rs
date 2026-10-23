// use colored::{Colorize};
// use std::{env::home_dir, fs, io};
// use crate::{common};

// fn get_user_choice_stdin() -> io::Result<String> {
//     let mut buffer = String::new();
//     let stdin = io::stdin();
//     stdin.read_line(&mut buffer)?;
//     Ok(buffer)
// }

// fn get_user_app_choice_pretty(stdin_string: String) -> Result<usize, &'static str> {
//     let cleaned = stdin_string.trim();

//     if let Some(hex_str) = cleaned.strip_prefix("0x") {
//         return usize::from_str_radix(hex_str, 16)
//             .mapDirEntry> apps_err(|_| "Error with converting from stdin string to int!");
//     }

//     cleaned.parse::<usize>().map_err(|_| "The conversion from stdin string to int failed!")
// }

// fn get_applications() -> Result<Vec<std::fs::DirEntry>, u16> {
    
//     let Some(home_dir_buf) = home_dir() else {
//         eprintln!("I could not find your home directroy!");
//         return Err(common::HOME_FOLDER_NOT_FOUND);
//     };

//     let apps_path = home_dir_buf.join("Documents/skeletonKey/apps");

//     let entries = match fs::read_dir(&apps_path) {
//         Ok(entries) => entries,
//         Err(err) => {
//             eprintln!("The folder with apps does not exist! Err: {err}");
//             println!("Please create the folder ~/Documents/skeletonKey/apps");
//             return Err(common::APPLICATION_FOLDER_NOT_FOUND);
//         }
//     };

//     let apps: Vec<std::fs::DirEntry> = entries.filter_map(|e| e.ok()).collect();

//     if apps.is_empty() {
//         eprintln!("No files found in the apps folder!");
//         return Err(common::NO_FILES_IN_APPS_FOLDER);
//     }

//     Ok(apps)
// }

// pub fn print_and_parse_() -> Result<std::fs::DirEntry, u16> {
    
//     let logo = "
//     ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣧⣶⣶⣶⣦⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⣠⣾⢿⣿⣿⣿⣏⠉⠉⠛⠛⠿⣷⣕⠀⠀⠀⠀⠀⠀⢀⡀
// ⠀⠀⠀⠀⣠⣾⢝⠄⢀⣿⡿⠻⣿⣄⠀⠀⠀⠀⠈⢿⣧⡀⣀⣤⡾⠀⠀⠀
// ⠀⠀⠀⢰⣿⡡⠁⠀⠀⣿⡇⠀⠸⣿⣾⡆⠀⠀⣀⣤⣿⣿⠋⠁⠀⠀⠀⠀
// ⠀⠀⢀⣷⣿⠃⠀⠀⢸⣿⡇⠀⠀⠹⣿⣷⣴⡾⠟⠉⠸⣿⡇⠀⠀⠀⠀⠀
// ⠀⠀⢸⣿⠗⡀⠀⠀⢸⣿⠃⣠⣶⣿⠿⢿⣿⡀⠀⠀⢀⣿⡇⠀⠀⠀⠀⠀
// ⠀⠀⠘⡿⡄⣇⠀⣀⣾⣿⡿⠟⠋⠁⠀⠈⢻⣷⣆⡄⢸⣿⡇⠀⠀⠀⠀⠀
// ⠀⠀⠀⢻⣷⣿⣿⠿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠻⣿⣷⣿⡟⠀⠀⠀⠀⠀⠀
// ⢀⣰⣾⣿⠿⣿⣿⣾⣿⠇⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣅⠀⠀⠀⠀⠀⠀
// ⠀⠰⠊⠁⠀⠙⠪⣿⣿⣶⣤⣄⣀⣀⣀⣤⣶⣿⠟⠋⠙⢿⣷⡄⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢀⣿⡟⠺⠭⠭⠿⠿⠿⠟⠋⠁⠀⠀⠀⠀⠙⠏⣦⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢸⡟⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//     ".red();

//     println!("{}", logo);
//     println!("Welcome to your system sir");

//     let user_choice_numerical: usize;
//     let application_vec: Vec<std::fs::DirEntry>;

//     match get_applications() {
//         Ok(apps) => application_vec = apps,
//         Err(_e) => {
//             eprintln!("Could not get and print application!");
//             return Err(common::COULD_NOT_GET_AND_PRINT_APPS);
//         },
//     }

//     for (index, entry) in application_vec.iter().enumerate() {
//         println!("Option {:#x}: {}", index, entry.file_name().to_string_lossy());
//     }
    
//     match get_user_choice_stdin() {
//         Ok(raw_input) => match get_user_app_choice_pretty(raw_input) {
//             Ok(choice) => user_choice_numerical = choice,
//             Err(e) => { 
//                 eprintln!("Invalid selection: {e}");
//                 return Err(common::INVALID_APP_SELECTION);
//             },
//         },
//         Err(e) => {
//             eprintln!("Failed to read input: {e}");
//             return Err(common::FAILED_TO_READ_USER_INPUT);
//         },
//     }

// }