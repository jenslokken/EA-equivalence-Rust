use std::fs;

pub fn parse_input(path: &str) -> (u32, Vec<u32>) {
    println!("path: {}", path);
    let numbers: String = fs::read_to_string(path)
        .expect("File not found");
    let numbers: Vec<u32> = numbers
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid number in the file"))
        .collect();
    return (numbers[0], numbers[1..].to_vec());
}