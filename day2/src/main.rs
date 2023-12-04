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

fn each_game2(game: &str) -> i32{
    //for each round:
    //Find highest amount of each color
    let mut red_amount: i32 = 0;
    let mut green_amount: i32 = 0;
    let mut blue_amount: i32 = 0;

    println!("{}", game);
    let substrings_game_id: Vec<&str> = game.split(':').collect();
    let substrings_split_comma: Vec<&str> = substrings_game_id[1].split(';').collect();

    for substring in &substrings_split_comma {
        //println!("round: {}", substring);
        if substring.contains(',') {
            let substring_split_color: Vec<&str> = substring.split(',').collect();
            for subtring_color in &substring_split_color {
                let digit_and_color: Vec<&str> = subtring_color.trim().split(' ').collect();
                let amount: i32 = digit_and_color[0].parse::<i32>().unwrap();
                //println!("digit and color: {}", digit_and_color[1]);
                if digit_and_color[1].contains("red") {
                    if amount > red_amount {
                        //println!("New high red amount: {}", amount);
                        red_amount = amount;
                    }
                } else if digit_and_color[1].contains("green") {
                    if amount > green_amount {
                        //println!("New high green amount: {}", amount);
                        green_amount = amount;
                    }
                } else {
                    if amount > blue_amount {
                        //println!("New high blue amount: {}", amount);
                        blue_amount = amount;
                    }
                }
            }
        } else {
            let digit_and_color: Vec<&str> = substring.trim().split(' ').collect();
            let amount: i32 = digit_and_color[0].parse::<i32>().unwrap();
            //println!("amount: {}", amount);
            if digit_and_color[1].contains("red") {
                if amount > red_amount {
                    //println!("New high red amount: {}", amount);
                    red_amount = amount;
                }
            } else if digit_and_color[1].contains("green") {
                if amount > green_amount {
                    //println!("New high green amount: {}", amount);
                    green_amount = amount;
                }
            } else {
                if amount > blue_amount {
                    //println!("New high blue amount: {}", amount);
                    blue_amount = amount;
                }
            }
        }
    }
    println!("Red amount: {}", red_amount);
    println!("Green amount: {}", green_amount);
    println!("Blue amount: {}", blue_amount);
    let sum: i32 = red_amount * green_amount * blue_amount;
    println!("Sum from game: {}", sum);
    return sum;
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
        sum = sum + each_game2(line);
    }
    println!("Sum: {}", sum);
}


