use std::fs::File;
use std::io::{self, BufRead};

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

struct map {
    destination: Vec<int>,
    source: Vec<int>,
    range: i32,
}

fn main() {
    let mut read_lines = Vec::new();
    match read_file_lines("example.txt") {
        Ok(lines) => { //behäver man alltid hantera ok och err när man kallat på en fn?
            read_lines = lines
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    let seeds: Vec<i32> = Vec<i32>::new();
    let seed_to_soil_destination = Vec<i32>::new();
    let seed_to_soil_source = Vec<i32>::new();
    let seed_to_soil_range: i32 = 0; 

    for i in 0..read_lines.len() {
        let line: &String = read_lines[i];
        println!(read_lines[i]);
        if line.contains("seeds:") {
            //Split line acc to seeds
        } 
        if read_lines[i].contains("seed-to-soil map:") {
            println!("Next lines are seed-to-soil")

        }
    }
}

//part 1:
//goal: so they'd like to know the closest location that needs a seed.
//Map source to destination trough each map to get location for each seed

