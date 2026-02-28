//! Compare PCA behavior across scaler choices.
//!
//! ```sh
//! cargo run --example pca-scaler-demo
//! ```

use flucoma_rs::data::{Pca, PcaConfig, PcaScaler};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rows = 32usize;
    let cols = 3usize;
    let data = build_dataset(rows);

    println!("PCA scaler comparison ({rows}x{cols})");
    println!("Dataset: correlated 3D trend + two strong outliers\n");

    let configs = [
        (
            "None",
            PcaConfig {
                whiten: false,
                scaler: PcaScaler::None,
            },
        ),
        (
            "Normalize",
            PcaConfig {
                whiten: false,
                scaler: PcaScaler::Normalize { min: 0.0, max: 1.0 },
            },
        ),
        (
            "Standardize",
            PcaConfig {
                whiten: false,
                scaler: PcaScaler::Standardize,
            },
        ),
        (
            "RobustScale",
            PcaConfig {
                whiten: false,
                scaler: PcaScaler::RobustScale {
                    low_percentile: 25.0,
                    high_percentile: 75.0,
                },
            },
        ),
    ];

    for (name, cfg) in configs {
        let mut pca = Pca::new(cfg)?;
        let (proj, explained) = pca.fit_transform(&data, rows, cols, 2)?;
        println!("{name:>12}: explained variance (k=2) = {explained:.5}");
        ascii_scatter_2d(name, &proj, 60, 16);
        println!();
    }

    Ok(())
}

fn build_dataset(rows: usize) -> Vec<f64> {
    let mut data = Vec::with_capacity(rows * 3);
    for i in 0..rows {
        let t = i as f64 / rows as f64;
        let x = 2.0 * t - 1.0;
        let y = 0.7 * x + (t * 8.0).sin() * 0.08;
        let z = 0.5 * x - 0.2 * y + (t * 6.0).cos() * 0.06;
        data.extend_from_slice(&[x, y, z]);
    }
    // Two outlier points to expose scaler behavior.
    if rows >= 6 {
        let a = 3 * (rows / 5);
        data[a] = 8.0;
        data[a + 1] = -6.0;
        data[a + 2] = 7.0;

        let b = 3 * ((rows * 4) / 5);
        data[b] = -7.0;
        data[b + 1] = 5.0;
        data[b + 2] = -6.5;
    }
    data
}

fn ascii_scatter_2d(name: &str, projected: &[f64], width: usize, height: usize) {
    let n = projected.len() / 2;
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        xs.push(projected[2 * i]);
        ys.push(projected[2 * i + 1]);
    }

    let xmin = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let xmax = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let ymin = ys.iter().copied().fold(f64::INFINITY, f64::min);
    let ymax = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let xspan = (xmax - xmin).max(1e-12);
    let yspan = (ymax - ymin).max(1e-12);

    let mut grid = vec![vec![' '; width]; height];
    for i in 0..n {
        let x = ((xs[i] - xmin) / xspan * (width as f64 - 1.0)).round() as usize;
        let y = ((ys[i] - ymin) / yspan * (height as f64 - 1.0)).round() as usize;
        let gy = height - 1 - y;
        grid[gy][x] = '*';
    }

    println!("  {name} 2D projection:");
    for row in grid {
        let s: String = row.into_iter().collect();
        println!("  |{s}|");
    }
}
