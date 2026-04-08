
// File I/O
// line by line reading
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*; // for read_line method
// File writing buffer
use std::io::{BufWriter, Write};
use std::fs::OpenOptions; // for appending to file

/// Main function that reads and displays the first two lines from a file
/// Returns a Result for error handling with the ? operator
fn main() -> std::io::Result<()> {
    // Open the tvshowlist.txt file - returns error if file doesn't exist
    let file = File::open("src/bin/tvshowlist.txt")?;
    
    
    // Create a buffered reader for efficient line-by-line reading
    let mut reader = BufReader::new(file);
    for i in 0..8 {
        println!("Line {}: ", i);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line = line.trim().to_string();
        println!("{}", line);
    }

    let file = OpenOptions::new()
        .append(true)
        .open("src/bin/tvshowlist.txt")?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "\nThe Office")?; // Write a new line to the file
    writer.flush()?; // Ensure all data is written to the file


    
   
    
    
    
    // Return Ok to indicate successful execution
    Ok(())
    
    
    
}
