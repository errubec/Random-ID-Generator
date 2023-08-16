extern crate rand;
use radix_fmt::radix_36;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let random_digit = rand::thread_rng().gen_range(1..10);

    let seed_string = format!("{timestamp}{random_digit}")
        .chars().rev().collect::<String>();
    let seed_int = seed_string.parse::<usize>()
        .expect(&format!("Could not parse into usize: {seed_string}"));

    let generated_id = format!("{:#}", radix_36(seed_int));

    println!("{}", generated_id);
}