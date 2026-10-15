use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::{fs::*};

use crate::common;

pub fn get_user_selection(user_choice_numerical: usize, mut application_vec: Vec<DirEntry>) -> Result<DirEntry, u16> {
    
    if user_choice_numerical >= application_vec.len() {
        eprintln!("The selected application could not be grabbed from the vector!");
        return Err(common::VECTOR_GETTING_FAILED);
    }

    Ok(application_vec.swap_remove(user_choice_numerical))
} // This is for the user_input file and for no other files

fn is_app_executable(app: DirEntry) -> bool {
    let metadata = match app.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    let permissions = metadata.permissions();
    metadata.is_file() && permissions.mode() & 0o111 != 0
}

fn clear_terminal_screen() { // This means that it is linux only for this moment
    Command::new("clear").status().expect("This should clear the terminal!");
}