use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to read a file line by line and return a vector of strings
fn read_file_lines(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a vector to store the lines
    let mut lines = Vec::new();

    // Create a buffered reader to efficiently read the file
    let reader = io::BufReader::new(file);

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Add each line to the vector
        lines.push(line?);
    }

    Ok(lines)
}

fn main() {
    // Specify the path to the file
    let file_path = "example.txt";
    let mut read_lines = Vec::new();
    let string_vector: Vec<String> = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];

    // Call the custom function to read the file lines
    match read_file_lines(file_path) {
        Ok(lines) => {
            read_lines = lines
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    for line in &read_lines {
        println!("{}", line);
        //get first digit or spelled digit
    }
}