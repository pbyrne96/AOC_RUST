use std::fs;

fn main() {
    let calories = fs::read_to_string("./dayOne.txt").expect("Read the data");
    let split_input_str = calories.lines();
    let mut _given_vec = split_input_str.collect::<Vec<&str>>();
    let mut _index_of = _given_vec.iter().position(|&r| r.is_empty()).unwrap();

    while Some(_index_of as i32) != None {
        let next = _given_vec.iter().position(|&r| r.is_empty());
        if next != None {
            _given_vec[_index_of] = ":-:";
            _index_of = next.unwrap();
        } else {
            break;
        }
    }

    let max_calories_joined = _given_vec.join(",");
    let _max_calories_vec = max_calories_joined.split(":-:").collect::<Vec<&str>>();

    let mut max_count: i32 = 0;
    for arr in _max_calories_vec.iter() {
        let res = arr.split(",").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let mut res_trimmed: Vec<i32> = Vec::new();
        for s in res.iter() {
            res_trimmed.push(s.trim().parse::<i32>().unwrap());
        }

        let _calorie_count: i32 = res_trimmed.iter().sum();
        if _calorie_count > max_count {
            max_count = _calorie_count;
        }
    }

    println!("running");
    println!("{0}", max_count);

}


