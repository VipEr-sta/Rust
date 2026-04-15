// CLI tool for comparing two files and finding matches
// This is particullarly useful for comparing a list of passwords to a list of leaked passwords to find matches
// File I/O
// line by line reading
use std::fs::File;




use std::io::BufReader;
use std::io::prelude::*;
// File writing buffer
use std::io::{BufWriter, Write};
use std::fs::OpenOptions;




/// Main function that reads and displays the first two lines from a file
/// Returns a Result for error handling with the ? operator
fn main() -> std::io::Result<()> {


    


    

    commands();

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
    let mut targets: Vec<String> = Vec::new();

    
     
    
    
    read_file(&mut targets, file);
   
    check_matches(targets, target_file_input.to_string());
    

    
    
    println!("Searching for password in file: {}...", target_file_input);
   
    
   

    


    
   
    
    
    
    // Return Ok to indicate successful execution
    Ok(())
    
    
    
}
fn check_matches(targets: Vec<String>, target_file_input: String) {
    let file = File::open(target_file_input).unwrap();
    let mut reader = BufReader::new(file);
    let mut count = 0;
    let mut line = String::new();
    let mut match_count = 0;
    let mut matches: Vec<String> = Vec::new();
     while reader.read_line(&mut line).unwrap() > 0 {

        
        let current = line.trim();
        if targets.contains(&current.to_string()) {
            match_count += 1;
            println!("Matches found: {}", match_count);          
            matches.push(current.trim().to_string()); 
            println!("Password found in line {}:",  current);
            println!("Matches: {:?}", matches);
            write_matches(matches.clone()).unwrap();
            
            
        }
        
        

        else {
            println!("Password not found in line {}", count + 1);
        }
        line.clear();
        count += 1;
        
    }

    
    
}
fn read_file(targets: &mut Vec<String>, file: File) {
    


    let mut reader = BufReader::new(file);

    let mut line = String::new();
    
    
   
   
    
    
    while reader.read_line(&mut line).unwrap() > 0 {
        let word = line.trim();
        if !word.is_empty() {
            targets.push(word.to_string()); // Skip empty lines
            println!("TARGETS: {:?}", targets);
        }
        
        
        line.clear();
        
    }
    
}

fn create_file() -> std::io::Result<()> {
    let mut file_name = String::new();
    println!("Enter the name of the file to create: ");
    std::io::stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim(); // Remove newline character from input
    let file = File::create(file_name)?;
    println!("File created: {}", file_name);
    Ok(())
}

fn delete_file() -> std::io::Result<()> {
    let mut file_name = String::new();
    println!("Enter the name of the file to delete: ");
    std::io::stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim(); // Remove newline character from input
    std::fs::remove_file(file_name)?;
    println!("File deleted: {}", file_name);
    Ok(())
}

fn write_matches(matches: Vec<String>) -> std::io::Result<()>{
    let file_matches = File::create("matches.txt")?;
    let file = OpenOptions::new()
        .append(true)
        .open("matches.txt")?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "\nMatches: {:?}", matches)?; // Write a new line to the file
    writer.flush()?; // Ensure all data is written to the file


    Ok(())
}

fn commands() {
        loop {
            let mut input = String::new();
            println!("List of commmands: \n1. compare: Compare two files\n2. touch: Create a new file\n3. rm: Delete a file\n4. Exit");
            println!("Enter a command: ");
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim(); // Remove newline character from input
        
        match input {
            "touch" => {create_file().unwrap();
            
            
            },
            "rm" => {delete_file().unwrap();
            
            
        }
            "compare" => {println!("");
            break;
        }
            "exit" => std::process::exit(0),
            _ => println!("Invalid command!"),
        }
    }
}
// put into functions
// write to new file with matches