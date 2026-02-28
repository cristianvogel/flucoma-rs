//! Compare `Normalize`, `Standardize`, and `RobustScale` on 1D data with outliers.
//!
//! Inspired by: <https://learn.flucoma.org/learn/comparing-scalers/>
//!
//! ```sh
//! cargo run --example compare-scalers
//! cargo run --example compare-scalers -- --n 60 --outlier 40
//! ```

use flucoma_rs::data::{Normalize, RobustScale, Standardize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::from_args(std::env::args().skip(1).collect());
    let raw = build_series(cfg.n_points, cfg.outlier_strength);
    let rows = raw.len();
    let cols = 1usize;

    let mut norm = Normalize::new(0.0, 1.0)?;
    let normalized = norm.fit_transform(&raw, rows, cols)?;

    let mut std = Standardize::new()?;
    let standardized = std.fit_transform(&raw, rows, cols)?;

    let mut robust = RobustScale::new(25.0, 75.0)?;
    let robust_scaled = robust.fit_transform(&raw, rows, cols)?;

    println!(
        "Scaler comparison (n={}, outlier={})",
        cfg.n_points, cfg.outlier_strength
    );
    println!("Reference: https://learn.flucoma.org/learn/comparing-scalers/");
    println!("Data pattern: centered signal with two extreme outliers (positive and negative)\n");

    print_summary("Raw", &raw);
    print_summary("Normalize [0,1]", &normalized);
    print_summary("Standardize (z)", &standardized);
    print_summary("RobustScale (25-75)", &robust_scaled);

    println!("\nASCII strip-plots (one line per sample):");
    plot_series("Raw", &raw, cfg.plot_width);
    plot_series("Normalize [0,1]", &normalized, cfg.plot_width);
    plot_series("Standardize (z)", &standardized, cfg.plot_width);
    plot_series("RobustScale (25-75)", &robust_scaled, cfg.plot_width);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Config {
    n_points: usize,
    outlier_strength: f64,
    plot_width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            n_points: 24,
            outlier_strength: 20.0,
            plot_width: 58,
        }
    }
}

impl Config {
    fn from_args(args: Vec<String>) -> Self {
        let mut cfg = Self::default();
        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--n" => {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                        cfg.n_points = v.max(8);
                    }
                    i += 1;
                }
                "--outlier" => {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        cfg.outlier_strength = v.max(1.0);
                    }
                    i += 1;
                }
                "--width" => {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                        cfg.plot_width = v.max(20);
                    }
                    i += 1;
                }
                _ => {}
            }
            i += 1;
        }
        cfg
    }
}

fn build_series(n: usize, outlier_strength: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        // Smooth central signal around zero.
        let x = i as f64 / n as f64;
        let base = (x * std::f64::consts::TAU * 1.5).sin() * 0.8 + (x * 12.0).cos() * 0.15;
        v.push(base);
    }
    // Inject two outliers similar to scaler-comparison demonstrations.
    if n >= 4 {
        v[n / 4] = outlier_strength;
        v[(3 * n) / 4] = -outlier_strength * 0.8;
    }
    v
}

fn print_summary(name: &str, values: &[f64]) {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let median = sorted[(sorted.len() - 1) / 2];
    let q1 = sorted[((sorted.len() - 1) as f64 * 0.25).round() as usize];
    let q3 = sorted[((sorted.len() - 1) as f64 * 0.75).round() as usize];
    println!(
        "{name:>20}: min={min:>9.4}  q1={q1:>9.4}  median={median:>9.4}  q3={q3:>9.4}  max={max:>9.4}"
    );
}

fn plot_series(name: &str, values: &[f64], width: usize) {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (max - min).max(1e-12);
    println!("\n{name}");
    println!("  axis: [{min:.4}, {max:.4}]");
    for (i, &v) in values.iter().enumerate() {
        let t = ((v - min) / span).clamp(0.0, 1.0);
        let pos = (t * (width - 1) as f64).round() as usize;
        let mut line = vec![' '; width];
        line[pos] = '*';
        let line: String = line.into_iter().collect();
        println!("  {:>3} |{}| {:>10.5}", i, line, v);
    }
}
