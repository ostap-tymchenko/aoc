use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn open_file() -> String {
    // Create a path to the desired file
    let path = Path::new("src/dummy-data.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains: data\n", display),
    }

    data
}

// GOAL:
// how many trees are visible from outside the grid?
//
//A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

pub fn main() {
    let file = open_file();

    let mut last_line: Vec<char> = vec![];

    for line in file.lines() {
        // if let Some()
        // if line != last_line { let last_line = line; };
        for (iter, tree_horizontal_index) in (line).chars().enumerate() {
            if last_line[iter].to_owned() == tree_horizontal_index {
                 
            }
        }
        
        last_line = vec![line.chars()];
    }
}        
