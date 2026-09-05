impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut islands = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 {
                    let islands_len = islands.len();
                    Self::dfs(&mut grid, row as i32, col as i32, &mut islands, islands_len);
                }
            }
        }

        *islands.iter().max().unwrap_or(&0)
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, islands: &mut Vec<i32>, i: usize) {
        if row < 0 || col < 0 || row as usize >= grid.len() || col as usize >= grid[0].len() {
            return;
        }

        if grid[row as usize][col as usize] != 1 {
            return;
        }

        grid[row as usize][col as usize] = 0;

        match islands.get_mut(i) {
            Some(val) => {
                *val += 1;
            },
            None => {
                islands.push(1)
            }
        }

        Self::dfs(grid, row + 1, col, islands, i);
        Self::dfs(grid, row - 1, col, islands, i);
        Self::dfs(grid, row, col + 1, islands, i);
        Self::dfs(grid, row, col - 1, islands, i);
    }
}
