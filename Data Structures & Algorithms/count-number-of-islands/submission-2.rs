impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut islands = 0;
        
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '1' {
                    islands += 1;
                    Self::dfs(&mut grid, row as i32, col as i32);
                }
            }
        }

        islands
    }

    fn dfs(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
        if row < 0 || col < 0 || row as usize >= grid.len() || col as usize >= grid[0].len() {
            return;
        }

        if grid[row as usize][col as usize] != '1' {
            return;
        }

        grid[row as usize][col as usize] = '0';

        Self::dfs(grid, row - 1, col);
        Self::dfs(grid, row + 1, col);
        Self::dfs(grid, row, col - 1);
        Self::dfs(grid, row, col + 1);
        
    }
}
