use std::fs;

pub fn read_file(path: &str, keep_empty_lines: bool) -> Vec<String> {
    let mut file = fs::read_to_string(path);
    if file.is_err() {
        panic!("Could not read file {}", path);
    }

    return file
        .unwrap()
        .split("\n")
        .filter(|x| keep_empty_lines || x.len() > 0)
        .map(|x| x.parse::<String>().unwrap())
        .collect();
}
