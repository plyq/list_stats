// Calculates mean, median and mode of a list of numbers
mod stats;

use stats::*;

fn main() {
    let mut list = [10, 1, 15, 30, 15, 15, 10, 3];
    println!("{}", mean(&list));
    println!("{}", median(&mut list));
    println!("{}", mode(&list, true));
}
