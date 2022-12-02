pub fn run(all_results: Vec<i32>) -> i32 {
    // this is shit way to find the max --> use a max heap
    all_results.to_owned().sort();
    let _length = all_results.iter().len();
    let mut top_three_sum: i32 = 0;
    for n in all_results
                    .as_slice()
                    [_length-3.._length]
                    .into_iter()
                    {
        top_three_sum += n;
    }
    return top_three_sum;
}