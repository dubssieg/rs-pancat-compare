use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn process_line_element_by_element(file_path: &str, line_number: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut current_line = 0;
    for line in reader.lines() {
        current_line += 1;
        if current_line == line_number {
            let line = line?;
            let mut element = String::new();
            for ch in line.chars() {
                if ch == ',' {
                    println!("Element: {}", element);
                    element.clear();
                } else {
                    element.push(ch);
                }
            }
            // Print the last element if there is no trailing comma
            if !element.is_empty() {
                println!("Element: {}", element);
            }
            break;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = "path/to/your/file.csv";
    let line_number = 3; // Change this to the line number you want to process
    process_line_element_by_element(file_path, line_number)
}
