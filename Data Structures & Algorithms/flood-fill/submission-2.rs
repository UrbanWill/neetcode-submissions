impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let orig = image[sr as usize][sc as usize];

        if orig == color {
            return image;
        }

        let mut image = image;
        let r_len = image.len();
        let c_len = image[0].len();
        Self::dfs(&mut image, r_len, c_len, sr as usize, sc as usize, orig, color);

        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, r_len: usize, c_len: usize, r: usize, c: usize, orig: i32, color: i32) {
        if image[r][c] != orig {
            return;
        }

        image[r][c] = color;

        if r + 1 < r_len { Self::dfs(image, r_len, c_len, r + 1, c, orig, color)};
        if r > 0 { Self::dfs(image, r_len, c_len, r - 1, c, orig, color)};
        if c + 1 < c_len { Self::dfs(image, r_len, c_len, r, c + 1, orig, color)};
        if c > 0 { Self::dfs(image, r_len, c_len, r, c - 1, orig, color)}
    }
}
