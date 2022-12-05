use std::collections::HashMap;
use std::{fs};


pub fn run () {
    let _initial_stack: HashMap<i32, Vec<&str>> = HashMap::from([
        (1, ["Z", "J", "G", " ", " ", " ", " ", " "].to_vec()),
        (2, ["Q", "L", "R", "P", "W", "F", "V", "C"].to_vec()),
        (3, ["F", "P", "M", "C", "L", "G", "R", ""].to_vec()),
        (4, ["L", "F", "B", "W", "P", "H", "M", ""].to_vec()),
        (5, ["G", "C", "F", "S", "V", "Q", " ", " "].to_vec()),
        (6, ["W", "H", "J", "Z", "M", "Q", "T", "L"].to_vec()),
        (7, ["F", "J", "Z", "S", " ", " ", " ", " "].to_vec()),
        (8, ["M", "C", "D", "P", "F", "H", "B", "T"].to_vec()),
    ]);


    let data:Vec<String> = fs::read_to_string("./day_five/stripped_input.txt")
        .expect("read the data")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    for line in data.iter() {

        let mut new_line = line.replace("move", ":");
        new_line = new_line.replace("from", ":");
        new_line = new_line.replace("to", ":");

        let just_nums = new_line
            .split(":")
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let (mut amount, from , to) = (just_nums[0], just_nums[1], just_nums[2]);

        let mut from_vec = _initial_stack.get(&from).unwrap();
        let mut to_vec = _initial_stack.get(&from).unwrap();

        let upper_lim = from_vec.clone().iter().len()-1;
        if amount as usize > upper_lim {
            amount = upper_lim as i32;
        }




    }
    println!("{:?}", _initial_stack);
}