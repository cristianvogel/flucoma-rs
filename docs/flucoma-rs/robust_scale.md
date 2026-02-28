# RobustScale

Module path: `flucoma_rs::data::RobustScale`

## Type

- `RobustScale`

## API

```rust
pub fn new(low_percentile: f64, high_percentile: f64) -> Result<RobustScale, &'static str>;
pub fn fit(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<(), &'static str>;
pub fn transform(&self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn inverse_transform(&self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn fit_transform(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn is_fitted(&self) -> bool;
```

## Notes

- Percentile-based scaling, generally more robust to outliers.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/robustscale/+page.svx`.

- RobustScale centers features by median and scales by percentile range (`low`..`high`).
- Because it relies on percentiles, it is less influenced by extreme outliers than mean/std or min/max scalers.
- Default interquartile-style settings (25/75) are useful for preserving the majority structure of noisy datasets.
