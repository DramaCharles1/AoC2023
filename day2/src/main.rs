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
    let mut char_iter = cubes.chars();
    let mut amount: i32 = 0;

    if let Some(first_char) = char_iter.next() { //funkar ej  iom att det finns värden med två siffor. tex 20
        amount = first_char as i32;
        amount = amount - 48;
        println!("amount: {}", amount);
    } else {
        println!("The string is empty.");
    }

    if cubes.contains("red") {
        if amount > MAX_RED {
            
            return false;
        } else {
            return true;
        }
    } else if cubes.contains("green") {
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

fn each_game(game: &str) {
    let substrings_game_id: Vec<&str> = game.split(':').collect();
    let char_iter = substrings_game_id[0].chars();
    if let Some(game_id) = char_iter.last() { //Some??
        println!("Game ID: {}", game_id);
    } else {
        println!("The string is empty.");
    }
    let substrings_split_comma: Vec<&str> = substrings_game_id[1].split(';').collect();

    for substring in &substrings_split_comma {
        if substring.contains(',') {
            let substring_split_color: Vec<&str> = substring.split(',').collect();
            for subtring_color in &substring_split_color {
                if !check_color_amount(&subtring_color.trim()) {
                    println!("This was an impossible game")
                } else {
                    println!("This was a possible game")
                }
            }
        }
        println!("Substring: {}", substring.trim());
    }
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

    for line in &read_lines {
        each_game(line);
    }
    //Max red = 12
    //Max green = 13
    //Max blue = 14


}


