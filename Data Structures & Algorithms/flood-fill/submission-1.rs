impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let orig = image[sr as usize][sc as usize];
        if color == orig {
            return image;
        }
        let mut image = image;
        let row = image.len();
        let col = image[0].len();
        Self::dfs(&mut image, row, col, sr as usize, sc as usize, color, orig);
        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, row: usize, col: usize, r: usize, c: usize, color: i32, orig: i32) {
        if image[r][c] != orig {
            return;
        }

        image[r][c] = color;

        if r + 1 < row { Self::dfs(image, row, col, r + 1, c, color, orig) };
        if r > 0 { Self::dfs(image, row, col, r - 1, c, color, orig) };
        if c + 1 < col { Self::dfs(image, row, col, r, c + 1, color, orig) };
        if c > 0 { Self::dfs(image, row, col, r, c - 1, color, orig) };
    }
}
