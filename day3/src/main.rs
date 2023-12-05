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

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut row: i32 = 0;
    for line in &read_lines {
        row = row + 1;
        println!("{}", line);

        let col_array: Vec<char> = line.chars().collect();
        matrix.push(col_array)
    }

    let mut sum: i32 = 0;
    let number_array: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let not_part_array: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let mut part_number_flag: bool = false;
    let mut new_number: Vec<char> = Vec::new();
    println!("y len: {}", matrix[0].len());
    for y in 0..matrix[0].len() {
        for x in 0..matrix[0].len() {
            //println!("({},{})", y,x);
            if number_array.contains(&matrix[y][x]) {
                //println!("({},{}) contains number: {}", y,x,matrix[y][x]);
                new_number.push(matrix[y][x]);
                if y > 0 && x > 0 && !not_part_array.contains(&matrix[y - 1][x - 1]) {
                    //println!("part number flag true case 1");
                    part_number_flag = true;
                } else if y > 0 && !not_part_array.contains(&matrix[y -1][x]) {
                    //println!("part number flag true case 2");
                    part_number_flag = true;
                } else if y > 0 && x < matrix[0].len() - 1 && !not_part_array.contains(&matrix[y - 1][x + 1]) {
                    //println!("part number flag true case 3");
                    part_number_flag = true;
                } else if x > 0 && !not_part_array.contains(&matrix[y][x - 1]) {
                    //println!("part number flag true case 4");
                    part_number_flag = true;
                } else if x < matrix[0].len() - 1 && !not_part_array.contains(&matrix[y][x + 1]) {
                    //println!("part number flag true case 5");
                    part_number_flag = true;
                } else if y < matrix[0].len() - 1 && x > 0 && !not_part_array.contains(&matrix[y + 1][x - 1]) {
                    //println!("part number flag true case 6");
                    part_number_flag = true;
                } else if y < matrix[0].len() - 1 && !not_part_array.contains(&matrix[y + 1][x]) {
                    //println!("part number flag true case 7");
                    part_number_flag = true;
                } else if y < matrix[0].len() - 1 && x < matrix[0].len() - 1 && !not_part_array.contains(&matrix[y + 1][x + 1]) {
                    //println!("part number flag true case 8");
                    part_number_flag = true;
                }
            } else if new_number.len() > 0 && part_number_flag {
                let number_string: String = new_number.clone().into_iter().collect();
                let add:i32 = number_string.parse::<i32>().unwrap();
                println!("sum to add: {}", add);
                sum = sum + add;
                new_number.clear();
                part_number_flag = false;
            } else {
                //println!("Clear new number since not a part number");
                new_number.clear();
                part_number_flag = false;
            }
            //println!("({},{}): {}",y,x, matrix[y][x]);
        }
    }
    println!("Total sum: {}", sum);
}