pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn from_str(s: &str) -> Line {
        let mut points = s.split(" -> ");
        let mut p1 = points.next().unwrap().split(",");
        let mut p2 = points.next().unwrap().split(",");

        Line {
            p1: Point {
                x: p1.next().unwrap().parse::<usize>().unwrap(),
                y: p1.next().unwrap().parse::<usize>().unwrap(),
            },
            p2: Point {
                x: p2.next().unwrap().parse::<usize>().unwrap(),
                y: p2.next().unwrap().parse::<usize>().unwrap(),
            },
        }
    }
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
        if l.p1.y == l.p2.y && l.p1.x == l.p2.x {
            self.grid[l.p1.x][l.p1.y] += 1;
        }

        if l.p1.y == l.p2.y && l.p1.x < l.p2.x {
            for i in l.p1.x..(l.p2.x + 1) {
                self.grid[i][l.p1.y] += 1;
            }
        }

        if l.p1.y == l.p2.y && l.p1.x > l.p2.x {
            for i in l.p2.x..(l.p1.x + 1) {
                self.grid[i][l.p1.y] += 1;
            }
        }


        if l.p1.x == l.p2.x && l.p1.y < l.p2.y {
            for j in l.p1.y..(l.p2.y + 1) {
                self.grid[l.p1.x][j] += 1;
            }
        }

        if l.p1.x == l.p2.x && l.p1.y > l.p2.y {
            for j in l.p2.y..(l.p1.y + 1) {
                self.grid[l.p1.x][j] += 1;
            }
        }


        if l.p1.x < l.p2.x && l.p1.y < l.p2.y {
            let mut k = 0;
            while l.p1.x + k <= l.p2.x {
                self.grid[l.p1.x + k][l.p1.y + k] += 1;
                k += 1;
            }
        }


        if l.p1.x > l.p2.x && l.p1.y < l.p2.y {
            let mut k = 0;
            while l.p2.x + k <= l.p1.x {
                self.grid[l.p2.x + k][l.p2.y - k] += 1;
                k += 1;
            }
        }

        if l.p1.x < l.p2.x && l.p1.y > l.p2.y {
            let mut k = 0;
            while l.p1.x + k <= l.p2.x {
                self.grid[l.p1.x + k][l.p1.y - k] += 1;
                k += 1;
            }
        }


        if l.p1.x > l.p2.x && l.p1.y > l.p2.y {
            let mut k = 0;
            while l.p2.x + k <= l.p1.x {
                self.grid[l.p2.x + k][l.p2.y + k] += 1;
                k += 1;
            }
        }
    }

    pub fn get_points_above(&self, n: i32) -> usize {
        let mut sum = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] > n {
                    sum += 1
                }
            }
        }
        sum
    }

    pub fn print(&self) {
        for j in 0..self.grid[0].len() {
            for i in 0..self.grid.len() {
                if self.grid[i][j] == 0 {
                    print!(".")
                } else {
                    print!("{}", self.grid[i][j])
                }
            }
            print!("\n");
        }
    }
}
