#[cfg(test)]
mod grid_tests {
    use crate::grid::Grid;
    use crate::line::Line;
    use crate::point::Point;

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
    fn add_backwards_line_horizontal_on_2x2_has_value_1s_on_line() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 1, y: 0 },
            p2: Point { x: 0, y: 0 },
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

    #[test]
    fn add_backwards_line_vertical_on_2x2_has_value_1s_on_line() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 0, y: 1 },
            p2: Point { x: 0, y: 0 },
        };

        grid.add_line(line);

        assert_eq!(1, grid.get_value(0, 0));
        assert_eq!(0, grid.get_value(1, 0));
        assert_eq!(1, grid.get_value(0, 1));
        assert_eq!(0, grid.get_value(1, 1));
    }

    #[test]
    fn diagonal_lines_get_added() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 1, y: 1 },
        };

        grid.add_line(line);

        assert_eq!(1, grid.get_value(0, 0));
        assert_eq!(0, grid.get_value(1, 0));
        assert_eq!(0, grid.get_value(0, 1));
        assert_eq!(1, grid.get_value(1, 1));
    }

    #[test]
    fn reverse_diagonal_lines_get_added() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 1, y: 1 },
            p2: Point { x: 0, y: 0 },
        };

        grid.add_line(line);

        assert_eq!(1, grid.get_value(0, 0));
        assert_eq!(0, grid.get_value(1, 0));
        assert_eq!(0, grid.get_value(0, 1));
        assert_eq!(1, grid.get_value(1, 1));
    }

    #[test]
    fn backwards_diagonal_lines_get_added() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 1, y: 0 },
            p2: Point { x: 0, y: 1 },
        };

        grid.add_line(line);

        assert_eq!(0, grid.get_value(0, 0));
        assert_eq!(1, grid.get_value(1, 0));
        assert_eq!(1, grid.get_value(0, 1));
        assert_eq!(0, grid.get_value(1, 1));
    }

    #[test]
    fn backwards_reverse_diagonal_lines_get_added() {
        let mut grid = Grid::new(2, 2);

        let line = Line {
            p1: Point { x: 0, y: 1 },
            p2: Point { x: 1, y: 0 },
        };

        grid.add_line(line);

        assert_eq!(0, grid.get_value(0, 0));
        assert_eq!(1, grid.get_value(1, 0));
        assert_eq!(1, grid.get_value(0, 1));
        assert_eq!(0, grid.get_value(1, 1));
    }

    #[test]
    fn line_for_0_0_0_0_is_created() {
        let line = Line::from_str("0,0 -> 0,0");

        assert_eq!(0, line.p1.x);
        assert_eq!(0, line.p1.y);
        assert_eq!(0, line.p2.x);
        assert_eq!(0, line.p2.y);
    }

    #[test]
    fn line_for_2_3_4_4_is_created() {
        let line = Line::from_str("2,3 -> 4,5");

        assert_eq!(2, line.p1.x);
        assert_eq!(3, line.p1.y);
        assert_eq!(4, line.p2.x);
        assert_eq!(5, line.p2.y);
    }
}



