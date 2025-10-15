// real world stuff 
// files are smtng in simple way which remembers things even after u complete a project a simple exeample how files talk with outside world

// understanding std::Fs  when we write std::Fs we are accesing rust standard libratys file system module 

// simple chandu std - means standard library
// fs - file system (which deals with files and folders)

// --- first - reading a file ---

use std::fs;

fn main() {
    let contents = fs::read_to_string("myfile.txt");
    println!("{:?}",contents);
}