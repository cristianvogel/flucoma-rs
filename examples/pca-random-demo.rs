//! PCA demo on deterministic pseudo-random source data.
//!
//! ```sh
//! cargo run --example pca-random-demo
//! ```

use flucoma_rs::data::{Pca, PcaConfig, PcaScaler};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rows = 128usize;
    let cols = 6usize;
    let target_dims = 2usize;

    let data = build_random_correlated_dataset(rows, cols, 0xC0FFEE_u64);

    let mut pca = Pca::new(PcaConfig {
        whiten: false,
        scaler: PcaScaler::RobustScale {
            low_percentile: 25.0,
            high_percentile: 75.0,
        },
    })?;

    let (projected, explained) = pca.fit_transform(&data, rows, cols, target_dims)?;
    let reconstructed = pca.inverse_transform(&projected, rows, target_dims)?;

    println!("PCA random-source demo");
    println!("  rows={rows}, cols={cols}, target_dims={target_dims}");
    println!("  scaler=RobustScale(25,75), whiten=false");
    println!("  explained variance ratio (k={target_dims}): {explained:.6}");
    println!("  reconstruction RMSE: {:.6}", rmse(&data, &reconstructed));

    println!("\nFirst 5 projected points (2D):");
    for i in 0..5 {
        println!(
            "  {:>3}: [{:>9.5}, {:>9.5}]",
            i,
            projected[2 * i],
            projected[2 * i + 1]
        );
    }

    Ok(())
}

fn build_random_correlated_dataset(rows: usize, cols: usize, seed: u64) -> Vec<f64> {
    let mut rng = Lcg::new(seed);
    let mut out = Vec::with_capacity(rows * cols);
    for _ in 0..rows {
        // Latent factors create meaningful low-dimensional structure.
        let f1 = rng.next_signed();
        let f2 = rng.next_signed() * 0.5;
        for c in 0..cols {
            let w1 = 1.0 - (c as f64 / cols as f64) * 0.6;
            let w2 = (c as f64 / cols as f64) * 0.8 - 0.2;
            let noise = rng.next_signed() * 0.05;
            out.push(w1 * f1 + w2 * f2 + noise);
        }
    }
    out
}

fn rmse(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len()).max(1);
    let mse = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let d = x - y;
            d * d
        })
        .sum::<f64>()
        / n as f64;
    mse.sqrt()
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        // Numerical Recipes-style LCG constants.
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        let v = self.next_u64() >> 11; // 53 bits
        (v as f64) / ((1u64 << 53) as f64)
    }

    fn next_signed(&mut self) -> f64 {
        self.next_f64() * 2.0 - 1.0
    }
}
