use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

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

fn check_if_value_exist_in_map(map: &HashMap<i32, i32>, check: &i32) -> bool {
    return map.contains_key(&check);
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

    let mut seed_soil_map: HashMap<i32, i32>=HashMap::new();
    let mut soil_fertilizer_map: HashMap<i32, i32>=HashMap::new();
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

    let mut max_seed_nmbr: i32 = 0;
    for i in seed_soil_start..soil_fertilizer_start - 2 {
        //split line on " "
        let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
        println!("::::::::::[DEBUG]::::::::");
        for item in &source_dest_range {
            println!("element after split: {}", item);
        }
        let mut destination: i32 = source_dest_range[0].parse().unwrap();
        let mut source: i32 = source_dest_range[1].parse().unwrap();
        let range: i32 = source_dest_range[2].parse().unwrap();

        if (source + range - 1) > max_seed_nmbr {
            max_seed_nmbr = source + range - 1;
        } 

        for j in 0..range {
            //source  += j;
            //destination += j;
            //println!("Add seed {} with soil {}", source + j, destination + j);
            seed_soil_map.insert(source + j, destination + j);
        }
    }

    for i in 0..max_seed_nmbr {
        if !seed_soil_map.contains_key(&i) {
            //println!("map does not contain: {}", i);
            seed_soil_map.insert(i, i);
        }
    }

    let mut test_seed = 50;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 98;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 99;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 79;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 14;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 55;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 13;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);


}


//part 1:
//goal: so they'd like to know the closest location that needs a seed.
//Map source to destination trough each map to get location for each seed
//Create list of seed numbers corresponding to soil numbers
//Create source list from number to number + range
//Create destination list from number to number + range
//hashmap key: int (source) value: int (destination)
//Seed-to-soil
//destination source range
//50 98 2
//for 0..range
    //add key 50 to hashmap with value 98
    //add key 51 to hasmap with value 99

