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
        let p1: Point;
        let p2: Point;

        if l.p1.x <= l.p2.x {
            p1 = l.p1;
            p2 = l.p2;
        } else {
            p1 = l.p2;
            p2 = l.p1;
        }

        if p1.y == p2.y {
            for i in p1.x..(p2.x + 1) {
                self.grid[i][p1.y] += 1;
            }
            return;
        }

        if p1.x == p2.x {
            let range;
            if p1.y < p2.y {
                range = p1.y..(p2.y + 1);
            } else {
                range = p2.y..(p1.y + 1);
            }
            for j in range {
                self.grid[p1.x][j] += 1;
            }
            return;
        }

        for k in 0..(p2.x - p1.x + 1) {
            if p1.y < p2.y {
                self.grid[p1.x + k][p1.y + k] += 1;
            } else {
                self.grid[p1.x + k][p1.y - k] += 1;
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
