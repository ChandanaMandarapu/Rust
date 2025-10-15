// real world stuff 
// files are smtng in simple way which remembers things even after u complete a project a simple exeample how files talk with outside world

// understanding std::Fs  when we write std::Fs we are accesing rust standard libratys file system module 

// simple chandu std - means standard library
// fs - file system (which deals with files and folders)

// --- first - reading a file ---

// what happens here when u want to read a file  rust asks os want to read tihs file os goes and checks does this file exist or this prgm execute  if yes os opens file and returns the contents 

// thats whwre the concept of errors comes here
// When you write let contents = fs::read_to_string("myfile.txt");, the variable contents doesn't contain the file's text. It contains a Result that might have the text, or might have an error.

// handlin errors using match method 1

use std::fs;
use std::io;

fn main() {
    let contents = fs::read_to_string("myfile.txt");
    match contents {
        Ok(text) => {
            println!("file contents");
            println!("{}", text);
        }
        Err(error) => {
            println!("error reading file : {}", error);
        }
    }
    // handling errors using match - explicit way 
    println!("{:?}", contents);
}

// handling using unwrap a shortcut kinda
// unwrap() -> jusst its certain this will work if it doesnt just crash whole program use unwrap() - only when wrting  aquick test 
fn unwrap_example() {
    let contents = fs::read_to_string("myfile.txt").unwrap();
    println!("{}", contents);
}

// method 3 - using expect 
// expect is jusut like unwrap() but here we can provide a custom error message if the file reading fails ur program will crash but display ur message 
fn expect_example() {
    let contents = fs::read_to_string("myfile.txt")
        .expect("failed to read myfile.txt");
    println!("{}", contents);
}

// method 4 - ? operator most profreessional wayuu
// the ? operator if this is ok unwrap the value and continue if this is err immediately return the error from this function 
fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("myfile.txt")?;
    Ok(contents)
}

fn main_read_file() {
    match read_file() {
        Ok(text) => println!("{}", text),
        Err(e) => println!("Error {}", e),
    }
}

// now lets practice writing to files - creating and modifyin until now understood how to read data
// OK(()) -> here is unit in rust it means operationis succeeded but theres no meaniful value ro return 

// also another learnin fs::write will completly erase the exising file and replace with new content 
fn main_write() {
    let content = "Hello im writing a file";
    let result = fs::write("output.txt", content);

    match result {
        Ok(()) => println!("Sucessfully wrote to file"),
        Err(e) => println!("failed to write {}", e),
    }
}

// appending to files adding without erasing  adding something to existing file without erasing 
// OpenOptions - is like a form u fill out to specify exactly how u want to open afile 

use std::fs::OpenOptions;
use std::io::Write;

fn main_append() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .expect("failed to open file");
    file.write_all(b"New log entry\n")
        .expect("Failed to write to file");
}

// understanding path  and pathbuf 

// path is like reference to a path that exists somewhere else it doesnt own the data but
// pathbuf is like owning the path data u can cmodigy it add to it return it from funcns 
use std::path::Path;
use std::path::PathBuf;

fn main_path() {
    let path = Path::new("folder/subfolder/file.txt");

    println!("File name: {:?}", path.file_name());
    println!("Parent folder: {:?}", path.parent());
    println!("Extension: {:?}", path.extension());
}

fn main_pathbuf() {
    let mut path = PathBuf::from("my_folder");
    path.push("subfolder");
    path.push("file.txt");
    
    println!("Full path: {:?}", path);
    // Prints: "my_folder/subfolder/file.txt"
}

// working with directories 

// creating a directory 

use std::fs;

fn main_create_dir() {
    fs::create_dir("my_new_folder")
        .expect("failed to create directory");
}

// fs::create_dir_all("project/src/modules")
//     .expect("Failed to create directories");

// reading directory contents

fn main_read_dir() {
    let entries = fs::read_dir(".")
        .expect("Failed to read directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to get entry");
        println!("Found: {:?}", entry.path());
    }
}

// checking if smtng exists

fn main_check_exists() {
    if Path::new("myfile.txt").exists() {
        println!("file exists");
    } else {
        println!("file not found");
    }

    let path = Path::new("myfolder");
    if path.is_dir() {
        println!("its a directory");
    } else if path.is_file() {
        println!("its a file");
    }
}

// deleting files and directories

fn main_delete() {
    // delete a file
    fs::remove_file("unwanted.txt")
        .expect("failed to delete file");

    // delete an empty directory
    fs::remove_dir("empty_folder")
        .expect("failed to delete directory");

    // delete a directory and all its contents
    fs::remove_dir_all("folder wit stuff")
        .expect("Failed to delete directory");
}

// error handlin for I/O - profressional approach

// a real world example

use std::fs;
use std::io;
use std::path::Path;

fn safe_read_file(path: &str) -> Result<String, io::Error> {
    // Check if file exists first
    if !Path::new(path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File '{}' does not exist", path)
        ));
    }
    
    // Try to read the file
    let contents = fs::read_to_string(path)?;
    
    Ok(contents)
}

fn main() {
    match safe_read_file("config.txt") {
        Ok(contents) => {
            println!("Successfully read file:");
            println!("{}", contents);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Error kind: {:?}", e.kind());
        }
    }
}