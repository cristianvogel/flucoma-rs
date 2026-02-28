//! Demonstrates `RunningStats` on a small live-like vector stream.
//!
//! Shows:
//! - incremental mean + sample standard deviation updates
//! - finite history window behavior
//! - `clear()` reset behavior
//!
//! ```sh
//! cargo run --example running-stats-demo
//! ```

use flucoma_rs::data::RunningStats;

const HISTORY_SIZE: usize = 4;
const INPUT_SIZE: usize = 3;

fn main() {
    let mut rs = RunningStats::new(HISTORY_SIZE, INPUT_SIZE).expect("RunningStats::new");

    let stream = [
        [1.0, 10.0, -1.0],
        [2.0, 12.0, -2.0],
        [4.0, 14.0, -4.0],
        [8.0, 16.0, -8.0],
        [16.0, 18.0, -16.0],
    ];

    println!(
        "RunningStats demo (history_size={}, input_size={})",
        rs.history_size(),
        rs.input_size()
    );
    println!("\nStreaming updates:");

    for (step, frame) in stream.iter().enumerate() {
        let (mean, stddev) = rs.process(frame);
        print_step(step + 1, frame, mean, stddev);
    }

    println!("\nCalling clear()...");
    rs.clear();

    let reset_frame = [100.0, 0.0, -100.0];
    let (mean, stddev) = rs.process(&reset_frame);
    println!("\nAfter clear(), first frame re-initializes history:");
    print_step(1, &reset_frame, mean, stddev);
}

fn print_step(step: usize, input: &[f64], mean: &[f64], stddev: &[f64]) {
    println!("  step {step:>2}: input={}", fmt_vec(input));
    println!("           mean={}", fmt_vec(mean));
    println!("         stddev={}", fmt_vec(stddev));
}

fn fmt_vec(v: &[f64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| format!("{x:>8.3}")).collect();
    format!("[{} ]", parts.join(", "))
}
