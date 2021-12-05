#[cfg(test)]
mod driver_tests {
    use crate::driver::Driver;

    #[test]
    fn one_lines_0_0_0_0_gets_processed() {
        let input = vec![
            "0,0 -> 0,0",
        ];

        let mut driver = Driver::new(1, 1);

        driver.add_inputs(&input);

        assert_eq!(0, driver.get_overlapping_points());
    }

    #[test]
    fn overlapping_lines_0_0_0_0_gets_processed() {
        let input = vec![
            "0,0 -> 0,0",
            "0,0 -> 0,0",
        ];

        let mut driver = Driver::new(1, 1);

        driver.add_inputs(&input);

        assert_eq!(1, driver.get_overlapping_points());
    }

    #[test]
    fn example_input_gives_5() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let mut driver = Driver::new(10, 10);

        driver.add_inputs(&input);

        driver.print();

        assert_eq!(12, driver.get_overlapping_points());
    }
}