use std::fs;




fn main() {

    let content = fs::read_to_string("/home/linus/Desktop/Code Projects/Aoc2024/dag_6/aoc_6_1/src/input.txt").unwrap();


    let mut grid :Vec<Vec<char>> = vec![];

    let directions = [(-1,0),(0,1),(1,0),(0,-1)];

    let mut guard_pos = parse_grid(&content, &mut grid);

    let mut turned_count : i32 = 0;

    loop {


        let direction = directions[(turned_count%4) as usize];

        let dx = guard_pos.0 + direction.1;
        let dy = guard_pos.1 + direction.0;

         if dx > grid[1].len() as i32 || dy > grid.len() as i32 || dx < 0 || dy < 0  {
            break;
        }

        if grid[dy as usize][dx as usize] == '#' {
            turned_count += 1;
            continue;
        }else{
            grid[guard_pos.1 as usize][guard_pos.0 as usize] = 'X';
            guard_pos = (dx,dy);
        }

    }



    println!("{}", check_for_x(&mut grid));





    let mut output : String = String::new();

    for line in grid {
        for char in line {
            output.push(char);
        }
        output += "\n";
    }

    fs::write("output.txt", output).unwrap();


}


fn check_for_x(grid: &mut Vec<Vec<char>>) -> i32 {

    let mut count = 0;

    for line in grid {
        for char in line {
            if *char == 'X' {
                count += 1;
            }
        }
    }


    count



}


fn parse_grid(input : &String, grid : &mut Vec<Vec<char>>) -> (i32,i32) {


    let mut player_pos : (i32,i32) = (0,0);

    for (i,line) in input.lines().enumerate(){

        let mut row : Vec<char> = vec![];

        for (j,character) in line.chars().enumerate() {
            row.push(character);

            if character == '^' {
                player_pos = (j as i32,i as i32);
            }
        }

        grid.push(row);

    }

    return player_pos;

}