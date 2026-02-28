# Normalize

Module path: `flucoma_rs::data::Normalize`

## Type

- `Normalize`

## API

```rust
pub fn new(min: f64, max: f64) -> Result<Normalize, &'static str>;
pub fn fit(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<(), &'static str>;
pub fn transform(&self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn inverse_transform(&self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn fit_transform(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<Vec<f64>, &'static str>;
pub fn is_fitted(&self) -> bool;
```

## Notes

- Matrix layout is row-major: `[row0..., row1..., ...]`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/normalize/+page.svx`.

- Normalize remaps each feature dimension from observed min/max to a target range (typically `[0, 1]`).
- Scaling is performed per column, preserving within-column ordering across points.
- Particularly useful before distance-based ML methods where feature ranges differ widely.
