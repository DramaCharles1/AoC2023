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

struct Map {
    destination: Vec<i32>,
    source: Vec<i32>,
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

    let mut seed_start: usize = 1;
    let mut seed_soil_start: usize = 0;
    let mut soil_fertilizer_start: usize = 0;
    let mut fertilizer_water_start: usize = 0;
    let mut water_light_start: usize = 0;
    let mut light_temperature_start: usize = 0;
    let mut temperature_humidity_start: usize = 0;
    let mut humidity_location_start: usize = 0;

    for i in 0..read_lines.len() {
        let line: &String = &read_lines[i];
        //println!("{}",read_lines[i]);
        if read_lines[i].contains("seed-to-soil map:") {
            //Seed to soil starts at i + 1
            seed_soil_start = i + 1;
        }
        else if read_lines[i].contains("soil-to-fertilizer map:") {
            soil_fertilizer_start = i + 1;
        }
        else if read_lines[i].contains("fertilizer-to-water map:") {
            //Seed to soil starts at i + 1
            fertilizer_water_start = i + 1;
        }
        else if read_lines[i].contains("water-to-light map:") {
            //Seed to soil starts at i + 1
            water_light_start = i + 1;
        }
        else if read_lines[i].contains("light-to-temperature map:") {
            //Seed to soil starts at i + 1
            light_temperature_start = i + 1;
        }
        else if read_lines[i].contains("temperature-to-humidity map:") {
            //Seed to soil starts at i + 1
            temperature_humidity_start = i + 1;
        }
        else if read_lines[i].contains("humidity-to-location map:") {
            //Seed to soil starts at i + 1
            humidity_location_start = i + 1;
        }
    }

    let seeds: Vec<&str> = read_lines[seed_start].split(" ").collect();
    for seed in seeds {
        println!("Seed: {}", seed);
    }

}

//part 1:
//goal: so they'd like to know the closest location that needs a seed.
//Map source to destination trough each map to get location for each seed

