impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = Vec::new();
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for row in 0..rows {
            for col in 0..cols {
                if grid[row as usize][col as usize] == 1 {
                    let i = res.len();

                    Self::dfs(&mut grid, &mut res, &dir, row, col, i);
                }
            }
        }


        *res.iter().max().unwrap_or(&0)
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, res: &mut Vec<i32>, dir: &Vec<(i32, i32)>, row: i32, col: i32, i: usize) {
        if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32 || grid[row as usize][col as usize] != 1 {
            return
        }

        grid[row as usize][col as usize] = 0;

        match res.get_mut(i) {
            Some(val) => {
                *val += 1;
            },
            None => {
                res.push(1);
            }
        }

        for (nr, nc) in dir.iter() {
            let (row, col) = (row + nr, col + nc);
            Self::dfs(grid, res, dir, row, col, i);
        }

    }
}
