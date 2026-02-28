//! Demonstrates direct `MultiStats` usage on the bundled FluCoMa WAV.
//!
//! ```sh
//! cargo run --example multi-stats-demo
//! cargo run --example multi-stats-demo -- path/to/file.wav
//! ```

use std::error::Error;
use std::path::Path;

use flucoma_rs::data::{MultiStats, MultiStatsConfig};
use wavers::Wav;

const DEFAULT_INPUT: &str =
    "vendor/flucoma-core/Resources/AudioFiles/Tremblay-AaS-AcousticStrums-M.wav";
const STATS_LABELS: [&str; 7] = ["mean", "std", "skew", "kurt", "low", "mid", "high"];

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_INPUT.to_string());

    if !Path::new(&input_path).exists() {
        return Err(format!("Input WAV not found: {}", input_path).into());
    }

    let (mono, sample_rate, n_channels) = read_mono(&input_path)?;
    println!(
        "Input: `{}` ({} samples, {} Hz, {} ch)",
        input_path,
        mono.len(),
        sample_rate,
        n_channels
    );

    let config = MultiStatsConfig {
        num_derivatives: 2,
        low_percentile: 10.0,
        middle_percentile: 50.0,
        high_percentile: 90.0,
        outliers_cutoff: Some(3.0),
    };

    let mut ms = MultiStats::new(config.clone())?;
    let unweighted = ms.process(&mono, mono.len(), 1, None)?;
    let weights = triangular_weights(mono.len());
    let weighted = ms.process(&mono, mono.len(), 1, Some(&weights))?;

    println!(
        "\nConfig: derivatives={}, percentiles={:.1}/{:.1}/{:.1}, outliers_cutoff={:?}",
        config.num_derivatives,
        config.low_percentile,
        config.middle_percentile,
        config.high_percentile,
        config.outliers_cutoff
    );

    println!("\nUnweighted:");
    print_channel(
        unweighted.channel(0).ok_or("missing output channel 0")?,
        config.num_derivatives as usize,
    );

    println!("\nWeighted (triangular):");
    print_channel(
        weighted.channel(0).ok_or("missing output channel 0")?,
        config.num_derivatives as usize,
    );

    Ok(())
}

fn read_mono(path: &str) -> Result<(Vec<f64>, u32, u16), Box<dyn Error>> {
    let mut wav = Wav::<f32>::from_path(path)?;
    let sample_rate = wav.sample_rate() as u32;
    let n_channels = wav.n_channels();

    let mut mono = Vec::new();
    for frame in wav.frames() {
        let sum: f32 = frame.iter().copied().sum();
        mono.push(sum as f64 / n_channels as f64);
    }
    Ok((mono, sample_rate, n_channels))
}

fn triangular_weights(n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![1.0];
    }
    (0..n)
        .map(|i| {
            let x = i as f64 / (n - 1) as f64;
            1.0 - (2.0 * x - 1.0).abs()
        })
        .collect()
}

fn print_channel(values: &[f64], num_derivatives: usize) {
    for d in 0..=num_derivatives {
        println!("  d{}:", d);
        for (j, label) in STATS_LABELS.iter().enumerate() {
            let idx = d * STATS_LABELS.len() + j;
            println!("    {:>4}: {:>12.6}", label, values[idx]);
        }
    }
}
