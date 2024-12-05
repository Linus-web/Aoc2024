use std::{collections::HashMap, fs, time::Instant};

fn main() {

    let start = Instant::now();
    let content = fs::read_to_string("/home/linus/Desktop/Code Projects/Aoc2024/dag_5/aoc_5_2/src/input.txt").expect("shouldve been able to read file");

    let mut rules: Vec<String> = vec![];
    let mut updates: Vec<String> = vec![];
    let mut all_rules_read: bool = false;

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

    get_rules(rules, &mut rules_map);

    let mut incorrect_updates: Vec<Vec<i32>> = vec![];

    get_incorrect_order(&rules_map, updates, &mut incorrect_updates);

    sort_incorrect_orders(&rules_map, &mut incorrect_updates);

    let count = count_updates(&incorrect_updates);


    println!("{:?}", count);
    println!("it took: {:?} to calculate.", start.elapsed());
}




fn count_updates(incorrect_updates: &Vec<Vec<i32>>) -> i32 {

    let mut count = 0;

    for update in incorrect_updates {
        count += update[update.len()/2];
    }

    return count

}




fn sort_incorrect_orders(rules: &HashMap<i32, Vec<i32>>, incorrect: &mut Vec<Vec<i32>>) {
    for order in incorrect.iter_mut() {
        let mut sorted = false;

        while !sorted {
            sorted = true; 

            for i in 0..order.len() {
                if let Some(nums_that_need_to_be_after) = rules.get(&order[i]) {
                    for j in (0..i).rev() {
                        if nums_that_need_to_be_after.contains(&order[j]) {
                            order.swap(i, j);
                            sorted = false; 
                        }
                    }
                }
            }
        }
    }
}

fn get_incorrect_order(
    rules: &HashMap<i32, Vec<i32>>,
    updates: Vec<String>,
    incorrect: &mut Vec<Vec<i32>>,
) {
    for update in updates {
        let order: Vec<i32> = update
            .split(",")
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();

        let mut correctly_ordered = true;

        for (i, page_number) in order.iter().enumerate() {
            let nums_that_need_to_be_after = rules.get(&page_number).unwrap();

            for j in (0..i).rev() {
                if nums_that_need_to_be_after.contains(&order[j]) {
                    correctly_ordered = false;
                }
            }
        }

        if !correctly_ordered {
            incorrect.push(order);
        }
    }
}

fn get_rules(rules: Vec<String>, map: &mut HashMap<i32, Vec<i32>>) {
    for rule in rules {
        let temp_rules: Vec<&str> = rule.split("|").collect();
        let key = temp_rules[0].parse::<i32>().unwrap();
        let value = temp_rules[1].parse::<i32>().unwrap();

        let mut updated_vec: Vec<i32> = vec![];

        if map.get(&key).is_some() {
            updated_vec = map.get(&key).unwrap().to_vec()
        }

        updated_vec.push(value);
        map.insert(key, updated_vec);
    }
}
