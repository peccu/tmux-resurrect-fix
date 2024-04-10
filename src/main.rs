use std::env;
use std::os::unix::fs::symlink;

mod utils;

fn main() {
    let target_dir = std::env::args().nth(1).expect("Please provide the target directory as the first argument");
    let link_name = std::env::args().nth(2).expect("Please provide the link name as the second argument");
    let min_lines = 4;

    let newest_file = utils::get_newest_file(&target_dir, min_lines);

    if let Some(file_path) = newest_file {
        let file_name = file_path.file_name().expect("Failed to get file name");
        let file_name_str = file_name.to_str().expect("Failed to convert file name to string");

        env::set_current_dir(&target_dir).expect("Failed to change directory");
        symlink(&file_name, &link_name).expect("Failed to create symbolic link");

        println!("Created symbolic link '{}' pointing to '{}'", link_name, file_name_str);
    } else {
        println!("No suitable files found in the target directory");
    }
}
