use std::fs;

pub fn main ()  {
    let mut _totals: Vec<i32> = Vec::new();
    let calories = fs::read_to_string("./day_one_input.txt")
        .expect("Read the data");
    let mut _iter_str = String::new();

    for line in  calories.lines() {
        if !line.is_empty() {
            _iter_str.push(',');
            _iter_str.push_str(line.trim());
        } else {
            let mut as_num: Vec<i32> = Vec::new();
             _iter_str.replacen(",", "", 1);
            _iter_str.clear();
        }
    }



}