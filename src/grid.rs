use std::ops::Range;
use std::str::Split;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn from_iter(iter: &mut dyn Iterator<Item=&str>) -> Point {
        Point::from_str(
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    }

    pub fn from_str(s_p1: &str, s_p2: &str) -> Point {
        Point {
            x: s_p1.parse::<usize>().unwrap(),
            y: s_p2.parse::<usize>().unwrap(),
        }
    }

    pub fn x_span_for(&self, p: &Point) -> Range<usize> {
        let range;
        if self.x < p.x {
            range = self.x..(p.x + 1);
        } else {
            range = p.x..(self.x + 1);
        }
        range
    }

    pub fn y_span_for(&self, p: &Point) -> Range<usize> {
        let range;
        if self.y < p.y {
            range = self.y..(p.y + 1);
        } else {
            range = p.y..(self.y + 1);
        }
        range
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

fn split_at_comma<'a, 'b>(mut points: &'a mut Split<&'b str>) -> Split<'a, &'b str> {
    points.next().unwrap().split(",")
}

impl Line {
    pub fn from_str(s: &str) -> Line {
        let mut points = s.split(" -> ");

        Line {
            p1: Point::from_iter(&mut split_at_comma(&mut points)),
            p2: Point::from_iter(&mut split_at_comma(&mut points)),
        }
    }


    fn get_x_span(&self) -> Range<usize> {
        self.p1.x_span_for(&self.p2)
    }

    fn get_y_span(&self) -> Range<usize> {
        self.p1.y_span_for(&self.p2)
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn get_x_ordered_points(&self) -> (&Point, &Point) {
        if self.p1.x <= self.p2.x {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
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
        if l.is_horizontal() {
            self.add_horizontal(&l);
        } else if l.is_vertical() {
            self.add_vertical(&l);
        } else {
            self.add_diagonal(&l);
        }
    }

    fn add_horizontal(&mut self, l: &Line) {
        for i in l.get_x_span() {
            self.grid[i][l.p1.y] += 1;
        }
    }

    fn add_vertical(&mut self, l: &Line) {
        for j in l.get_y_span() {
            self.grid[l.p1.x][j] += 1;
        }
    }

    fn add_diagonal(&mut self, l: &Line) {
        let (p1, p2) = l.get_x_ordered_points();

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
