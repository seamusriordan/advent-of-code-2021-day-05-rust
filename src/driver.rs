use crate::grid::Grid;
use crate::line::Line;

pub struct Driver {
    grid: Grid,
}


impl Driver {
    pub fn new(height: usize, width: usize) -> Driver {
        Driver {
            grid: Grid::new(height, width)
        }
    }

    pub fn add_inputs(&mut self, inputs: &[&str]) {
        for line_string in inputs {
            let line = Line::from_str(line_string);
            self.grid.add_line(line);
        }
    }

    pub fn get_overlapping_points(&self) -> usize {
        self.grid.get_overlapping_points()
    }

    pub fn print(&self) {
        self.grid.print();
    }
}