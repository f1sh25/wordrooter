use super::*;
use std::time::Instant;

#[test]
fn performance_find_chain_test() {
    let start = "haa".to_string();
    let letters = "hrajauam".to_string();

    let start_time = Instant::now();

    let result = wordrooter(start, letters).unwrap();
    let duration = start_time.elapsed();

    println!("Longest chain found ({} steps):", result.len());
    for word in &result {
        println!("  - {}", word);
    }
    println!("Time taken: {:.2?}", duration);
}
