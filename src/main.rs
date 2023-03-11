use std::{env, fs, io};
use std::ffi::OsString;
use std::path::PathBuf;

use simple_config_parser::Config;

struct ConfigSettings {
    config_file_exists: bool,
    leave_original_file: bool,
    folder_for_each_archive: bool,
    exit: bool,
}

impl ConfigSettings {
    fn new(config_file_exists: bool) -> ConfigSettings {
        ConfigSettings {
            config_file_exists,
            leave_original_file: true,
            folder_for_each_archive: false,
            exit: false,
        }
    }
}

fn get_settings() -> ConfigSettings {
    match Config::new().file("config.txt") {
        Err(config_error) => ConfigSettings::new(false),
        Ok(config_file) => {
            let leave_original_file = config_file.get::<bool>("leave-original-file");
            let folder_for_each_archive = config_file.get::<bool>("folder-for-each-archive");

            if leave_original_file.is_err() | folder_for_each_archive.is_err() {
                ConfigSettings::new(false)
            } else {
                ConfigSettings {
                    config_file_exists: true,
                    leave_original_file: leave_original_file.unwrap(),
                    folder_for_each_archive: folder_for_each_archive.unwrap(),
                    exit: false,
                }
            }
        }
        _ => ConfigSettings::new(false),
    }
}

fn scan_directory(current_dir: &String) -> Vec<String> {
    let mut archives_list = vec![];

    for entry in fs::read_dir(current_dir).expect("error occurred while trying to scan directory") {
        let entries = entry.expect("error adding file name to list");
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
            Ok(_) => match input.to_uppercase().trim() {
                "N" => {
                    leave_original_file = false;
                    println!("original files will not be kept");
                    choice_made = true;
                }
                "Y" => {
                    println!("original files will be kept");
                    choice_made = true;
                }
                "E" => {
                    exit = true;
                }
                _ => {
                    input.clear();
                    println!("invalid input detected");
                    println!("would you like to leave the original files? Y/N");
                }
            },
            Err(inp_err) => {
                println!("error while reading input: {}", inp_err);
                exit = true;
            }
        }
    }
    (leave_original_file, exit)
}

fn get_current_directory() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn process_zip_files(
    archive_list: Vec<String>,
    leave_original_file: bool,
    folder_for_each_archive: bool,
) {
    println!("there are: {} archives to process", archive_list.len());

    for archive in archive_list {
        let original_archive_name = archive.clone();
        let mut new_archive_name = archive.clone();
        new_archive_name.truncate(new_archive_name.len() - 4);
        let final_folder_name = new_archive_name.clone();
        new_archive_name = format!("{}{}", new_archive_name, ".cbz");
        println!("new archive name: {new_archive_name}");

        if leave_original_file {
            if folder_for_each_archive {
                fs::create_dir(&final_folder_name).expect("unable to create folder");
                let final_dir_and_name = format!("{}/{}", &final_folder_name, &new_archive_name);
                match fs::copy(original_archive_name, final_dir_and_name) {
                    Ok(_ok) => {}
                    Err(error) => println!("error while copying: {error}"),
                }
            } else {
                match fs::copy(original_archive_name, new_archive_name) {
                    Ok(_ok) => {}
                    Err(error) => println!("error while copying: {error}"),
                }
            }
        } else if !leave_original_file {
            if folder_for_each_archive {
                fs::create_dir(final_folder_name).expect("unable to create folder");
                let final_dir_and_name = format!("{}/{}", &new_archive_name, &new_archive_name);
                match fs::rename(original_archive_name, final_dir_and_name) {
                    Ok(_ok) => {}
                    Err(error) => println!("error while renaming archive: {error}"),
                }
                println!("original file removed");
            } else {
                match fs::rename(original_archive_name, new_archive_name) {
                    Ok(_ok) => {}
                    Err(error) => println!("error while renaming archive: {error}"),
                }
                println!("original file removed");
            }
        }
    }
    println!("done processing archives");
}

fn main() {
    let config_settings = get_settings();

    if config_settings.config_file_exists {
        let leave_original_file = config_settings.leave_original_file;
        let folder_for_each_archive = config_settings.folder_for_each_archive;

        match get_current_directory() {
            Ok(current_directory) => {
                let archive_list = scan_directory(current_directory);

                if archive_list.is_empty() {
                    println!("no archives detected")
                } else {
                    process_zip_files(archive_list, leave_original_file, folder_for_each_archive);
                }
            }
            Err(error) => {
                println!("encountered error while trying to get current directory");
                println!("error: {error}");
            }
        }
    } else {
        let (leave_original_file, exit_program) = get_input();

        if exit_program {
            return;
        }

        match get_current_directory() {
            Ok(current_directory) => {
                let archive_list = scan_directory(current_directory);

                if archive_list.is_empty() {
                    println!("no archives detected")
                } else {
                    process_zip_files(archive_list, leave_original_file, folder_for_each_archive);
                }
            }
            Err(error) => {
                println!("encountered error while trying to get current directory");
                println!("error: {error}");
            }
        }
    }
}
