use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_newest_file(target_dir: &str, min_lines: usize) -> Option<PathBuf> {
    let mut file_heap: BinaryHeap<(Reverse<u64>, PathBuf)> = BinaryHeap::new();
    for entry in fs::read_dir(target_dir).expect("Failed to read target directory") {
        let file_path = entry.expect("Failed to get file path").path();
        if file_path.is_file() {
            let modified_time = file_path.metadata().expect("Failed to get file metadata")
                .modified().expect("Failed to get modification time")
                .elapsed().expect("Failed to get elapsed time")
                .as_secs();
            file_heap.push((Reverse(modified_time), file_path));
        }
    }

    while let Some((_, file_path)) = file_heap.pop() {
        let line_count = count_lines(&file_path);
        if line_count >= min_lines {
            return Some(file_path);
        }
    }

    None
}

fn count_lines(file_path: &Path) -> usize {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .count()
}
