# KMeans

Module path: `flucoma_rs::data::{KMeans, KMeansConfig, KMeansInit, KMeansResult}`

## Types

- `KMeansInit`
  - `RandomPartition`
  - `RandomPoint`
  - `RandomSampling`
- `KMeansConfig { k: usize, max_iter: usize, init: KMeansInit, seed: isize }`
- `KMeansResult { means: Vec<f64>, assignments: Vec<usize>, k: usize, dims: usize }`
- `KMeans`

## API

```rust
pub fn new() -> Result<KMeans, &'static str>;

pub fn fit(
    &mut self,
    data: &[f64],
    rows: usize,
    dims: usize,
    config: KMeansConfig,
) -> Result<KMeansResult, &'static str>;
```

## Notes

- Input matrix layout is row-major.
- `means` is row-major with shape `k x dims`.
- `assignments.len() == rows` and each assignment is in `[0, k)`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/kmeans/+page.svx`.

- KMeans partitions points into a fixed number of clusters by minimizing within-cluster distance.
- Initialization strategy and iteration count affect stability and runtime.
- Feature scaling is usually important before clustering so dimensions contribute comparably.
