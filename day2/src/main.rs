use std::fs::File;
use std::io::{self, BufRead};
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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

fn check_color_amount(cubes: &str) -> bool{
    //println!("cubes: {}", cubes);
    let digit_and_color: Vec<&str> = cubes.split(' ').collect();
    let amount: i32 = digit_and_color[0].parse::<i32>().unwrap();

    if digit_and_color[1].contains("red") {
        if amount > MAX_RED {
            return false;
        } else {
            return true;
        }
    } else if digit_and_color[1].contains("green") {
        if amount > MAX_GREEN {
            return false;
        } else {
            return true;
        }
    } else {
        if amount > MAX_BLUE {
            return false;
        } else {
            return true;
        }
    }

}

fn each_game(game: &str) -> i32{
    println!("{}", game);
    let substrings_game_id: Vec<&str> = game.split(':').collect();
    let game_id: i32 = substrings_game_id[0].replace("Game ","").parse::<i32>().unwrap();
    //println!("game ID: {}", game_id);

    let substrings_split_comma: Vec<&str> = substrings_game_id[1].split(';').collect();

    for substring in &substrings_split_comma {
        //println!("Round: {}", substring.trim());
        if substring.contains(',') {
            let substring_split_color: Vec<&str> = substring.split(',').collect();
            for subtring_color in &substring_split_color {
                if !check_color_amount(&subtring_color.trim()) {
                    println!("This was an impossible game");
                    return 0;
                }
            }
        } else {
            if !check_color_amount(&substring.trim()) {
                println!("This was an impossible game");
                return 0;
            }
        }
    }
    return game_id;
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

    let mut sum: i32 = 0;
    for line in &read_lines {
        sum = sum + each_game(line);
    }
    println!("Sum: {}", sum);
    //Max red = 12
    //Max green = 13
    //Max blue = 14


}


