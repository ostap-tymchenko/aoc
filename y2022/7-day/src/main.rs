use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashMap;
use std::{fs, fmt};
use std::path::Path;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

const FOLDER_SPLIT_STR: &str = "/";
const FOLDER_SPLIT_CHAR: char = '/';
const WHITESPACE: char = ' ';
const ROOT: &str = "/";

#[derive(Clone, Default, Debug)]
struct StackPath(Vec<String>);

impl StackPath {
    fn push(&mut self, s: String) {
        self.0.push(s);
    }

    fn pop(&mut self) -> String {
        self.0.pop().expect("stack should not be empty")
    }
}

impl fmt::Display for StackPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for s in self.0.iter() {
            write!(f, "[{s}]")?
        }

        Ok(())
    }
}

fn go_to_target(mut path: StackPath, target: String) -> StackPath {
    if target == ROOT {
        return StackPath(vec![ROOT.to_string()]);
    } else if target == ".." {
        StackPath(vec![path.pop()])
    } else {
        path.push(target);
        path
    }
}

fn get_top_lvl_dir(path: &StackPath) -> String {
    let binding = path.0.get(0).expect("get_top_lvl_dir should be ran from a dir other than root");
    binding.to_string()
}

fn part_one(file_name: &str) -> i32 {
    let data_path = "data/".to_owned() + file_name;
    let data = read_data(Path::new(&data_path));

    let mut sim_path = StackPath::default();
    let mut top_lvl_dir_index: HashMap<String, i32> = HashMap::new();

    for line in data.lines() {
        let fc = line.chars().nth(0).expect("line should have at least one char");

        if fc == '$' {
            // command
            let sc = line.chars().nth(2).expect("line command should have at least two chars");
            if sc == 'c' {
                // command starting with c is cd
                print!("goto from '{sim_path}' with target '{}'", line.split_whitespace().nth(2).unwrap());
                let target = line.split_whitespace().nth(2).expect("line with command should have at least 3 words").to_string();
                sim_path = go_to_target(sim_path, target);
                println!(" new path:'{sim_path:?}'");

            } else if sc == 'l' {
                // command starting with l is ls
            } else {
                panic!("line with command should have at least 3 chars");
            }
        } else if fc == 'd' {
            // dir name
        } else {
            // file
            let file_size = line.split_once(WHITESPACE).expect("file line should have space").0.parse::<i32>().expect("file size should be parsable");
            
            if file_size <= 100_000 {
                let dir = get_top_lvl_dir(&sim_path);
                // if top_lvl_dir_index.contains_key(&dir) {
                //     let top_lvl_dir_index.get_mut(&dir)
                // }
                if let Some(v) = top_lvl_dir_index.get_mut(&dir) {
                    *v + file_size;
                } else {
                    top_lvl_dir_index.insert(dir, file_size);
                };
            }
        }
    }

    let mut total_added_size = 0;
    for value in top_lvl_dir_index.values() {
        total_added_size += value;
    }

    total_added_size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), 95437);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two("dummy-data.txt"), todo!);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one("data.txt"));

    Ok(())
}
