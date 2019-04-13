use std::{env, fs};

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

    for archive in archives_list {
        println!("started for archive loop");
        let mut archive_name = archive.to_string();
        let original_file_name = archive.to_string();
        let archive_name_length = archive_name.len();
        archive_name.truncate(archive_name_length - 4);
        archive_name = format!("{}{}", archive_name, ".cbz");
        println!("Truncated archive name: {}", archive_name);
        fs::rename(original_file_name, archive_name).expect("error renaming a file");
    }
}
