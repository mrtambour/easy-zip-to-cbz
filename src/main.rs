use std::{env, fs, io};

fn scan_directory(current_dir: &String) -> Vec<String> {
    let mut archives_list = vec![];
    for entry in fs::read_dir(current_dir).expect("error reading current directory") {
        let entries = entry.expect("error copying entry variable");
        let file_name = entries
            .file_name()
            .to_str()
            .expect("error getting file name")
            .to_string();

        if file_name.ends_with(".zip") {
            archives_list.push(file_name);
        }
    }

    archives_list
}

fn get_input() -> (bool, bool) {
    let mut leave_original_file = true;
    let mut input = String::new();
    let mut choice_made = false;
    let mut exit = false;
    println!("By default original files are kept");
    println!("You can exit by entering E");
    println!("Would you like to leave the original files? Y/N");

    while !exit & !choice_made {
        match io::stdin().read_line(&mut input) {
            Ok(_ok) => {
                if input.to_uppercase().trim() == "N" {
                    leave_original_file = false;
                    choice_made = true;
                    println!("original files will not be kept");
                } else if input.to_uppercase().trim() == "Y" {
                    println!("original files will be kept");
                    choice_made = true;
                } else if input.to_uppercase().trim() == "E" {
                    exit = true;
                } else {
                    input.clear();
                    println!("invalid input detected");
                    println!("Would you like to leave the original files? Y/N");
                }
            }
            Err(error) => {
                println!("error while reading input: {}", error);
                exit = true;
            }
        }
    }
    (leave_original_file, exit)
}

fn get_current_directory() -> String {
    let current_path = env::current_dir().expect("error getting current directory");
    let current_dir = current_path
        .into_os_string()
        .into_string()
        .expect("error converting current path to string");

    current_dir
}

fn process_zip_files(archive_list: Vec<String>, leave_original_file: bool) {
    for archive in archive_list {
        println!("Processing: {}", archive);
        let mut archive_name = archive.to_string();
        let original_file_name = archive.to_string();
        let archive_name_length = archive_name.len();
        archive_name.truncate(archive_name_length - 4);
        archive_name = format!("{}{}", archive_name, ".cbz");
        println!("Truncated archive name: {}", archive_name);

        if leave_original_file {
            match fs::copy(original_file_name, archive_name) {
                Ok(_ok) => {}
                Err(error) => println!("error while copying: {}", error)
            }
        } else if !leave_original_file {
            match fs::rename(original_file_name, archive_name) {
                Ok(_ok) => {}
                Err(error) => println!("error while renaming archive: {}", error)
            }
            println!("original file removed");
        }
    }
}

fn main() {
    let (leave_original_file, exit_program) = get_input();
    let current_directory = get_current_directory();
    let archive_list = scan_directory(&current_directory);

    if exit_program {
        println!("program exiting")
    } else if archive_list.is_empty() {
        println!("no archives detected")
    } else {
        process_zip_files(archive_list, leave_original_file);
    }
}
