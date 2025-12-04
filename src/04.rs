pub fn solve4(content: &str) -> crate::Solutions {
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let (result1, new_grid) = compute_accessible(&grid);
    let mut removed = 0;
    let mut result2 = result1;
    let mut ng2 = new_grid.clone();
    while result2 != 0{
        removed += result2;
        let (res, new_grid) = compute_accessible(&remove_x_grid(&ng2));
        result2 = res;
        ng2 = new_grid;
    }


    crate::Solutions::Four {
        accessible: result1,
        results: new_grid,
        removed: removed,
        final_grid: ng2,
    }
}

fn compute_accessible(grid: &Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {
    let rows: usize = grid.len();
    let cols = grid[0].len();
    let mut new_grid = grid.clone();
    let mut result = 0;
    for i in 0..rows {
        for j in 0..cols {
            if new_grid[i][j] == '.' {
                continue;
            }
            let valid = valid_i_j(i as u16, j as u16, rows, cols);
            let adjacent = valid
                .iter()
                .filter(|(vi, vj)| new_grid[*vi][*vj] != '.')
                .count();
            if adjacent < 4 {
                new_grid[i][j] = 'x';
                result += 1;
            }
        }
    }
    (result, new_grid)
}
fn remove_x_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows: usize = grid.len();
    let cols = grid[0].len();
    let mut new_grid = grid.clone();
    for i in 0..rows {
        for j in 0..cols {
            if new_grid[i][j] == 'x' {
                new_grid[i][j] = '.';
            }
        }
    }
    new_grid
}
// Returns the valid adjacent positions for (i, j) in a grid of size rows x cols
fn valid_i_j(i: u16, j: u16, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    if i > 0 && i < rows as u16 - 1 && j > 0 && j < cols as u16 - 1 {
        return Vec::from([
            (i as usize - 1, j as usize - 1),
            (i as usize - 1, j as usize),
            (i as usize - 1, j as usize + 1),
            (i as usize, j as usize - 1),
            (i as usize, j as usize + 1),
            (i as usize + 1, j as usize - 1),
            (i as usize + 1, j as usize),
            (i as usize + 1, j as usize + 1),
        ])
    }
     else if i == 0 && j == 0 {
        return Vec::from([
            (i as usize, j as usize + 1),
            (i as usize + 1, j as usize),
            (i as usize + 1, j as usize + 1),
        ])
    } else if i == 0 && j == (cols as u16 - 1) {
        return Vec::from([
            (i as usize, j as usize - 1),
            (i as usize + 1, j as usize - 1),
            (i as usize + 1, j as usize),
        ])
    } else if i == (rows as u16 - 1) && j == 0 {
        return Vec::from([
            (i as usize - 1, j as usize),
            (i as usize - 1, j as usize + 1),
            (i as usize, j as usize + 1),
        ])
    } else if i == (rows as u16 - 1) && j == (cols as u16 - 1) {
        return Vec::from([
            (i as usize - 1, j as usize - 1),
            (i as usize - 1, j as usize),
            (i as usize, j as usize - 1),
        ])
    } else if i == 0 {
        return Vec::from([
            (i as usize, j as usize - 1),
            (i as usize, j as usize + 1),
            (i as usize + 1, j as usize - 1),
            (i as usize + 1, j as usize),
            (i as usize + 1, j as usize + 1),
        ])
    } else if i == (rows as u16 - 1) {
        return Vec::from([
            (i as usize - 1, j as usize - 1),
            (i as usize - 1, j as usize),
            (i as usize - 1, j as usize + 1),
            (i as usize, j as usize - 1),
            (i as usize, j as usize + 1),
        ])
    } else if j == 0 {
        return Vec::from([
            (i as usize - 1, j as usize),
            (i as usize - 1, j as usize + 1),
            (i as usize, j as usize + 1),
            (i as usize + 1, j as usize),
            (i as usize + 1, j as usize + 1),
        ])
    } else if j == (cols as u16 - 1) {
        return Vec::from([
            (i as usize - 1, j as usize - 1),
            (i as usize - 1, j as usize),
            (i as usize, j as usize - 1),
            (i as usize + 1, j as usize - 1),
            (i as usize + 1, j as usize),
        ])
    }
    Vec::new()
}
