//! Demonstrates BufStats on a real WAV file with:
//! - selected statistics
//! - first derivative statistics
//! - optional frame weights
//!
//! ```sh
//! cargo run --example bufstats-demo
//! cargo run --example bufstats-demo -- path/to/file.wav
//! ```

use std::error::Error;
use std::path::Path;

use flucoma_rs::data::{BufStat, BufStats, BufStatsConfig, BufStatsSelect};
use wavers::Wav;

const DEFAULT_INPUT: &str =
    "vendor/flucoma-core/Resources/AudioFiles/Tremblay-AaS-AcousticStrums-M.wav";

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

    let selected_stats = vec![
        BufStat::Mean,
        BufStat::Std,
        BufStat::Low,
        BufStat::Mid,
        BufStat::High,
    ];
    let config = BufStatsConfig {
        select: BufStatsSelect::from_stats(&selected_stats),
        num_derivatives: 1,
        low_percentile: 5.0,
        middle_percentile: 50.0,
        high_percentile: 95.0,
        outliers_cutoff: Some(3.0),
        ..BufStatsConfig::default()
    };

    let mut stats = BufStats::new(config.clone())?;
    let unweighted = stats.process(&mono, mono.len(), 1, None)?;

    let weights = triangular_weights(mono.len());
    let weighted = stats.process(&mono, mono.len(), 1, Some(&weights))?;

    println!("\nSelected stats: {:?}", selected_stats);
    println!(
        "Derivatives: {} (0 = original, 1 = first difference)",
        config.num_derivatives
    );
    println!(
        "Percentiles: low/mid/high = {:.1}/{:.1}/{:.1}, outliers_cutoff = {:?}",
        config.low_percentile,
        config.middle_percentile,
        config.high_percentile,
        config.outliers_cutoff
    );

    println!("\nUnweighted BufStats:");
    print_channel_stats(
        unweighted.channel(0).ok_or("missing output channel 0")?,
        &selected_stats,
        config.num_derivatives as usize,
    );

    println!("\nWeighted BufStats (triangular weights):");
    print_channel_stats(
        weighted.channel(0).ok_or("missing output channel 0")?,
        &selected_stats,
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

fn print_channel_stats(values: &[f64], selected_stats: &[BufStat], num_derivatives: usize) {
    let stats_per_derivative = selected_stats.len();
    for d in 0..=num_derivatives {
        println!("  d{}:", d);
        for (s_idx, stat) in selected_stats.iter().enumerate() {
            let idx = d * stats_per_derivative + s_idx;
            println!("    {:>4}: {:>12.6}", stat_label(*stat), values[idx]);
        }
    }
}

fn stat_label(stat: BufStat) -> &'static str {
    match stat {
        BufStat::Mean => "mean",
        BufStat::Std => "std",
        BufStat::Skew => "skew",
        BufStat::Kurtosis => "kurt",
        BufStat::Low => "low",
        BufStat::Mid => "mid",
        BufStat::High => "high",
    }
}
