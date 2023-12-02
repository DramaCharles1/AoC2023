//use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    // let file_contents = fs::read_to_string("example.txt")
    //     .expect("LogRocket: Should have been able to read the file");
    // println!("info.txt context =\n{file_contents}");
    
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut sum: i32 = 0;
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(line_content) => {
                
                let mut my_vector: Vec<char> = Vec::new();
                println!("Line: {}", line_content);
                // Do something with the line content
                let chars: Vec<char> = line_content.chars().collect();
                if let Some(first_digit) = chars.iter().find(|&c| c.is_digit(10)) {
                    my_vector.push(*first_digit);
                    let digit: u32 = first_digit.to_digit(10).unwrap();
                    println!("The first digit is: {}", digit);
                    //my_vector.push(digit);
                }else{
                    println!("No digit found in the string.");
                }
                if let Some(last_digit) = chars.iter().rev().find(|&c| c.is_digit(10)) {
                    my_vector.push(*last_digit);
                    let digit: u32 = last_digit.to_digit(10).unwrap();
                    println!("The last digit is: {}", digit);
                    //my_vector.push(digit);
                }else{
                    println!("No digit found in the string.");
                }
                let string_result: String = my_vector.into_iter().collect();
                println!("string_result: {}", string_result);
                sum = sum + string_result.parse::<i32>().unwrap();
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                // Handle the error as needed
            }
        }
    }
    println!("sum: {}", sum);
    Ok(())

    //let chars: Vec<char> = input_string.chars().collect();

    //Readln
    //get first digit from line
    //get last digit from line
    //get sum of first and last digit
    //save result for line
    //do for all lines
    //get sum for all lines

}
