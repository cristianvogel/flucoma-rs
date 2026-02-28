# Standardize

Module path: `flucoma_rs::data::Standardize`

## Type

- `Standardize`

## API

```rust
pub fn new() -> Result<Standardize, &'static str>;
pub fn fit(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<(), &'static str>;
pub fn transform(&self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn inverse_transform(&self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn fit_transform(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn is_fitted(&self) -> bool;
```

## Notes

- Performs z-score standardization per feature.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/standardize/+page.svx`.

- Standardize makes each feature mean-centered (0) with unit standard deviation (1).
- It is often beneficial for ML algorithms that assume comparable feature scales.
- The reference cautions that non-normal data may favour alternatives like Normalize or RobustScale.
