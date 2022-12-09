pub fn run(_max_calories_vec: Vec<i32> ) -> i32 {
    let mut max_count = i32::from(0);
    for _calorie_count in _max_calories_vec.iter() {
        if _calorie_count > &max_count {
            max_count = *_calorie_count;
        }
    }
    return max_count;
}


