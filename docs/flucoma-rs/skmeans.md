# SKMeans

Module path: `flucoma_rs::data::{SKMeans, KMeansConfig, KMeansInit, KMeansResult}`

## Types

- `KMeansInit`
  - `RandomPartition`
  - `RandomPoint`
  - `RandomSampling`
- `KMeansConfig { k: usize, max_iter: usize, init: KMeansInit, seed: isize }`
- `KMeansResult { means: Vec<f64>, assignments: Vec<usize>, k: usize, dims: usize }`
- `SKMeans`

## API

```rust
pub fn new() -> Result<SKMeans, &'static str>;

pub fn fit(
    &mut self,
    data: &[f64],
    rows: usize,
    dims: usize,
    config: KMeansConfig,
) -> Result<KMeansResult, &'static str>;

pub fn encode(
    &self,
    data: &[f64],
    rows: usize,
    dims: usize,
    alpha: f64,
) -> Result<Vec<f64>, &'static str>;
```

## Notes

- Input matrix layout is row-major.
- `fit` must be called before `encode`.
- `encode` returns a row-major matrix with shape `rows x k`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/skmeans/+page.svx`.

- SKMeans performs fixed-`k` clustering in a spherical/cosine-oriented feature space.
- It is commonly used for dictionary-style representations before sparse encoding.
- Consistent normalization/preprocessing improves cluster geometry and encoding behavior.
