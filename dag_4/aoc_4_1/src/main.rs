use std::fs;





fn main() {
    let content = fs::read_to_string("src/input.txt").expect("shouldve been able to read file");

    let grid : Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();


    let directions: [(i32,i32);8] = [
        (-1, -1), // Top-left
        (-1,  0), // Up
        (-1,  1), // Top-right
        (0, -1), // Left
        (0,  1), // Right
        (1, -1), // Bottom-left
        (1,  0), // Down
        (1,  1), // Bottom-right
    ];  
    

    let mut counter = 0;


    for (x,row) in grid.iter().enumerate()  {
        for (y, letter) in row.iter().enumerate() {

            if *letter == 'X' {


                for &(dx, dy) in &directions {
                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;

                    if let Some(neighbor_letter) = get_cell(&grid, new_x, new_y) {
                        if neighbor_letter == 'M' {

                            if check_for_word(&grid, x as i32 ,y as i32,dx,dy,&['X','M','A','S']) {
                                counter += 1;
                            }


                        }
                    }
                }



            }

        }
        
    }

    println!("the word was found {} times.", counter);

}


fn get_cell(grid: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if x >= 0 && y >= 0 {
        grid.get(x as usize).and_then(|row| row.get(y as usize).cloned())
    } else {
        None
    }
}



fn check_for_word(grid: &Vec<Vec<char>>, x:i32,y:i32, dx:i32, dy:i32, checkword:&[char]) -> bool {

    let mut current_x = x;
    let mut current_y = y;

    for &expected_char in checkword {

        if let Some(cell_char) = get_cell(grid, current_x, current_y) {
            if cell_char != expected_char {
                return false;
            }
        } else {
            return false;
        }

        current_x += dx;
        current_y += dy;

    }

    return true

}