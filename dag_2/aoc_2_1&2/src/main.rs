use std::fs;

fn is_line_safe(line: &str) -> bool {
    let mut decreasing: bool = false;
    let mut increasing: bool = false;
    let mut last_num: Option<i32> = None;

    for num in line.split_whitespace() {
        let temp: i32 = num.parse::<i32>().unwrap();

        if let Some(last) = last_num {
            if temp > last && temp <= last + 3 {
                increasing = true;
                last_num = Some(temp);
            } else if temp < last && temp >= last - 3 {
                decreasing = true;
                last_num = Some(temp);
            } else {
                return false;
            }

            if decreasing && increasing {
                return false;
            }
        } else {
            last_num = Some(temp);
        }
    }

    return true;
}

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("shouldve been able to read file.");

    let mut counter: i32 = 0;
    for line in content.lines() {


        if is_line_safe(line){
            counter += 1;
            continue;
        } else {

            let numbers : Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();

            for i in 0..numbers.len() {

                let mut temp_vec :Vec<i32> = vec![];
                
                for (j,&num) in numbers.iter().enumerate() {
                    
                    if j != i {
                        temp_vec.push(num);
                    }
                    
                }

                if is_line_safe(&temp_vec.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")) {
                    counter += 1;
                    break;
                }
                    
            }

        }

        }


        println!("safe: {}", counter);



        }

