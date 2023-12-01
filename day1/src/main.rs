use std::fs;
fn main() {
    let file_contents = fs::read_to_string("example.txt")
        .expect("LogRocket: Should have been able to read the file");
    println!("info.txt context =\n{file_contents}");
}
