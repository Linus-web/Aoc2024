use std::{collections::HashMap, fs};


fn main() {

    let contents = fs::read_to_string("src/list.txt").expect("Shouldve been able to read from file.");

    let mut line1 : Vec<&str> = vec![];
    let mut line2 : Vec<&str> = vec![];

    let mut num_similary_map : HashMap<&str, i32> = HashMap::new();

    for line in contents.lines() {
        let temp_split : Vec<&str> = line.split_whitespace().collect();

        line1.push(temp_split[0]);
        line2.push(temp_split[1]);

        *num_similary_map.entry(temp_split[1]).or_insert(0) += 1;

    }

    let mut similary_score :i32 = 0;

    for num in line1 {
        if num_similary_map.contains_key(num) {

            similary_score += num.parse::<i32>().unwrap() * num_similary_map.get(num).unwrap();
        }
    }

    println!("{}", similary_score)


}
