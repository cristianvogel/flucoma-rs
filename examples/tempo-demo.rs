//! Demonstrates offline tempo estimation from a WAV file.
//!
//! ```sh
//! cargo run --example tempo-demo
//! cargo run --example tempo-demo -- path/to/file.wav
//! ```

use std::error::Error;
use std::path::Path;

use flucoma_rs::analyzation::{TempoConfig, TempoEstimator, OnsetFunction};
use wavers::Wav;

const DEFAULT_INPUT: &str =
    "resources/audio_files/RN_808Beat_115.wav";

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_INPUT.to_string());

    if !Path::new(&input_path).exists() {
        return Err(format!("Input WAV not found: {}", input_path).into());
    }

    let (samples, sample_rate, n_channels, n_frames) = read_wav_channel_major(&input_path)?;
    println!(
        "Input: `{}` ({} samples, {} Hz, {} ch, {} frames)",
        input_path,
        samples.len(),
        sample_rate,
        n_channels,
        n_frames
    );

    let config = TempoConfig {
        threshold: 0.5,
        function: OnsetFunction::PowerSpectrum,
        ..TempoConfig::default()
    };

    let estimator = TempoEstimator::new(config);
    
    println!("Estimating tempo...");
    match estimator.estimate_with_details(&samples, n_frames, n_channels as usize, sample_rate as f64) {
        Ok(estimate) => {
            println!("\nEstimated Average Tempo: {:.2} BPM", estimate.bpm);
            println!("Confidence: {:.2}", estimate.confidence);
            println!(
                "Onsets: {} (rate {:.2} Hz, regularity {:.2}, novelty fallback: {})",
                estimate.onset_metrics.onset_count,
                estimate.onset_metrics.onset_rate_hz,
                estimate.onset_metrics.regularity,
                estimate.onset_metrics.used_novelty_fallback
            );
            if !estimate.alternatives.is_empty() {
                println!("Alternatives:");
                for alt in estimate.alternatives.iter().take(3) {
                    println!("  - {:.2} BPM (conf {:.2})", alt.bpm, alt.confidence);
                }
            }
        }
        Err(e) => {
            println!("\nTempo estimation failed: {}", e);
        }
    }

    Ok(())
}

fn read_wav_channel_major(path: &str) -> Result<(Vec<f64>, u32, u16, usize), Box<dyn Error>> {
    let mut wav = Wav::<f32>::from_path(path)?;
    let sample_rate = wav.sample_rate() as u32;
    let n_channels = wav.n_channels();
    
    let mut interleaved = Vec::new();
    for frame in wav.frames() {
        for &s in frame.iter() {
            interleaved.push(s as f64);
        }
    }
    
    let n_frames = interleaved.len() / n_channels as usize;
    let mut channel_major = vec![0.0; interleaved.len()];
    
    for f in 0..n_frames {
        for c in 0..n_channels as usize {
            channel_major[c * n_frames + f] = interleaved[f * n_channels as usize + c];
        }
    }

    Ok((channel_major, sample_rate, n_channels, n_frames))
}
