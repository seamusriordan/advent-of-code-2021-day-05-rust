use std::ops::Range;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

fn build_range(x1: usize, x2: usize) -> Range<usize> {
    if x1 < x2 {
        x1..x2+1
    } else {
        x2..x1+1
    }
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

    pub fn x_range_for(&self, p: &Point) -> Range<usize> {
        build_range(self.x, p.x)
    }

    pub fn y_range_for(&self, p: &Point) -> Range<usize> {
        build_range(self.y, p.y)
    }
}
