# Loudness

Module path: `flucoma_rs::analyzation::Loudness`

## Types

- `Loudness`
- `LoudnessResult { loudness_db: f64, peak_db: f64 }`

## API

```rust
pub fn new(frame_size: usize, sample_rate: f64) -> Result<Loudness, &'static str>;
pub fn process_frame(&mut self, input: &[f64], k_weighting: bool, true_peak: bool) -> LoudnessResult;
pub fn frame_size(&self) -> usize;
```

## Notes

- `input.len()` must equal `frame_size`.
- Output contains integrated loudness and peak in dBFS.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/loudness/+page.svx`.

- Loudness follows EBU R128 ideas and reports loudness in LUFS plus true-peak in dBFS.
- The reference emphasizes perceptual weighting (K-weighting) and time-windowed analysis.
- It explains true-peak as accounting for inter-sample peaks, not just raw sample peaks.
