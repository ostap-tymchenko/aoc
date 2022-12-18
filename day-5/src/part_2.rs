use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::*;

//         [H]     [W] [B]            
//     [D] [B]     [L] [G] [N]        
// [P] [J] [T]     [M] [R] [D]        
// [V] [F] [V]     [F] [Z] [B]     [C]
// [Z] [V] [S]     [G] [H] [C] [Q] [R]
// [W] [W] [L] [J] [B] [V] [P] [B] [Z]
// [D] [S] [M] [S] [Z] [W] [J] [T] [G]
// [T] [L] [Z] [R] [C] [Q] [V] [P] [H]
//
// 1   2   3   4   5   6   7   8   9 

fn open_file(path: &Path) -> String {
    // Create a path to the desired file
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
        Ok(_) => print!(""),
    }

    data
}

fn dbg_print(severity: &String, messege: &String, from: &String) {
    if severity == "warn" {
        println!("{}", format!("{messege} called by {from}").red());
    } else if severity == "debug" {
        println!("{}", format!("{messege} called by {from}").yellow());
    } else {
        println!("{}", format!("malformed debug: severity: {severity}, messege: {messege}, from: {from}").purple());
    }
}

pub fn main() {
    let data_path = Path::new("src/dummy-data.txt");
    let data = open_file(data_path);

    let chart_path = Path::new("src/chart.txt");
    let chart = open_file(chart_path);

    // println!("{chart}");
    for line in chart.lines() {
        if line == "" {
            // nothing
        } else {
            println!("{line}")
        }
    }
    println!();

    let mut chart_vec = vec![];

    for line in chart.lines() {
        chart_vec.push(line.to_string())
    }

    let chart_vec_old = &chart_vec.to_owned();

    for line in data.lines() {
        if line != "" {
            

            let i: Vec<&str> = line.split("-").collect();

            let mut move_times:usize = i[0].parse().unwrap();
            let crate_from:usize = i[1].parse().unwrap();
            let crate_to:usize = i[2].parse().unwrap();
            
            let crate_from = crate_from -1;
            let crate_to = crate_to -1;

            // these 4 lines make sure that there are enough crates to move, and prevents a crash later on if there arent.
            let old_move_times = move_times; 
            if move_times > chart_vec[crate_from].len() {
                move_times = chart_vec[crate_from].len() 
            } 

            if move_times != old_move_times {
                println!("line {line:?} asks to move {old_move_times:?} crates to be moved but {move_times:?} crates will be moved from {crate_from:?} to {crate_to:?}");
            } else {
                println!("line {line:?} asks to move {move_times:?} crates from {crate_from:?} to {crate_to:?}");
            }





            // CORELOGIC


            // we save the var were about to remove to a buffer because this is a move not delete operation
            let move_line = &mut chart_vec[crate_from];
            let mut move_buffer = "".to_string();
            
            for _ in 0..move_times {
                move_buffer += move_line.pop().unwrap_or('/').to_string().as_str();
            } 

            // println!("move buffer = {move_buffer}");
            dbg_print("{},{},{}", format!("debug"), format!("move buffer = {move_buffer}"), format!("CORELOGIC"));

            let move_buffer_copy = &move_buffer.to_owned();

            let response_push = ("pushed +{0} onto {1}", &move_buffer.clone(), &crate_to);

            if let Some(x) = chart_vec.get_mut(crate_to) {x.push_str(&move_buffer); println!("{response_push:#?}")}
            println!("moved crates {} from {crate_from:?} to {crate_to:?}", move_buffer_copy.len());

            let chart_vec_crate_from = &chart_vec[crate_from].to_uppercase();
            let chart_vec_crate_to = &chart_vec[crate_to].to_uppercase();

            if chart_vec_crate_from == "" {
                println!("line (removed from) now contains nothing")
            } else {
                println!("line (removed from) is now {chart_vec_crate_from}");
            }
            println!("line (added to) is now {chart_vec_crate_to}");


            // CORELOGIC





        } 
    }

    let mut top = "".to_string();

    for stack in &chart_vec {
        if stack != "" {
            if stack == " " {
                top.push('|');
            } else {
                top.push(stack.chars().last().unwrap_or('/'));
            }
        }
    }

    let top = top.replace("/", "");
    let top = top.replace("|", " ");
    let top = top.to_uppercase();

    print!("\nthe old crate chart looks like:\n");
    println!("   1  2  3  4  5  6  7  8  9");

    let mut index = 0;
    for line in chart_vec_old {
        index += 1;
        let line = line.to_uppercase();
        print!("{index} ");
        for char in line.chars() {
            print!("[{char}]");
        }
        println!();
    }

    let mut num_crates_one = 0;
    for stack in chart_vec_old {
        for var_crate in stack.chars() {
            num_crates_one += 1;
        }
    }

    print!("\nthe final crate chart looks like:\n");
    println!("   1  2  3  4  5  6  7  8  9");

    let mut index = 0;
    for line in &chart_vec {
        index += 1;
        let line = line.to_uppercase();
        print!("{index} ");
        for char in line.chars() {
            print!("[{char}]");
        }
        println!();
    }

    let mut num_crates_two = 0;
    for stack in &chart_vec {
        for var_crate_two in stack.chars() {
            num_crates_two += 1;
        }
    }
    if num_crates_one == num_crates_two {
        println!("\nthe number of crates is the same ✅ ({num_crates_one})")
    } else {
        let error = "NOT the same ❎";
        println!("\nthe number of crates is {} ({}, {})",error.red() , {num_crates_one}, {num_crates_two})
    }

    println!("the top crates are: {top}");
}
