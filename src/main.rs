use std::{env, fs, io};

fn main() {
    let current_path = env::current_dir().expect("error getting current directory");
    let current_dir = current_path
        .into_os_string()
        .into_string()
        .expect("error converting current path to string");
    let mut archives_list = vec![];
    for entry in fs::read_dir(&current_dir).expect("error reading current directory") {
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

    let leave_original_file = true;
    println!(
        "Would you like to leave the original file? Y/N /n Currently set to: {}",
        leave_original_file.to_string() // accept input
    );

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("Setting to {} ", input);
            if input == "N" {
                *leave_original_file = false;
            }
        }
        Err(error) => println!("error: {}", error),
    }

    for archive in archives_list {
        println!("Processing: {}", archive.to_string());
        let mut archive_name = archive.to_string();
        let original_file_name = archive.to_string();
        let archive_name_length = archive_name.len();
        archive_name.truncate(archive_name_length - 4);
        archive_name = format!("{}{}", archive_name, ".cbz");
        println!("Truncated archive name: {}", archive_name);

        if leave_original_file == true {
            fs::copy(original_file_name, archive_name);
        } else if leave_original_file == false {
            fs::rename(original_file_name, archive_name).expect("error renaming a file");
            println!("deleted original");
        }
    }
}
