use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("shouldve been able to read file");

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let directions: [(i32, i32); 4] = [
        (-1, -1), // Top-left
        (-1, 1),  // Top-right
        (1, -1),  // Bottom-left
        (1, 1),   // Bottom-right
    ];

    let mut counter = 0;


    for (x, row) in grid.iter().enumerate() {
        for (y, letter) in row.iter().enumerate() {
            if *letter == 'A' {

            let mut line_in_x_counter = 0;

                for &(dx, dy) in &directions {
                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;
                    let reverse_new_x = x as i32 - dx;
                    let reverse_new_y = y as i32 - dy;

                    if let Some(neighbor_letter) = get_cell(&grid, new_x, new_y) {
                        if neighbor_letter == 'M' {
                            if let Some(neighbor_letter) = get_cell(&grid, reverse_new_x, reverse_new_y) {
                                if neighbor_letter == 'S' {
                                    line_in_x_counter += 1
                                }
                            }

                        }
                    }

                }

                if line_in_x_counter == 2 {
                    counter += 1;
                }
            }
        }
    }

    println!("an X-MAS appears {} times.", counter);
}

fn get_cell(grid: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if x >= 0 && y >= 0 {
        grid.get(x as usize)
            .and_then(|row| row.get(y as usize).cloned())
    } else {
        None
    }
}
