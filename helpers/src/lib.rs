use std::str::FromStr;
#[derive(Debug)]
pub struct CharGrid {
    lines: Vec<String>,
    pub height: i32,
    pub width: i32,
}
#[derive(Debug)]
pub struct ParseGridError;

impl FromStr for CharGrid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s.lines().map(|s| s.to_owned()).collect();
        let width = lines.get(0).ok_or(ParseGridError)?.len() as i32;
        let height = lines.len() as i32;

        Ok(CharGrid {
            lines,
            width,
            height,
        })
    }
}

impl CharGrid {
    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y >= self.height || x < 0 || x >= self.width {
            return None;
        }
        self.lines[y as usize].chars().nth(x as usize)
    }

    pub fn get_neighbours(&self, x: i32, y: i32) -> Vec<((i32, i32), char)> {
        let mut res = Vec::with_capacity(8);
        for xi in x - 1..=x + 1 {
            for yi in y - 1..=y + 1 {
                if xi == x && yi == y {
                    continue;
                }
                if let Some(neighbour) = self.get(xi, yi) {
                    res.push(((xi, yi), neighbour))
                }
            }
        }
        res
    }

    pub fn find_coords(&self, to_find: &char) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y).unwrap() == *to_find {
                    res.push((x, y));
                }
            }
        }
        res
    }
}
