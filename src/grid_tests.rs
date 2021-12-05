#[cfg(test)]
mod grid_tests {
    use crate::grid::{Grid, Line, Point};

    #[test]
    fn grid_starts_off_empty() {
        let mut grid = Grid::new(1, 1);

        assert_eq!(0, grid.get_value(0, 0));
    }

    #[test]
    fn add_line_0_0_0_0_has_value_1() {
        let mut grid = Grid::new(1, 1);

        let line = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 0, y: 0 },
        };

        grid.add_line(line);

        assert_eq!(1, grid.get_value(0, 0));
    }

    #[test]
    fn add_line_horizontal_on_2x2_has_value_1s_on_line() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 1, y: 0 },
        };

        grid.add_line(line);

        assert_eq!(1, grid.get_value(0, 0));
        assert_eq!(1, grid.get_value(1, 0));
        assert_eq!(0, grid.get_value(0, 1));
        assert_eq!(0, grid.get_value(1, 1));
    }

    #[test]
    fn add_line_vertical_on_2x2_has_value_1s_on_line() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 0, y: 1 },
        };

        grid.add_line(line);

        assert_eq!(1, grid.get_value(0, 0));
        assert_eq!(0, grid.get_value(1, 0));
        assert_eq!(1, grid.get_value(0, 1));
        assert_eq!(0, grid.get_value(1, 1));
    }
}