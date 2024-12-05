use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("shouldve been able to read file");

    let mut rules: Vec<String> = vec![];
    let mut updates: Vec<String> = vec![];
    let mut all_rules_read: bool = false;
    let mut count :i32 = 0;

    for line in content.lines() {
        if line == "" {
            all_rules_read = true
        } else {
            if !all_rules_read {
                rules.push(line.to_string());
            } else {
                updates.push(line.to_string());
            }
        }
    }

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        let temp_rules: Vec<&str> = rule.split("|").collect();
        let key = temp_rules[0].parse::<i32>().unwrap();
        let value = temp_rules[1].parse::<i32>().unwrap();

        let mut updated_vec: Vec<i32> = vec![];

        if rules_map.get(&key).is_some() {
            updated_vec = rules_map.get(&key).unwrap().to_vec()
        }

        updated_vec.push(value);
        rules_map.insert(key, updated_vec);
    }

    for update in updates {
        let order: Vec<i32> = update
            .split(",")
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();

        let mut correctly_ordered = true;

        for (i, page_number) in order.iter().enumerate() {
            let nums_that_need_to_be_after = rules_map.get(&page_number).unwrap();

            for j in (0..i).rev() {
                if nums_that_need_to_be_after.contains(&order[j]) {
                    correctly_ordered = false;
                }
            }
        }

        if correctly_ordered {
            let middle_num = order[order.len()/2];

            count += middle_num;
        }
    }


    println!("{}", count);
}
