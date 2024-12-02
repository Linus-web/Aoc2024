use std::fs;


fn main() {

    let contents = fs::read_to_string("src/list.txt").expect("shouldve been able to read this file");

    let split_content : Vec<&str> = contents.trim().split("\n").collect();
    let split_content_len = split_content.len();
    let mut list1 : Vec<&str> = vec![];
    let mut list2 : Vec<&str> = vec![];


    println!("{:?}", split_content);


    for line in split_content {
        let temp : Vec<&str> = line.split_whitespace().collect();
        list1.push(temp[0]);
        list2.push(temp[1]);
    }

    list1.sort();
    list2.sort();

    let mut diff : i32 = 0;

    for i in 0..split_content_len{

        let list1_num : i32 = list1[i].parse::<i32>().unwrap();
        let list2_num : i32 = list2[i].parse::<i32>().unwrap();

        if list1_num < list2_num {
            diff += list2_num - list1_num;
        }else if list1_num > list2_num {
            diff += list1_num - list2_num;
        }else {
            diff += 0;
        }
        

    }

    println!("{}", diff)
    

}
