use std::collections::HashMap;
use std::fs;

pub fn get_priority_map () -> HashMap<char, usize> {
    let mut lower: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .map(|c| (c, c as usize - 96))
        .collect();

    let upper: HashMap<char, usize> = ('A'..='Z')
        .into_iter()
        .map(|c| (c, c as usize - 38))
        .collect();

    lower.extend(upper);

    return lower;
}

pub fn run () {
    let data: String = fs::read_to_string("./d3/day_three.txt")
        .expect("Read data");

    let mut string_vec: Vec<String> = Vec::new();

    for strs in data.split_whitespace().into_iter() {
        string_vec.push(String::from(strs));
    };

    let map_values:HashMap<char, usize> = get_priority_map();

    let part_one = string_vec.iter()
        .map(|r| {
            let (comp1, comp2) = r.split_at(r.len() / 2);
            for c in comp1.chars() {
                if comp2.contains(c) {
                    return *map_values.get(&c).unwrap();
                }
            }
            panic!("Couldn't find a pair.");
        })
        .sum::<usize>();

    let part_two = string_vec.chunks(3)
        .map(|chunk| {
            for c in chunk[0].chars() {
                if chunk[1].contains(c) && chunk[2].contains(c) {
                    return *map_values.get(&c).unwrap();
                }
            }
            panic!("Couldn't find common item.");
        })
        .sum::<usize>();

    println!("part one --> {:?}", part_one);
    println!("part two --> {:?}", part_two);

}