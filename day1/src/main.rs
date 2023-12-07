use std::fs::File;
use std::io::{self, BufRead};

fn replace_digit_string_with_int(mut line: String) -> String{
    let string_vector: Vec<String> = vec![
        String::from("zero"),
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];

    let number_vector: Vec<String> = vec![
        String::from("zero0zero"),
        String::from("one1one"),
        String::from("two2two"),
        String::from("three3three"),
        String::from("four4four"),
        String::from("five5five"),
        String::from("six6six"),
        String::from("seven7seven"),
        String::from("eight8eight"),
        String::from("nine9nine"),
    ];

    for i in 0..string_vector.len() {
        if line.contains(&string_vector[i]) {
            line = line.replace(&string_vector[i], &number_vector[i]);
        }
    }
    return line;
}

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
            Ok(mut line_content) => {
                
                let mut my_vector: Vec<char> = Vec::new();
                println!("Line: {}", line_content);
                // Do something with the line content
                line_content = replace_digit_string_with_int(line_content);
                println!("modified line: {}", line_content);
                let chars: Vec<char> = line_content.chars().collect();
                if let Some(first_digit) = chars.iter().find(|&c| c.is_digit(10)) {
                    my_vector.push(*first_digit);
                    let digit: u32 = first_digit.to_digit(10).unwrap();
                    //println!("The first digit is: {}", digit);
                }else{
                    println!("No digit found in the string.");
                }
                if let Some(last_digit) = chars.iter().rev().find(|&c| c.is_digit(10)) {
                    my_vector.push(*last_digit);
                    let digit: u32 = last_digit.to_digit(10).unwrap();
                    //println!("The last digit is: {}", digit);
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

    //Readln
    //get first digit from line
    //get last digit from line
    //get sum of first and last digit
    //save result for line
    //do for all lines
    //get sum for all lines

}
