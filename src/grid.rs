use std::ops::Range;
use std::str::Split;
use crate::line::Line;


pub struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid { grid: vec![vec![0; height]; width] }
    }

    pub fn get_width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.grid.len()
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
        for i in l.get_x_range() {
            self.increment_at(i, l.p1.y);
        }
    }

    fn add_vertical(&mut self, l: &Line) {
        for j in l.get_y_range() {
            self.increment_at(l.p1.x, j);
        }
    }

    fn add_diagonal(&mut self, l: &Line) {
        let (p1, p2) = l.get_normal_ordered_points();

        for k in 0..l.get_x_len() {
            if p1.y < p2.y {
                self.increment_at(p1.x + k, p1.y + k);
            } else {
                self.increment_at(p1.x + k, p1.y - k);
            }
        }
    }

    pub fn get_overlapping_points(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                if self.get_value(i, j) > 1 {
                    sum += 1
                }
            }
        }
        sum
    }

    fn increment_at(&mut self, i: usize, j: usize) {
        self.grid[i][j] += 1
    }

    pub fn print(&self) {
        for j in 0..self.get_height() {
            for i in 0..self.get_width() {
                if self.get_value(i, j) == 0 {
                    print!(".")
                } else {
                    print!("{}", self.get_value(i, j))
                }
            }
            print!("\n");
        }
    }
}
