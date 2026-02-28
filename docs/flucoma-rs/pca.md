# Pca

Module path: `flucoma_rs::data::{Pca, PcaConfig, PcaScaler}`

## Types

- `PcaScaler`
  - `None`
  - `Normalize { min, max }`
  - `Standardize`
  - `RobustScale { low_percentile, high_percentile }`
- `PcaConfig { whiten: bool, scaler: PcaScaler }`
- `Pca`

## API

```rust
pub fn new(config: PcaConfig) -> Result<Pca, &'static str>;
pub fn config(&self) -> PcaConfig;
pub fn fit(&mut self, data: &[f64], rows: usize, cols: usize) -> Result<(), &'static str>;

pub fn transform(
    &self,
    data: &[f64],
    rows: usize,
    cols: usize,
    target_dims: usize,
) -> Result<(Vec<f64>, f64), &'static str>;

pub fn fit_transform(
    &mut self,
    data: &[f64],
    rows: usize,
    cols: usize,
    target_dims: usize,
) -> Result<(Vec<f64>, f64), &'static str>;

pub fn inverse_transform(
    &self,
    projected: &[f64],
    rows: usize,
    projected_cols: usize,
) -> Result<Vec<f64>, &'static str>;

pub fn is_fitted(&self) -> bool;
pub fn dims(&self) -> Option<usize>;
```

## Notes

- `transform` / `fit_transform` return `(projected_data, explained_variance_ratio)`.
- Matrix layout is row-major.
- Optional scaler is fit during `fit` and reused for `transform` and `inverse_transform`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/pca/+page.svx`.

- PCA rotates data into principal components ordered by explained variance.
- Dimensionality reduction keeps lower-order components and tracks explained variance ratio to quantify retained structure.
- Optional whitening equalizes component variances; this can help some tasks but may also amplify noise.
