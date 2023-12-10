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

struct CardResult {
    matches: usize,
    amount: usize,
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

    let mut card_results: Vec<CardResult> = Vec::new();
    let mut points: i32 = 0;
    let mut sum: i32 = 0;
    let mut matches: usize = 0;
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
                    matches = 1;
                } else {
                    points = points * 2;
                    matches = matches + 1;
                }
            }
        }
        card_results.push(CardResult { matches: matches, amount: 1});
        //println!("Points from card 1: {}", points);
        sum = sum + points;
        points = 0;
        matches = 0;
    }
    println!("Total points from part 1: {}", sum);

    for i in 0..card_results.len() {
        //println!("Matches from card {}: {}", i, card_results[i].matches);
        for _amount in 0 .. card_results[i].amount {
            //println!("Amount {} of card {}", card_results[i].amount, i);
            for j in i + 1.. i + 1 + card_results[i].matches {
                //println!("Add card {}", j);
                card_results[j].amount = card_results[j].amount + 1;
            }
        }
    }

    let mut amount = 0;
    for i in 0..card_results.len() {
        //println!("Amount of card {}: {}", i + 1, card_results[i].amount);
        amount = amount + card_results[i].amount;
    }
    println!("Total amount of card from part 2: {}", amount);
}
//Check if winning number is in number I have
//  for each card
    //  if winning number n is in number I have
    //      if points == 0
    //          points = 1
    //      else
    //          points = points * 2
//  sum = sum + points

//part 2
//save result from each card: [points, matches]
//loop trough all cards
    //sum = sum + points
    //loop trough matches
        //sum = sum + points