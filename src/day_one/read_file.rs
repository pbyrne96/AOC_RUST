use std::fs;

pub fn run () -> Vec<i32> {
    let mut _totals: Vec<i32> = Vec::new();
    let calories = fs::read_to_string("./day_one/day_one_input.txt")
        .expect("Read the data");
    let mut _iter_str = String::new();

    for line in  calories.lines() {
        if !line.is_empty() {
            _iter_str.push(',');
            _iter_str.push_str(line.trim());
        } else {

            _iter_str.push(':');
        }
    }

    for strs in _iter_str.split(":") {
        let mut _iter_vec: Vec<i32> = Vec::new();
        let chars_vec:Vec<char> = strs.chars().collect();
        let _sliced_vec = &chars_vec.as_slice()[1..chars_vec.iter().len()].iter().collect::<String>();
        for sliced_string in _sliced_vec.split(",") {
            let as_num: i32 = sliced_string.parse::<i32>().unwrap();
            _iter_vec.push(as_num);
        }
        _totals.push(_iter_vec.iter().sum());
    }

    return _totals;
}
