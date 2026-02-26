//! Detects onsets in an audio file, computes a mean mel-band vector per slice, and writes up to
//! N timbrally unique slices as individual WAV files.
//!
//! Slices that are too similar to an already-kept slice are skipped.
//!
//! ```sh
//! cargo run --example unique-slices -- input.wav [topN]
//! ```
//!
//! Output: `<input_stem>_slices/slice1_<start>_<end>.wav`, etc.

use std::error::Error;
use std::path::Path;

use wavers::Wav;

use flucoma_rs::{
    analyzation::{MelBands, OnsetFunction, Stft, WindowType},
    segmentation::OnsetSegmentation,
};

// -------------------------------------------------------------------------------------------------
// OnsetSegmentation & MelBands config

const WINDOW_SIZE: usize = 1024;
const HOP_SIZE: usize = WINDOW_SIZE / 2;
const FFT_SIZE: usize = 4096;
const FILTER_SIZE: usize = 5;
const NUM_MEL_BANDS: usize = 40;
const MIN_FREQ_HZ: f64 = 20.0;
const ONSET_FUNCTION: OnsetFunction = OnsetFunction::PowerSpectrum;
const ONSET_THRESHOLD: f64 = 0.0;
const ONSET_DEBOUNCE: usize = 0;
const MIN_SLICE_SAMPLES: usize = 2048;
const SIMILARITY_THRESHOLD: f64 = 0.15;

// -------------------------------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: unique-slices <input.wav> [topN]");
        std::process::exit(1);
    }
    let input_path = args.get(1).unwrap().as_str();
    let top_n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    let (mono, sample_rate, n_channels) = read_mono(input_path)?;
    println!(
        "Read `{}`: {} samples, {} Hz, {} ch",
        input_path,
        mono.len(),
        sample_rate,
        n_channels
    );

    let boundaries = detect_onsets(&mono, sample_rate);
    println!("Detected {} onset boundaries", boundaries.len());

    // Build slices from consecutive boundaries; discard slices shorter than MIN_SLICE_SAMPLES
    let slices: Vec<Slice> = boundaries
        .windows(2)
        .filter_map(|w| {
            let (start, end) = (w[0], w[1]);
            if end - start < MIN_SLICE_SAMPLES {
                return None;
            }
            let mel = mean_mel(&mono, start, end, sample_rate);
            Some(Slice { start, end, mel })
        })
        .collect();

    println!(
        "{} slices after filtering (min {} samples)",
        slices.len(),
        MIN_SLICE_SAMPLES
    );

    if slices.is_empty() {
        println!("No slices found; nothing to write.");
        return Ok(());
    }

    let k = top_n.min(slices.len());
    let selected = deduplicate(&slices, k);

    // Create output folder
    let input_p = Path::new(input_path);
    let parent = input_p.parent().unwrap_or(Path::new("."));
    let stem = input_p
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("audio");
    let out_dir = parent.join(format!("{}_slices", stem));
    std::fs::create_dir_all(&out_dir)?;

    // Remove previously written slices matching slice*.wav
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("slice") && name.ends_with(".wav") {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }

    // Read original file as interleaved i16 for slice writing
    let raw_samples = read_interleaved_i16(input_path)?;

    println!(
        "\nWriting {} slices to `{}/`:",
        selected.len(),
        out_dir.display()
    );

    for (rank, &idx) in selected.iter().enumerate() {
        let sl = &slices[idx];
        let fname = format!("slice{}_{:08}_{:08}.wav", rank + 1, sl.start, sl.end);
        let out_path = out_dir.join(&fname);

        let sample_start = sl.start * n_channels as usize;
        let sample_end = sl.end * n_channels as usize;
        let slice_data = &raw_samples[sample_start..sample_end.min(raw_samples.len())];

        wavers::write(
            out_path.to_str().unwrap(),
            slice_data,
            sample_rate as i32,
            n_channels,
        )?;

        println!(
            "  [{}] {} -- samples {}..{} ({:.3}s)",
            rank + 1,
            fname,
            sl.start,
            sl.end,
            (sl.end - sl.start) as f64 / sample_rate as f64,
        );
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

/// Read a WAV file and mix all channels to mono f64.
/// Returns `(mono_samples, sample_rate, n_channels)`.
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

/// Read a WAV file as interleaved i16 samples (all channels).
fn read_interleaved_i16(path: &str) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut wav = Wav::<i16>::from_path(path)?;
    let mut samples = Vec::new();
    for frame in wav.frames() {
        for &s in frame.iter() {
            samples.push(s);
        }
    }
    Ok(samples)
}

// -------------------------------------------------------------------------------------------------

/// Run `OnsetSegmentation` hop-by-hop and return a sorted list of sample boundaries
/// (always includes 0 and `mono.len()` as the outer sentinels).
fn detect_onsets(mono: &[f64], sample_rate: u32) -> Vec<usize> {
    let _ = sample_rate; // not needed by the API but kept for documentation

    let mut seg =
        OnsetSegmentation::new(WINDOW_SIZE, FFT_SIZE, FILTER_SIZE).expect("OnsetSegmentation::new");

    let mut boundaries = vec![0usize];
    let mut frame = vec![0.0f64; WINDOW_SIZE];

    let n_hops = mono.len().saturating_sub(WINDOW_SIZE) / HOP_SIZE + 1;
    for hop in 0..n_hops {
        let start = hop * HOP_SIZE;
        for i in 0..WINDOW_SIZE {
            frame[i] = if start + i < mono.len() {
                mono[start + i]
            } else {
                0.0
            };
        }

        let onset = seg.process_frame(
            &frame,
            ONSET_FUNCTION,
            FILTER_SIZE,
            ONSET_THRESHOLD,
            ONSET_DEBOUNCE,
            0,
        );

        if onset == 1.0 {
            boundaries.push(start);
        }
    }

    boundaries.push(mono.len());
    boundaries.sort_unstable();
    boundaries.dedup();
    boundaries
}

// -------------------------------------------------------------------------------------------------

/// Compute the mean mel-band vector for a slice `mono[start..end]`.
fn mean_mel(mono: &[f64], start: usize, end: usize, sample_rate: u32) -> Vec<f64> {
    let hi_hz = sample_rate as f64 / 2.0;
    let n_bins = FFT_SIZE / 2 + 1;

    let mut stft = Stft::new(WINDOW_SIZE, FFT_SIZE, HOP_SIZE, WindowType::Hann).expect("Stft::new");
    let mut mel = MelBands::new(
        NUM_MEL_BANDS,
        n_bins,
        MIN_FREQ_HZ,
        hi_hz,
        sample_rate as f64,
        WINDOW_SIZE,
    )
    .expect("MelBands::new");

    let mut accumulator = vec![0.0f64; NUM_MEL_BANDS];
    let mut count = 0usize;
    let mut frame = vec![0.0f64; WINDOW_SIZE];

    let slice_len = end.saturating_sub(start);
    let n_hops = slice_len.saturating_sub(WINDOW_SIZE) / HOP_SIZE + 1;

    for hop in 0..n_hops {
        let pos = start + hop * HOP_SIZE;
        for i in 0..WINDOW_SIZE {
            frame[i] = if pos + i < end && pos + i < mono.len() {
                mono[pos + i]
            } else {
                0.0
            };
        }

        let spec = stft.process_frame(&frame);
        let mags = spec.magnitudes();
        let bands = mel.process_frame(&mags, false, true, false);

        for (a, b) in accumulator.iter_mut().zip(bands.iter()) {
            *a += b;
        }
        count += 1;
    }

    if count > 0 {
        for v in &mut accumulator {
            *v /= count as f64;
        }
    }

    accumulator
}

// -------------------------------------------------------------------------------------------------

struct Slice {
    start: usize,
    end: usize,
    mel: Vec<f64>,
}

fn pearson_dist(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;
    let num: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - mean_a) * (y - mean_b))
        .sum();
    let den_a: f64 = a.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>().sqrt();
    let den_b: f64 = b.iter().map(|y| (y - mean_b).powi(2)).sum::<f64>().sqrt();
    let denom = den_a * den_b;
    if denom < 1e-12 {
        return 1.0; // treat constant vectors as maximally distant
    }
    1.0 - (num / denom).clamp(-1.0, 1.0)
}

/// Keep up to `k` slices by deduplication: walk in temporal order and skip any
/// slice whose Pearson distance to an already-kept slice is below the threshold.
fn deduplicate(slices: &[Slice], k: usize) -> Vec<usize> {
    let mut kept: Vec<usize> = Vec::with_capacity(k);
    for i in 0..slices.len() {
        if kept.len() >= k {
            break;
        }
        let too_similar = kept
            .iter()
            .any(|&j| pearson_dist(&slices[i].mel, &slices[j].mel) < SIMILARITY_THRESHOLD);
        if !too_similar {
            kept.push(i);
        }
    }
    kept
}
