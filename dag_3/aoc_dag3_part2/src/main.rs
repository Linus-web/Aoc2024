use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("should be able to read file");

    let re = Regex::new(
        r"(mul\((?:1000|[1-9][0-9]{0,2}),(?:1000|[1-9][0-9]{0,2})\))|(do\(\))|(don't\(\))",
    )
    .unwrap();

    let collected_data: Vec<String> = re
        .find_iter(&content)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let mut sum: i32 = 0;
    let mut counting: bool = true;

    for item in collected_data {

        if item == "do()" {
            counting = true
        } else if item == "don't()" {
            counting = false
        }

        if counting && item != "do()" && item != "don't()"{
            let nums: Vec<i32> = item[4..item.len()-1].split(",").map(|f| f.parse::<i32>().unwrap()).collect();
            sum += nums[0] * nums[1];
        }
    }

    println!("{sum}");
}
