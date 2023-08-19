use radix_fmt::radix_36;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Got system time before Unix Epoch")
        .as_millis(); // u128

    let seed: String = (timestamp / 32 - 1).to_string() // does not end in 0
        .chars().rev().collect(); // does not start with 0
    let seed: u64 = seed.parse().expect(
        &format!("Could not parse '{seed}' into u64")
    );

    println!("{:#}", radix_36(seed)); // Generated ID
}