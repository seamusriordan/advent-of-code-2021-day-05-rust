use std::str::Split;
use std::ops::Range;
use crate::point::Point;

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


    pub fn get_x_range(&self) -> Range<usize> {
        self.p1.x_range_for(&self.p2)
    }

    pub fn get_x_len(&self) -> usize {
        self.get_x_range().len()
    }

    pub fn get_y_range(&self) -> Range<usize> {
        self.p1.y_range_for(&self.p2)
    }

    pub fn get_y_len(&self) -> usize {
        self.get_y_range().len()
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn get_normal_ordered_points(&self) -> (&Point, &Point) {
        if self.p1.x <= self.p2.x {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
        }
    }
}
