use std::path::Path;
use std::fs::File;
use std::io::Read;

fn open_file(file_path: &str) -> String {
    let file_path = Path::new(file_path);
    let display = file_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&file_path) {
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

pub fn main() {
    let file = open_file("src/dummy-data.txt");

    let forrest: Vec<String> = file.split('\n').map(|x| x.to_string()).collect();
    display_forrest(&forrest, "input"); 

    // for row in forrest {
    //     for considered_tree in row.chars() {
    //         let considered_tree = considered_tree.try_into().unwrap();
    //         let mut scenic_score = 0;
    //         for (iter, very_scenic_tree) in row.chars().enumerate() {
    //             if very_scenic_tree >= considered_tree {
    //                 if scenic_score != 0 {
    //                     scenic_score = scenic_score * iter
    //                 }
    //             } 
    //         }

    //     }
    // }
    
    for (x, row) in forrest.iter().enumerate() {
        for (y, tree) in row.chars().enumerate() {
            // println!("x:{x}, row:{row}, y:{y}, tree:{tree}")
            // println!("row {x} column {y} is {}", check_height(x, y, &forrest));


            
        }
    }
}

fn calc_senic_score(x: usize, y:usize, forrest: &Vec<String>) {
    let center_height = check_height(x, y, forrest);
    let mut counter: usize = 0;
    loop {
        counter += 1;
        if !check_height(x+counter, y+counter, forrest) >= center_height {
            break;
        }
    }
}

fn check_height(x: usize, y:usize, forrest: &Vec<String>) -> char {
    forrest[x].chars().nth(y).unwrap()
}

fn display_forrest(forrest_reconstruction: &Vec<String>, name: &str) {
    println!("\n | {name}");
    for (row_iter, row) in forrest_reconstruction.iter().enumerate() {
        if row_iter < 10 {
            // The different spacing is on pourpose in 
            // order to get a cleaner formated output
            println!("{row_iter}|  {row}");
        } else {
            println!("{row_iter}| {row}");
        }
    }
} 
