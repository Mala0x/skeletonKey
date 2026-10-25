use std::{env::home_dir, fs::DirEntry, io, os::unix::process::CommandExt, process::Command};

use colored::Colorize;

use reqwest::{Error};
use serde_json;

/*
I want to first make the basics of this application in the main rust file and if it works i want to part it into other files and make it better
premature optimization and abstraction really halts the development of my application imho. Therefore this way of programming this program.
*/

/*
The flow of the program is as follows:
1. it prints the anarchy logo
2. it asks the user what it wants to do? (start a program, do some random thing (idk funny random goofy something), power off the device)
3. just kill the program and make it some kinda daemon that always runs on startup so that if the user closes the program that it opened it will automagically
start this main program again
*/

fn print_main_menu() {
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
    println!("Welcome back to your system sir.");
}

fn get_user_choice() -> Option<String> {
    let user_choices = [
        (1, "Launch application"), 
        (2, "Something random funny xD"),
        (3, "Power off device")
    ]; // Nicely formatted list of what the user can choose in the first screen

    for choice in user_choices {
        println!("{}, {}", choice.0.to_string().green(), choice.1.to_string().purple());
    } // Prints the choices the user has in a nice list in hex formatting

    let mut buffer = String::new();
    
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => return Some(buffer),
        Err(e) => {
            println!("Error! {}", e);
            return None;
        }     
    }
}

fn sanitize_user_input(user_choice: &String) -> Result<u16, &'static str> {

    match user_choice.trim_ascii().parse::<u16>() { // Both trims the default terminal fluff and parses it into the u16 type
        Ok(int) => Ok(int),
        Err(_) => Err("Converting from string to integer failed!")
    }

    // Using the u16 type is nice here because the choice can and never will be negative and i am never ever going to implement more then 2^16 options lmao

}

fn get_installed_apps() -> Result<Vec<DirEntry>, &'static str> {
    if let Some(home_dir) = home_dir() {
        // TODO: make this a non hardcoded path string
        let application_folder = std::fs::read_dir(home_dir.as_path().join("Documents/Programming/skeletonKey/apps"));

        let mut application_vec: Vec<DirEntry> = vec![];
        
        // Insert multiple checks if a folder actually exists and stuff
        for (index, app) in application_folder.unwrap().enumerate() {
            application_vec.insert(index, app.unwrap());
        }
        return Ok(application_vec);
    } else {
        return Err("I could not get applications in the default application folder!");
    }
}

fn launch_applications_menu() {

    _= Command::new("clear").status();

    println!("Hello this is a list of all the launchable programs that are installed on the system at this moment!");
    
    if let Ok(installed_apps) = get_installed_apps() {

        for (index, app) in installed_apps.iter().enumerate() {
            println!("{}: {}", index, app.file_name().into_string().unwrap());
        }

        println!("What application do you want to launch: ");

        let mut buffer = String::new();

        _= io::stdin().read_line(&mut buffer).unwrap();

        let user_choice = buffer.trim().parse::<usize>();

        if let Ok(user_choice_numerical) = user_choice {
            match Command::new(installed_apps[user_choice_numerical].path()).status() { // TODO: panics now because OOB error
                Ok(status) => {
                    println!("Exit status: {}", status);
                    println!("Returning to the main menu!");
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
    // TODO:
}

fn get_location_international_space_station() -> Option<(f32, f32)> {

    let request_url = "http://api.open-notify.org/iss-now.json".to_string();
    println!("Going to make a request to: {}", request_url);

    let client = reqwest::blocking::Client::new();
    let response = client.get(request_url).send();

    if let Ok(response) = response {
        let data = response.text().unwrap();
        println!("{}", data);
        let parsed_json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let lat = parsed_json["iss_position"]["latitude"]
        .as_str()?
        .parse::<f32>()
        .ok()?;
        let long = parsed_json["iss_position"]["longitude"]
        .as_str()?
        .parse::<f32>()
        .ok()?;
        // println!("Long: {}, Lat: {}", parsed_json["iss_position"]["latitude"], parsed_json["iss_position"]["longitude"]);
        Some((lat, long))
    } else {
        None
    }
}

fn some_random_funny() {

    // What funny things can happen?
    println!("Long: {}, Lat: {}", get_location_international_space_station().unwrap().0, get_location_international_space_station().unwrap().1);

    println!("Those are the exact cordinates of the internation space station :x");
}

fn power_off_device(running: &mut bool) {
    println!("We want to power off the device!");
    println!("This now just kills the program because powering off my pc for a silly test is fairly impractical lmao");
    *running = false;
}

fn main() {

    let mut running: bool = true;

    while running {

        print_main_menu();

        let user_choice_optional = get_user_choice();

        if user_choice_optional == None {
            eprintln!("There was no user choice!");
            continue;
        }

        let user_choice_sanitized_result = sanitize_user_input(&user_choice_optional.unwrap());

        if let Ok(user_input_numerical) = user_choice_sanitized_result {
            match user_input_numerical {
                1 => launch_applications_menu(),
                2 => some_random_funny(),
                3 => power_off_device(&mut running),
                _ => {
                    eprintln!("An invalid option was given. Quitting!");
                    continue;
                }
            } // We matched everything when someone does not enter a valid number we just quit the program!
        }

    }
}