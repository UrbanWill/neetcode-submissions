use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut time = 0;
        let mut fresh = 0;
        let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for row in 0..rows {
            for col in 0..cols {
                let val = grid[row as usize][col as usize];

                if val == 1 {
                    fresh += 1;
                }

                if val == 2 {
                    q.push_back((row, col));
                }
            }
        }

        while !q.is_empty() && fresh > 0 {
            for _ in 0..q.len() {
                let (row, col) = q.pop_front().unwrap();
                for (nr, nc) in dir.iter() {
                    let (row, col) = (row + nr, col + nc);

                    if row < 0 || col < 0 || row >= rows || col >= cols || grid[row as usize][col as usize] != 1 {
                        continue;
                    }

                    grid[row as usize][col as usize] = 2;
                    
                    q.push_back((row, col));

                    fresh -= 1;
                }
            }
            time += 1;
        }

        if fresh > 0 { return - 1 } else { return time }
    }
}
