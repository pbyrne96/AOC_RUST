use::std::fs;
use std::collections::HashSet;

pub fn check_unique (given_string: &String) -> bool {
    let mut seen:Vec<char> = Vec::new();

    for s in given_string.chars() {
        seen.push(s);
    }

    let mut test: HashSet<char> = HashSet::new();
    seen.retain(|e| test.insert(e.clone()));

    return seen.iter().len() == given_string.len();
}


pub fn part_one (buffer: &String, depth: &usize) -> usize {
    let data = buffer.as_bytes();
    for (i, _) in buffer.chars().enumerate() {
        let mut temp_str = String::new();
        for idx in i..(i + depth) + 1 {
            temp_str.push(data[idx] as char);
        }
        if check_unique(&temp_str) {
            return i + 1 +  depth;
        };
    }
    return 0;
}

fn part_two(slice: &str, depth: usize) -> usize{
    for (i, strings) in slice.as_bytes().windows(depth).enumerate() {
        let as_str = strings.into_iter().map(|x| *x as char).collect::<String>();
        if check_unique(&as_str) {
            return i + depth;
        }
    }
    return 0 as usize;
}

pub fn main () {
    let buffer = fs::read_to_string("./day_six/day_six.txt").expect("read the data");
    let depth: usize = 3;
    let message_detector: usize = 14;
    for line in buffer.lines() {
        println!("part one {:?}", part_one(&String::from(line), &depth));
        println!("part two {:?}", part_two(line, message_detector));
    }

}