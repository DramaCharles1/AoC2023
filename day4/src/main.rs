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

    let mut points: i32 = 0;
    let mut sum: i32 = 0;
    for line in &read_lines {
        //println!("{}", line);
        let line_card: Vec<&str> = line.split(':').collect();
        //println!("line_card: {}", line_card[0]);
        let all_numbers: Vec<&str> = line_card[1].split('|').collect();
        let winning_numbers: Vec<&str> = all_numbers[0].split(' ').collect();
        let my_numbers: Vec<&str> = all_numbers[1].split(' ').collect();

        for i in 0..winning_numbers.len() {
            //println!("Check if winning number {} is in my numbers", winning_numbers[i]);
            if winning_numbers[i] != "" && my_numbers.contains(&winning_numbers[i]) {
                //println!("winning number {} is in my numbers", winning_numbers[i]);
                if points == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }
        //println!("Points from card: {}", points);
        sum = sum + points;
        points = 0;
    }
    println!("Total points: {}", sum);
}
//Check if winning number is in number I have
//  for each card
    //  if winning number n is in number I have
    //      if points == 0
    //          points = 1
    //      else
    //          points = points * 2
//  sum = sum + points
