//! Demonstrates `Normalize` on a tiny dataset matrix.
//!
//! ```sh
//! cargo run --example normalize-demo
//! ```

use flucoma_rs::data::Normalize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 4 rows x 3 cols, row-major
    let data = vec![
        10.0, 100.0, -1.0, //
        20.0, 200.0, 0.0, //
        30.0, 300.0, 1.0, //
        40.0, 400.0, 2.0,
    ];
    let rows = 4;
    let cols = 3;

    let mut norm = Normalize::new(0.0, 1.0)?;
    let normalized = norm.fit_transform(&data, rows, cols)?;
    let restored = norm.inverse_transform(&normalized, rows, cols)?;

    println!("Original:");
    print_matrix(&data, rows, cols);

    println!("\nNormalized [0, 1]:");
    print_matrix(&normalized, rows, cols);

    println!("\nInverse transformed:");
    print_matrix(&restored, rows, cols);

    let max_abs_err = data
        .iter()
        .zip(restored.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);
    println!("\nMax reconstruction error: {:.6e}", max_abs_err);

    Ok(())
}

fn print_matrix(data: &[f64], rows: usize, cols: usize) {
    for r in 0..rows {
        let start = r * cols;
        let end = start + cols;
        let row = &data[start..end];
        println!("  [{:>10.4}, {:>10.4}, {:>10.4}]", row[0], row[1], row[2]);
    }
}
