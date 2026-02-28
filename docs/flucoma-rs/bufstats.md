# BufStats

Module path: `flucoma_rs::data::{BufStats, BufStatsConfig, BufStatsSelect, BufStat, BufStatsOutput}`

## Types

- `BufStat`: `Mean | Std | Skew | Kurtosis | Low | Mid | High`
- `BufStatsSelect`
- `BufStatsConfig`
- `BufStatsOutput`
- `BufStats`

## Select API

```rust
pub fn all() -> BufStatsSelect;
pub fn from_stats(stats: &[BufStat]) -> BufStatsSelect;
```

## BufStats API

```rust
pub fn new(config: BufStatsConfig) -> Result<BufStats, &'static str>;
pub fn config(&self) -> &BufStatsConfig;
pub fn set_config(&mut self, config: BufStatsConfig) -> Result<(), &'static str>;

pub fn process(
    &mut self,
    source: &[f64],
    source_num_frames: usize,
    source_num_channels: usize,
    weights: Option<&[f64]>,
) -> Result<BufStatsOutput, &'static str>;
```

## Output API

```rust
pub fn values(&self) -> &[f64];
pub fn num_channels(&self) -> usize;
pub fn values_per_channel(&self) -> usize;
pub fn channel(&self, channel: usize) -> Option<&[f64]>;
```

## Notes

- Input layout is channel-major: `[ch0_frames..., ch1_frames..., ...]`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/bufstats/+page.svx`.

- BufStats summarizes time-series channels with mean/std/skew/kurtosis and percentile stats.
- Derivative statistics (`numDerivs`) capture how descriptors change over time, not just absolute levels.
- Supports optional outlier filtering and per-frame weighting to bias summaries toward musically relevant moments.
