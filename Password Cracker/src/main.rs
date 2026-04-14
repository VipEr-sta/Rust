
// File I/O
// line by line reading
use std::fs::File;

use std::io::BufReader;
use std::io::prelude::*;


/// Main function that reads and displays the first two lines from a file
/// Returns a Result for error handling with the ? operator
fn main() -> std::io::Result<()> {

    let mut file_input = String::new();
    println!("Enter the name of the file to read: ");
    std::io::stdin().read_line(&mut file_input).unwrap();
    let file_input = file_input.trim(); // Remove newline character from input
    
    let file = File::open(file_input)?;

    let mut target_file_input = String::new();
    println!("Enter the name of the file to search for: ");
    std::io::stdin().read_line(&mut target_file_input).unwrap();
    let target_file_input = target_file_input.trim(); // Remove newline character from input
    
    // Input 

    
    let mut found = false;


    let mut reader = BufReader::new(file);

    let mut line = String::new();
    
    
   
   
    let mut targets: Vec<String> = Vec::new();
    
   

    while reader.read_line(&mut line).unwrap() > 0 {
        let word = line.trim();
        if !word.is_empty() {
            targets.push(word.to_string()); // Skip empty lines
            println!("TARGETS: {:?}", targets);
        }
        
        
        line.clear();
        
    }

    let file = File::open(target_file_input)?;
    let mut reader = BufReader::new(file);
    let mut count = 0;
    let mut line = String::new();
    let mut match_count = 0;
    let mut matches: Vec<String> = Vec::new();
    
    println!("Searching for password in file: {}...", target_file_input);
    while reader.read_line(&mut line).unwrap() > 0 {
        
        let current = line.trim();
        if targets.contains(&current.to_string()) {
            found = true;
            match_count += 1;
            println!("Matches found: {}", match_count);          
            matches.push(current.trim().to_string()); 
            println!("Password found in line {}:",  current);
            println!("Matches: {:?}", matches);
        }
        
        

        else {
            println!("Password not found in line {}", count + 1);
        }
        line.clear();
        count += 1;
        
    }

   

    


    
   
    
    
    
    // Return Ok to indicate successful execution
    Ok(())
    
    
    
}
