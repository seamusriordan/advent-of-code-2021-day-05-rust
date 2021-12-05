pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

pub struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid { grid: vec![vec![0; height]; width] }
    }

    pub fn get_value(&self, x: usize, y: usize) -> i32 {
        self.grid[x][y]
    }

    pub fn add_line(&mut self, l: Line) {
        if l.p1.y == l.p2.y {
            for i in l.p1.x..(l.p2.x + 1) {
                self.grid[i][l.p1.y] += 1;
            }
        } else {
            for j in l.p1.y..(l.p2.y + 1) {
                self.grid[l.p1.x][j] += 1;
            }
        }
    }
}