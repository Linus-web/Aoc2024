use std::fs;
use regex::Regex;

fn main() {
    
    let content = fs::read_to_string("src/input.txt").expect("should be able to read file");

    let re = Regex::new(r"mul\((?:1000|[1-9][0-9]{0,2}),(?:1000|[1-9][0-9]{0,2})\)").unwrap();

    let multipliers: Vec<String> = re.find_iter(&content)
        .map(|mat| {
            let matched = mat.as_str();
            return matched[4..matched.len() -1].to_string();
        })
        .collect();


    let mut sum : i32 = 0;
    for mul in multipliers  {
       let nums:Vec<i32> = mul.split(",").map(|f|f.parse::<i32>().unwrap()).collect();
        sum += nums[0] * nums[1];

    }


    println!("{sum}");



}
