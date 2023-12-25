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

fn main() {
    let mut read_lines = Vec::new();
    match read_file_lines("input.txt") {
        Ok(lines) => { //behäver man alltid hantera ok och err när man kallat på en fn?
            read_lines = lines
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    let mut seed_soil_map: HashMap<i64, i64>=HashMap::new();
    let mut soil_fertilizer_map: HashMap<i64, i64>=HashMap::new();
    let mut fertilizer_water_map: HashMap<i64, i64>=HashMap::new();
    let mut water_light_map: HashMap<i64, i64>=HashMap::new();
    let mut light_temperature_map: HashMap<i64, i64>=HashMap::new();
    let mut temperature_humidity_map: HashMap<i64, i64>=HashMap::new();
    let mut humidity_location_map: HashMap<i64, i64>=HashMap::new();
    let seed_start: usize = 1;
    let mut seed_soil_start: usize = 0;
    let mut soil_fertilizer_start: usize = 0;
    let mut fertilizer_water_start: usize = 0;
    let mut water_light_start: usize = 0;
    let mut light_temperature_start: usize = 0;
    let mut temperature_humidity_start: usize = 0;
    let mut humidity_location_start: usize = 0;
    let mut stop: usize = 0;
    let max_number = 999999;

    for i in 0..read_lines.len() {
        //let line: &String = &read_lines[i];
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
        else if read_lines[i].contains("stop") {
            //Seed to soil starts at i + 1
            stop = i + 1;
        }
    }

    let seeds: Vec<&str> = read_lines[seed_start].split(" ").collect();
    // for seed in seeds {
    //     println!("Seed: {}", seed);
    // }

    //Seed to soil
    //let mut max_seed_nmbr: i32 = 0;
    print!("Seed to soil");
    for i in seed_soil_start..soil_fertilizer_start - 2 {
        //split line on " "
        let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
        // println!("::::::::::[DEBUG]::::::::");
        // for item in &source_dest_range {
        //     println!("element after split: {}", item);
        // }
        let destination: i64 = source_dest_range[0].parse().unwrap();
        let source: i64 = source_dest_range[1].parse().unwrap();
        let range: i64 = source_dest_range[2].parse().unwrap();

        // if (source + range - 1) > max_seed_nmbr {
        //     max_seed_nmbr = source + range - 1;
        // }
        insert_into_map(&mut seed_soil_map, source, destination, range);
    }

    insert_rest_of_numbers(&mut seed_soil_map, max_number);
    let mut test_seed = 50;
    // println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    // test_seed = 98;
    // println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    // test_seed = 99;
    // println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 79;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 14;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 55;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);
    test_seed = 13;
    println!("Seed: {} should be at soil: {}", test_seed, seed_soil_map[&test_seed]);


    //soil to fertilizer
    //let mut max_soil_nmbr: i32 = 0;
    for i in soil_fertilizer_start..fertilizer_water_start - 2 {

        let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
        // println!("::::::::::[DEBUG]::::::::");
        // for item in &source_dest_range {
        //     println!("element after split: {}", item);
        // }

        let destination: i64 = source_dest_range[0].parse().unwrap();
        let source: i64 = source_dest_range[1].parse().unwrap();
        let range: i64 = source_dest_range[2].parse().unwrap();

        // if (source + range - 1) > max_soil_nmbr {
        //     max_soil_nmbr = source + range - 1;
        // }
        insert_into_map(&mut soil_fertilizer_map, source, destination, range);
    }
    insert_rest_of_numbers(&mut soil_fertilizer_map, max_number);

    let mut test_soil = 81;
    println!("Soil: {} should use fertilizer: {}", test_soil, soil_fertilizer_map[&test_soil]);
    test_soil = 14;
    println!("Soil: {} should use fertilizer: {}", test_soil, soil_fertilizer_map[&test_soil]);
    test_soil = 57;
    println!("Soil: {} should use fertilizer: {}", test_soil, soil_fertilizer_map[&test_soil]);
    test_soil = 13;
    println!("Soil: {} should use fertilizer: {}", test_soil, soil_fertilizer_map[&test_soil]);

    //fertilizer to water
    for i in fertilizer_water_start..water_light_start - 2 {

        let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
        // println!("::::::::::[DEBUG]::::::::");
        // for item in &source_dest_range {
        //     println!("element after split: {}", item);
        // }

        let destination: i64 = source_dest_range[0].parse().unwrap();
        let source: i64 = source_dest_range[1].parse().unwrap();
        let range: i64 = source_dest_range[2].parse().unwrap();

        insert_into_map(&mut fertilizer_water_map, source, destination, range);
    }
    insert_rest_of_numbers(&mut fertilizer_water_map, max_number);

    let mut test_fertilizer = 81;
    println!("fertilizer: {} should use water: {}", test_fertilizer, fertilizer_water_map[&test_fertilizer]);
    test_fertilizer = 53;
    println!("fertilizer: {} should use water: {}", test_fertilizer, fertilizer_water_map[&test_fertilizer]);
    test_fertilizer = 57;
    println!("fertilizer: {} should use water: {}", test_fertilizer, fertilizer_water_map[&test_fertilizer]);
    test_fertilizer = 52;
    println!("fertilizer: {} should use water: {}", test_fertilizer, fertilizer_water_map[&test_fertilizer]);

        //water to light
        for i in water_light_start..light_temperature_start - 2 {

            let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
            // println!("::::::::::[DEBUG]::::::::");
            // for item in &source_dest_range {
            //     println!("element after split: {}", item);
            // }
    
            let destination: i64 = source_dest_range[0].parse().unwrap();
            let source: i64 = source_dest_range[1].parse().unwrap();
            let range: i64 = source_dest_range[2].parse().unwrap();
    
            insert_into_map(&mut water_light_map, source, destination, range);
        }
        insert_rest_of_numbers(&mut water_light_map, max_number);
    
        let mut test_water = 81;
        println!("water: {} should use light: {}", test_water, water_light_map[&test_water]);
        test_water = 49;
        println!("water: {} should use light: {}", test_water, water_light_map[&test_water]);
        test_water = 53;
        println!("water: {} should use light: {}", test_water, water_light_map[&test_water]);
        test_water = 41;
        println!("water: {} should use light: {}", test_water, water_light_map[&test_water]);

        //light to temperature
        for i in light_temperature_start..temperature_humidity_start - 2 {

            let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
            // println!("::::::::::[DEBUG]::::::::");
            // for item in &source_dest_range {
            //     println!("element after split: {}", item);
            // }
    
            let destination: i64 = source_dest_range[0].parse().unwrap();
            let source: i64 = source_dest_range[1].parse().unwrap();
            let range: i64 = source_dest_range[2].parse().unwrap();
    
            insert_into_map(&mut light_temperature_map, source, destination, range);
        }
        insert_rest_of_numbers(&mut light_temperature_map, max_number);
    
        let mut test_light = 74;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);
        test_light = 42;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);
        test_light = 46;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);
        test_light = 34;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);

                //light to temperature
        for i in light_temperature_start..temperature_humidity_start - 2 {

            let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
            // println!("::::::::::[DEBUG]::::::::");
            // for item in &source_dest_range {
            //     println!("element after split: {}", item);
            // }
    
            let destination: i64 = source_dest_range[0].parse().unwrap();
            let source: i64 = source_dest_range[1].parse().unwrap();
            let range: i64 = source_dest_range[2].parse().unwrap();
    
            insert_into_map(&mut light_temperature_map, source, destination, range);
        }
        insert_rest_of_numbers(&mut light_temperature_map, max_number);
    
        let mut test_light = 74;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);
        test_light = 42;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);
        test_light = 46;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);
        test_light = 34;
        println!("light: {} should use temperature: {}", test_light, light_temperature_map[&test_light]);

        //temperature to humidity
        for i in temperature_humidity_start..humidity_location_start - 2 {

            let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
            // println!("::::::::::[DEBUG]::::::::");
            // for item in &source_dest_range {
            //     println!("element after split: {}", item);
            // }
    
            let destination: i64 = source_dest_range[0].parse().unwrap();
            let source: i64 = source_dest_range[1].parse().unwrap();
            let range: i64 = source_dest_range[2].parse().unwrap();
    
            insert_into_map(&mut temperature_humidity_map, source, destination, range);
        }
        insert_rest_of_numbers(&mut temperature_humidity_map, max_number);
    
        let mut test_temperature = 78;
        println!("temperature: {} should use humidity: {}", test_temperature, temperature_humidity_map[&test_temperature]);
        test_temperature = 42;
        println!("temperature: {} should use humidity: {}", test_temperature, temperature_humidity_map[&test_temperature]);
        test_temperature = 82;
        println!("temperature: {} should use humidity: {}", test_temperature, temperature_humidity_map[&test_temperature]);
        test_temperature = 34;
        println!("temperature: {} should use humidity: {}", test_temperature, temperature_humidity_map[&test_temperature]);

        //humidity to location
        for i in humidity_location_start..stop - 2 { //??

            let source_dest_range:Vec<&str> = read_lines[i].split(" ").collect();
            // println!("::::::::::[DEBUG]::::::::");
            // for item in &source_dest_range {
            //     println!("element after split: {}", item);
            // }
    
            let destination: i64 = source_dest_range[0].parse().unwrap();
            let source: i64 = source_dest_range[1].parse().unwrap();
            let range: i64 = source_dest_range[2].parse().unwrap();
    
            insert_into_map(&mut humidity_location_map, source, destination, range);
        }
        insert_rest_of_numbers(&mut humidity_location_map, max_number);
    
        let mut test_humidity = 78;
        println!("humidity: {} should use location: {}", test_humidity, humidity_location_map[&test_humidity]);
        test_humidity = 43;
        println!("humidity: {} should use location: {}", test_humidity, humidity_location_map[&test_humidity]);
        test_humidity = 82;
        println!("humidity: {} should use location: {}", test_humidity, humidity_location_map[&test_humidity]);
        test_humidity = 35;
        println!("humidity: {} should use location: {}", test_humidity, humidity_location_map[&test_humidity]);

        //Seed-to-soil
        let mut temp: i64 = 0;
        let mut low_location: i64 = 9999999;
        for seed in seeds {
            temp = seed.parse().unwrap();
            temp = seed_soil_map[&temp];
            temp = soil_fertilizer_map[&temp];
            temp = fertilizer_water_map[&temp];
            temp = water_light_map[&temp];
            temp = light_temperature_map[&temp];
            temp = temperature_humidity_map[&temp];
            temp = humidity_location_map[&temp];
            if temp < low_location {
                low_location = temp;
            }
        }
        print!("lowest location is {}", low_location);
    }

fn insert_into_map(map: &mut HashMap<i64, i64>, source: i64, destination: i64, range: i64) {
    for i in 0..range {
        map.insert(source + i, destination + i);
    }
}

fn insert_rest_of_numbers(map: &mut HashMap<i64, i64>, max_number: i64) {
    for i in 0..max_number {
        if !map.contains_key(&i) {
            map.insert(i, i);
        }
    }
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

