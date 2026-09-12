use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
      let mut q = VecDeque::new();
      let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
      let dirs = vec![(0, -1), (-1, 1), (0, 1), (1, 1), (1, 0), (1, - 1), (-1, 0), (-1, -1)];

      if grid[0][0] == 1 || grid[rows as usize -1][cols as usize -1] == 1 {
        return -1;
      }

      if rows == 0 && cols == 0 {
        return 1;
      }

      q.push_back((0, 0, 1));   

      while let Some((row, col, len)) = q.pop_front() {
        if row == rows -1 && col == cols -1 {
            return len;
        }

        for (nr, nc) in dirs.iter() {
            let (row, col) = (row + nr, col + nc);

            if row < 0 || col < 0 || row >= rows || col >= cols || grid[row as usize][col as usize] == 1 {
                continue;
            }

            grid[row as usize][col as usize] = 1;

            q.push_back((row, col, len + 1));

        }
      }
      
      -1
    }
}
