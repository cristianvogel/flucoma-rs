use crate::segmentation::OnsetSlice;
pub use crate::onset::OnsetFunction;

/// Configuration for [`TempoEstimator`].
#[derive(Debug, Clone)]
pub struct TempoConfig {
    pub window_size: usize,
    pub fft_size: usize,
    pub hop_size: usize,
    pub filter_size: usize,
    pub threshold: f64,
    pub debounce: usize,
    pub function: OnsetFunction,
    pub min_bpm: f64,
    pub max_bpm: f64,
}

impl Default for TempoConfig {
    fn default() -> Self {
        Self {
            window_size: 1024,
            fft_size: 1024,
            hop_size: 512,
            filter_size: 5,
            threshold: 0f64,
            debounce: 0,
            function: OnsetFunction::PowerSpectrum,
            min_bpm: 40.0,
            max_bpm: 240.0,
        }
    }
}

/// Offline tempo estimator.
pub struct TempoEstimator {
    config: TempoConfig,
}

/// Alternative tempo candidate with relative confidence.
#[derive(Debug, Clone)]
pub struct TempoAlternative {
    pub bpm: f64,
    pub confidence: f64,
}

/// Onset diagnostics used during tempo estimation.
#[derive(Debug, Clone)]
pub struct TempoOnsetMetrics {
    pub onset_count: usize,
    pub duration_seconds: f64,
    pub onset_rate_hz: f64,
    pub mean_ioi_seconds: Option<f64>,
    pub ioi_std_seconds: Option<f64>,
    pub regularity: f64,
    pub used_novelty_fallback: bool,
}

/// Detailed tempo estimation result.
#[derive(Debug, Clone)]
pub struct TempoEstimate {
    pub bpm: f64,
    pub confidence: f64,
    pub alternatives: Vec<TempoAlternative>,
    pub onset_metrics: TempoOnsetMetrics,
}

#[derive(Debug, Clone)]
struct OnsetDetectionResult {
    hops: Vec<usize>,
    used_novelty_fallback: bool,
}

#[derive(Debug, Clone)]
struct TempoVoteResult {
    bpm: f64,
    confidence: f64,
    alternatives: Vec<TempoAlternative>,
}

impl TempoEstimator {
    pub fn new(config: TempoConfig) -> Self {
        Self { config }
    }

    /// Estimate average tempo (BPM) from a multichannel buffer.
    ///
    /// Input layout is channel-major: `[ch0_samples..., ch1_samples..., ...]`.
    pub fn estimate(
        &self,
        input: &[f64],
        num_frames: usize,
        num_channels: usize,
        sample_rate: f64,
    ) -> Result<f64, &'static str> {
        self.estimate_with_details(input, num_frames, num_channels, sample_rate)
            .map(|estimate| estimate.bpm)
    }

    /// Estimate average tempo (BPM) from a multichannel buffer.
    ///
    /// Input layout is channel-major: `[ch0_samples..., ch1_samples..., ...]`.
    pub fn estimate_with_details(
        &self,
        input: &[f64],
        num_frames: usize,
        num_channels: usize,
        sample_rate: f64,
    ) -> Result<TempoEstimate, &'static str> {
        if num_frames == 0 || num_channels == 0 {
            return Err("input buffer is empty");
        }
        if sample_rate <= 0.0 {
            return Err("sample_rate must be > 0");
        }
        if input.len() != num_frames * num_channels {
            return Err("input length does not match num_frames * num_channels");
        }
        if self.config.window_size == 0
            || self.config.hop_size == 0
            || self.config.fft_size < self.config.window_size
        {
            return Err("invalid tempo analysis configuration");
        }
        if num_frames < self.config.window_size {
            return Err("input shorter than analysis window");
        }
        if self.config.min_bpm <= 0.0 || self.config.max_bpm <= self.config.min_bpm {
            return Err("invalid BPM range");
        }

        let n_hops = (num_frames - self.config.window_size) / self.config.hop_size + 1;

        // 1) Detect onset candidates.
        let onset_result = self.detect_onset_hops(input, num_frames, num_channels, n_hops)?;
        if onset_result.hops.len() < 2 {
            return Err("not enough onsets detected to estimate tempo");
        }

        // 2) Weighted IOI voting over multiple onset separations.
        let vote = self.estimate_bpm_from_onsets(&onset_result.hops, sample_rate)?;
        let onset_metrics = self.compute_onset_metrics(
            &onset_result.hops,
            num_frames,
            sample_rate,
            onset_result.used_novelty_fallback,
        );
        let coverage = (onset_metrics.onset_count as f64 / 16.0).clamp(0.0, 1.0);
        let fallback_penalty = if onset_metrics.used_novelty_fallback {
            0.95
        } else {
            1.0
        };
        let confidence =
            (0.65 * vote.confidence + 0.25 * onset_metrics.regularity + 0.10 * coverage)
                .clamp(0.0, 1.0)
                * fallback_penalty;

        Ok(TempoEstimate {
            bpm: vote.bpm,
            confidence,
            alternatives: vote.alternatives,
            onset_metrics,
        })
    }

    fn detect_onset_hops(
        &self,
        input: &[f64],
        num_frames: usize,
        num_channels: usize,
        n_hops: usize,
    ) -> Result<OnsetDetectionResult, &'static str> {
        // Try robust threshold sweep with the binary onset segmenter first.
        let mut thresholds = vec![self.config.threshold.max(0.0)];
        let t = self.config.threshold.max(0.0);
        thresholds.extend_from_slice(&[
            t * 0.75,
            t * 0.5,
            t * 0.35,
            t * 0.25,
            t * 0.18,
            t * 0.12,
            t * 0.08,
            t * 0.05,
            0.02,
            0.01,
            0.005,
        ]);
        thresholds.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        thresholds.dedup_by(|a, b| (*a - *b).abs() <= 1e-12);

        let mut best_seg = Vec::new();
        for threshold in thresholds {
            let mut all_hops = Vec::new();
            for c in 0..num_channels {
                let start = c * num_frames;
                let end = start + num_frames;
                let channel = &input[start..end];
                all_hops.extend(self.segment_channel_onsets(channel, n_hops, threshold)?);
            }
            let merged = merge_close_hops(all_hops, 3);
            if merged.len() > best_seg.len() {
                best_seg = merged.clone();
            }
            if merged.len() >= 8 {
                return Ok(OnsetDetectionResult {
                    hops: merged,
                    used_novelty_fallback: false,
                });
            }
        }
        if best_seg.len() >= 2 {
            return Ok(OnsetDetectionResult {
                hops: best_seg,
                used_novelty_fallback: false,
            });
        }

        // Fallback: adaptive peak-picking from continuous novelty.
        let mut fused_novelty = vec![0.0; n_hops];
        for c in 0..num_channels {
            let start = c * num_frames;
            let end = start + num_frames;
            let channel = &input[start..end];
            let novelty = self.compute_novelty(channel, n_hops)?;
            for (dst, val) in fused_novelty.iter_mut().zip(novelty) {
                if val > *dst {
                    *dst = val;
                }
            }
        }
        Ok(OnsetDetectionResult {
            hops: self.pick_onset_hops(&fused_novelty),
            used_novelty_fallback: true,
        })
    }

    fn segment_channel_onsets(
        &self,
        channel: &[f64],
        n_hops: usize,
        threshold: f64,
    ) -> Result<Vec<usize>, &'static str> {
        let mut seg = OnsetSlice::new(
            self.config.window_size,
            self.config.fft_size,
            self.config.filter_size,
        )?;

        let mut onsets = Vec::new();
        for h in 0..n_hops {
            let start = h * self.config.hop_size;
            let frame = &channel[start..start + self.config.window_size];
            let detected = seg.process_frame(
                frame,
                self.config.function,
                self.config.filter_size,
                threshold,
                self.config.debounce,
                0,
            );
            if detected > 0.5 {
                onsets.push(h);
            }
        }
        Ok(onsets)
    }

    fn compute_novelty(&self, channel: &[f64], n_hops: usize) -> Result<Vec<f64>, &'static str> {
        let mut odf = OnsetSlice::new(
            self.config.window_size,
            self.config.fft_size,
            self.config.filter_size,
        )?;
        let mut novelty = Vec::with_capacity(n_hops);
        for h in 0..n_hops {
            let start = h * self.config.hop_size;
            let frame = &channel[start..start + self.config.window_size];
            let val = odf.process_frame(frame, self.config.function, self.config.filter_size, 0.5, 1, 0);
            novelty.push(val.max(0.0));
        }
        Ok(novelty)
    }

    fn pick_onset_hops(&self, novelty: &[f64]) -> Vec<usize> {
        if novelty.len() < 3 {
            return Vec::new();
        }

        // Normalize by a robust high percentile so thresholding is scale-free.
        let mut sorted = novelty.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let p95 = percentile_sorted(&sorted, 0.95).max(1e-12);
        let normalized: Vec<f64> = novelty
            .iter()
            .map(|v| (v / p95).clamp(0.0, 4.0))
            .collect();

        let median = percentile_sorted(&sorted, 0.5);
        let mut abs_dev: Vec<f64> = novelty.iter().map(|v| (v - median).abs()).collect();
        abs_dev.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mad = percentile_sorted(&abs_dev, 0.5);
        let adaptive = ((median + 2.5 * mad) / p95).clamp(0.02, 1.5);

        let mut thresholds = vec![self.config.threshold.max(adaptive)];
        thresholds.extend_from_slice(&[0.6, 0.45, 0.35, 0.25, 0.18, 0.12, 0.08, 0.05]);
        thresholds.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        thresholds.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);

        let min_required = 8usize;
        let mut best = Vec::new();
        for t in thresholds {
            let peaks = pick_local_maxima(&normalized, t, self.config.debounce);
            if peaks.len() > best.len() {
                best = peaks.clone();
            }
            if peaks.len() >= min_required {
                return peaks;
            }
        }
        best
    }

    fn estimate_bpm_from_onsets(
        &self,
        onset_hops: &[usize],
        sample_rate: f64,
    ) -> Result<TempoVoteResult, &'static str> {
        let bin_size = 0.5;
        let n_bins = ((self.config.max_bpm - self.config.min_bpm) / bin_size).ceil() as usize + 1;
        let mut hist = vec![0.0; n_bins];
        let max_forward_pairs = 16usize;
        let min_period_secs = 60.0 / self.config.max_bpm;
        let max_period_secs = 60.0 / self.config.min_bpm;
        let preferred_bpm = 120.0f64.clamp(self.config.min_bpm, self.config.max_bpm);

        for i in 0..onset_hops.len() {
            let upper = (i + max_forward_pairs + 1).min(onset_hops.len());
            for j in i + 1..upper {
                let hop_delta = onset_hops[j].saturating_sub(onset_hops[i]);
                if hop_delta == 0 {
                    continue;
                }

                let period_secs = (hop_delta * self.config.hop_size) as f64 / sample_rate;
                if period_secs <= 0.0 {
                    continue;
                }
                if period_secs < min_period_secs || period_secs > max_period_secs {
                    continue;
                }
                let spacing = (j - i) as f64;
                let weight = 1.0 / spacing;

                let bpm = 60.0 / period_secs;
                let log2_dist = (bpm / preferred_bpm).log2().abs();
                let prior = (-0.5 * (log2_dist / 1.0).powi(2)).exp();

                let idx = ((bpm - self.config.min_bpm) / bin_size).round() as usize;
                if idx < hist.len() {
                    hist[idx] += weight * prior;
                }

                // Also support tactus interpretation from dense subdivisions.
                let half_bpm = bpm * 0.5;
                if bpm >= preferred_bpm * 1.5 && half_bpm >= self.config.min_bpm {
                    let half_idx =
                        ((half_bpm - self.config.min_bpm) / bin_size).round() as usize;
                    if half_idx < hist.len() {
                        hist[half_idx] += weight * prior * 0.7;
                    }
                }
            }
        }

        let (_, best_score) = hist
            .iter()
            .copied()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or("failed to build tempo histogram")?;
        if best_score <= 0.0 {
            return Err("no Inter-Onset Intervals within the specified BPM range");
        }

        // Pick the best tactus candidate using harmonic reinforcement.
        let mut combined_scores = vec![0.0; hist.len()];
        for (idx, combined_score) in combined_scores.iter_mut().enumerate() {
            let bpm = self.config.min_bpm + idx as f64 * bin_size;
            let mut combined = hist[idx];

            let double_bpm = bpm * 2.0;
            if double_bpm <= self.config.max_bpm {
                let double_idx = ((double_bpm - self.config.min_bpm) / bin_size).round() as usize;
                if double_idx < hist.len() {
                    combined += hist[double_idx] * 0.7;
                }
            }

            let half_bpm = bpm * 0.5;
            if half_bpm >= self.config.min_bpm {
                let half_idx = ((half_bpm - self.config.min_bpm) / bin_size).round() as usize;
                if half_idx < hist.len() {
                    combined += hist[half_idx] * 0.3;
                }
            }

            *combined_score = combined;
        }
        let (best_idx, best_combined) = combined_scores
            .iter()
            .copied()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or("failed to rank tempo candidates")?;
        if best_combined <= 0.0 {
            return Err("failed to rank tempo candidates");
        }

        // Refine by weighted average in a local neighborhood.
        let best_bpm = refine_histogram_peak(&hist, self.config.min_bpm, bin_size, best_idx)
            .ok_or("failed to refine tempo estimate")?;

        let mut rank: Vec<(usize, f64)> = combined_scores.iter().copied().enumerate().collect();
        rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let total_score: f64 = combined_scores.iter().sum();
        let second_score = rank.get(1).map(|(_, score)| *score).unwrap_or(0.0);
        let separation = ((best_combined - second_score) / best_combined).clamp(0.0, 1.0);
        let share = if total_score > 0.0 {
            (best_combined / total_score).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let vote_confidence = (0.6 * separation + 0.4 * share).clamp(0.0, 1.0);

        let mut alternatives = Vec::new();
        for (idx, score) in rank.into_iter().skip(1) {
            if alternatives.len() >= 3 || score <= 0.0 {
                break;
            }
            let bpm = match refine_histogram_peak(&hist, self.config.min_bpm, bin_size, idx) {
                Some(value) => value,
                None => continue,
            };
            if (bpm - best_bpm).abs() < 3.0 {
                continue;
            }
            let confidence = if total_score > 0.0 {
                (score / total_score).clamp(0.0, 1.0)
            } else {
                0.0
            };
            alternatives.push(TempoAlternative { bpm, confidence });
        }

        Ok(TempoVoteResult {
            bpm: best_bpm,
            confidence: vote_confidence,
            alternatives,
        })
    }

    fn compute_onset_metrics(
        &self,
        onset_hops: &[usize],
        num_frames: usize,
        sample_rate: f64,
        used_novelty_fallback: bool,
    ) -> TempoOnsetMetrics {
        let duration_seconds = num_frames as f64 / sample_rate;
        let onset_count = onset_hops.len();
        let onset_rate_hz = if duration_seconds > 0.0 {
            onset_count as f64 / duration_seconds
        } else {
            0.0
        };

        let iois: Vec<f64> = onset_hops
            .windows(2)
            .map(|pair| ((pair[1] - pair[0]) * self.config.hop_size) as f64 / sample_rate)
            .filter(|value| *value > 0.0)
            .collect();
        let (mean_ioi_seconds, ioi_std_seconds, regularity) = if iois.is_empty() {
            (None, None, 0.0)
        } else {
            let mean = iois.iter().sum::<f64>() / iois.len() as f64;
            let variance = iois
                .iter()
                .map(|ioi| {
                    let d = *ioi - mean;
                    d * d
                })
                .sum::<f64>()
                / iois.len() as f64;
            let std = variance.sqrt();
            let cv = if mean > 1e-12 { std / mean } else { 1.0 };
            let regularity = (1.0 / (1.0 + cv)).clamp(0.0, 1.0);
            (Some(mean), Some(std), regularity)
        };

        TempoOnsetMetrics {
            onset_count,
            duration_seconds,
            onset_rate_hz,
            mean_ioi_seconds,
            ioi_std_seconds,
            regularity,
            used_novelty_fallback,
        }
    }
}

fn pick_local_maxima(novelty: &[f64], threshold: f64, debounce_frames: usize) -> Vec<usize> {
    if novelty.len() < 3 {
        return Vec::new();
    }
    let min_spacing = debounce_frames.max(1);
    let mut peaks = Vec::new();
    let mut last_peak_idx: Option<usize> = None;

    for i in 1..novelty.len() - 1 {
        let center = novelty[i];
        let is_peak = center >= threshold && center >= novelty[i - 1] && center > novelty[i + 1];
        if !is_peak {
            continue;
        }

        if let Some(prev) = last_peak_idx {
            if i - prev < min_spacing {
                if center > novelty[prev] {
                    if let Some(last) = peaks.last_mut() {
                        *last = i;
                    }
                    last_peak_idx = Some(i);
                }
                continue;
            }
        }

        peaks.push(i);
        last_peak_idx = Some(i);
    }

    peaks
}

fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let clamped = p.clamp(0.0, 1.0);
    let idx = ((sorted.len() - 1) as f64 * clamped).round() as usize;
    sorted[idx]
}

fn merge_close_hops(mut hops: Vec<usize>, max_gap_hops: usize) -> Vec<usize> {
    if hops.is_empty() {
        return hops;
    }
    hops.sort_unstable();
    let mut merged = Vec::with_capacity(hops.len());
    for hop in hops {
        if let Some(last) = merged.last_mut() {
            if hop.saturating_sub(*last) <= max_gap_hops {
                *last = (*last + hop) / 2;
                continue;
            }
        }
        merged.push(hop);
    }
    merged
}

fn refine_histogram_peak(
    hist: &[f64],
    min_bpm: f64,
    bin_size: f64,
    center_idx: usize,
) -> Option<f64> {
    if hist.is_empty() {
        return None;
    }
    let lo = center_idx.saturating_sub(2);
    let hi = (center_idx + 2).min(hist.len() - 1);
    let mut sum_w = 0.0;
    let mut sum_bpm = 0.0;
    for (idx, score) in hist.iter().enumerate().take(hi + 1).skip(lo) {
        if *score > 0.0 {
            let bpm = min_bpm + idx as f64 * bin_size;
            sum_w += *score;
            sum_bpm += bpm * *score;
        }
    }
    if sum_w <= 0.0 {
        None
    } else {
        Some(sum_bpm / sum_w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempo_estimation_on_metronome() {
        let sample_rate = 44100.0;
        let bpm = 120.0;
        let beat_interval = 60.0 / bpm;
        let samples_per_beat = (beat_interval * sample_rate) as usize;
        
        let num_beats = 16;
        let num_frames = num_beats * samples_per_beat + 1024;
        let mut data = vec![0.0; num_frames];
        
        // Add clicks
        for i in 0..num_beats {
            let pos = i * samples_per_beat;
            data[pos] = 1.0; 
        }

        let estimator = TempoEstimator::new(TempoConfig::default());
        let estimated_bpm = estimator.estimate(&data, num_frames, 1, sample_rate).unwrap();
        
        // Allow some margin because of hop-size quantization
        assert!((estimated_bpm - bpm).abs() < 5.0, "Expected ~120 BPM, got {}", estimated_bpm);
    }

    #[test]
    fn tempo_estimation_with_sub_beats() {
        let sample_rate = 44100.0;
        let main_bpm = 100.0;
        let main_interval = 60.0 / main_bpm;
        let samples_per_main = (main_interval * sample_rate) as usize;
        
        let num_main_beats = 16;
        let num_frames = num_main_beats * samples_per_main + 1024;
        let mut data = vec![0.0; num_frames];
        
        // Add main beats
        for i in 0..num_main_beats {
            let pos = i * samples_per_main;
            data[pos] = 1.0; 
        }
        
        // Add some random sub-beats (e.g. 16th notes) that are weaker
        // or just scattered, to see if clustering works.
        // If we add an 8th note every beat, we want to see if it still picks 100 or 200.
        // Usually we want the most prominent period.
        for i in 0..num_main_beats {
            let pos = i * samples_per_main + samples_per_main / 2;
            if i % 3 == 0 { // Only some 8th notes
                 data[pos] = 0.5;
            }
        }

        let estimator = TempoEstimator::new(TempoConfig::default());
        let estimated_bpm = estimator.estimate(&data, num_frames, 1, sample_rate).unwrap();
        
        // It should still favor 100 BPM because there are more 100 BPM intervals than 50 BPM ones
        // or 200 BPM ones in this specific setup.
        assert!((estimated_bpm - main_bpm).abs() < 5.0, "Expected ~100 BPM, got {}", estimated_bpm);
    }

    #[test]
    fn tempo_estimation_with_details_exposes_quality_metrics() {
        let sample_rate = 44100.0;
        let bpm = 120.0;
        let beat_interval = 60.0 / bpm;
        let samples_per_beat = (beat_interval * sample_rate) as usize;
        let num_beats = 16;
        let num_frames = num_beats * samples_per_beat + 1024;
        let mut data = vec![0.0; num_frames];
        for i in 0..num_beats {
            let pos = i * samples_per_beat;
            data[pos] = 1.0;
        }

        let estimator = TempoEstimator::new(TempoConfig::default());
        let estimate = estimator
            .estimate_with_details(&data, num_frames, 1, sample_rate)
            .unwrap();

        assert!((estimate.bpm - bpm).abs() < 5.0, "Expected ~120 BPM, got {}", estimate.bpm);
        assert!(
            (0.0..=1.0).contains(&estimate.confidence),
            "confidence out of range: {}",
            estimate.confidence
        );
        assert!(
            estimate.onset_metrics.onset_count >= 8,
            "expected several onsets, got {}",
            estimate.onset_metrics.onset_count
        );
        assert!(
            estimate.onset_metrics.mean_ioi_seconds.is_some(),
            "mean IOI should be available"
        );
        assert!(
            estimate.onset_metrics.regularity > 0.45,
            "expected regular pulse, got regularity {}",
            estimate.onset_metrics.regularity
        );
        for alt in &estimate.alternatives {
            assert!(
                (0.0..=1.0).contains(&alt.confidence),
                "alternative confidence out of range: {}",
                alt.confidence
            );
        }
    }

}
