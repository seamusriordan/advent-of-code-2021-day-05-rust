use std::fs;
use day_05_rust::driver::Driver;

fn main() {
    let input_lines = fs::read_to_string("input.txt").unwrap();

    let mut driver = Driver::new(1000,1000);

    driver.add_inputs(&input_lines.lines().collect::<Vec<&str>>());

    print!("{}\n", driver.get_overlapping_points());
}
