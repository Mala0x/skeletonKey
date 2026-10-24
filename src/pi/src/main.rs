use std::io;

use colored::Colorize;

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

fn launch_applications_menu() {
    println!("We want to launch an application!");
}

fn some_random_funny() {
    println!("We want to do something random funny!");
}

fn power_off_device() {
    println!("We want to power off the device!");
}

fn main() {

    print_main_menu();

    let user_choice_optional = get_user_choice();

    if user_choice_optional == None {
        eprintln!("There was no user choice! Quitting");
        return;
    }

    let user_choice_sanitized_result = sanitize_user_input(&user_choice_optional.unwrap());

    if let Ok(user_input_numerical) = user_choice_sanitized_result {
        match user_input_numerical {
            1 => launch_applications_menu(),
            2 => some_random_funny(),
            3 => power_off_device(),
            _ => {
                eprintln!("An invalid option was given. Quitting!");
                main(); // TODO: check how you would replace this with another function call that does not overflow the stack within a microsecond :)
                return;
            }
        } // We matched everything when someone does not enter a valid number we just quit the program!
    }

}