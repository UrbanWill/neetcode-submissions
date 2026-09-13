impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let dir = vec![(0,1), (0, -1), (1,0), (-1,0)];
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

        for row in 0..rows {
            for col in 0..cols {
                if grid[row as usize][col as usize] == '1' {
                    count += 1;
                    Self::bfs(&mut grid, &dir, row, col);
                }
            }
        }

        count
    }

    fn bfs(grid: &mut Vec<Vec<char>>, dir: &Vec<(i32, i32)>, row: i32, col: i32) {
        if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32 || grid[row as usize][col as usize] != '1' {
            return;
        }

        grid[row as usize][col as usize] = '0';

        for (nr, nc) in dir.iter() {
            let (row, col) = (row + nr, col + nc);
            Self::bfs(grid, dir, row, col);
        }
    }
}
