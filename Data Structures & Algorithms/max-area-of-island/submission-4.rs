impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut area = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row as usize][col as usize] == 1 {

                    area = area.max(Self::dfs(&mut grid, &dir, row, col));
                }
            }
        }


        area
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, dir: &Vec<(i32, i32)>, row: i32, col: i32) -> i32 {
        if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32 || grid[row as usize][col as usize] != 1 {
            return 0;
        }

        grid[row as usize][col as usize] = 0;

        let mut island_area = 1;

        for (nr, nc) in dir.iter() {
            let (row, col) = (row + nr, col + nc);
            island_area += Self::dfs(grid, dir, row, col)
        }

        island_area
    }
}
