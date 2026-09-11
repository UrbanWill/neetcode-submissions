use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut len = 1;
        let mut q = VecDeque::new();
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let neighbours = vec![(0, 1), (-1, 1), (0, -1), (1, -1), (1, 0), (1, 1), (-1,0), (-1, 1)];

        if grid[0][0] == 1 || grid[(rows - 1) as usize][(cols - 1) as usize] == 1 {
            return -1;
        }

        if rows == 1 && cols == 1 {
            return 1;
        }

        grid[0][0] = 1;

        q.push_back((0,0));

        while !q.is_empty() {
            len += 1;
            for _ in 0..q.len() {
                let (row, col) = q.pop_front().unwrap();
                'neighbours: for (nr, nc) in neighbours.iter() {
                    let (row, col) = (row + nr, col + nc);

                    if row < 0 || col < 0 || row >= rows || col >= cols {
                        continue 'neighbours;
                    }
                    
                    if grid[row as usize][col as usize] == 1 {
                        continue 'neighbours;
                    }


                    if row == rows -1 && col == cols -1 {
                        return len;
                    }

                    grid[row as usize][col as usize] = 1;

                    q.push_back((row, col));
                }
            }
        }

        -1
    }
}
