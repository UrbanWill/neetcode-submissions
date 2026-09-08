impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut res = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                let i = res.len();
                Self::dfs(&mut grid, row as i32, col as i32, i, &mut res);
            }
        }

        *res.iter().max().unwrap_or(&0)

    }

    fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, i: usize, res: &mut Vec<i32>) {
        if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32 {
            return;
        }


        if grid[row as usize][col as usize] != 1 {
            return
        }

        grid[row as usize][col as usize] = 0;



        match res.get_mut(i) {
            Some(val) => {
                *val += 1;
            },
            None => { res.push(1) }
        }

        Self::dfs(grid, row + 1, col, i, res);
        Self::dfs(grid, row - 1, col, i, res);
        Self::dfs(grid, row, col + 1, i, res);
        Self::dfs(grid, row, col -1, i, res);
    }
}
