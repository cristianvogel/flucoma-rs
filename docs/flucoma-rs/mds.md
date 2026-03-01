# Mds

Module path: `flucoma_rs::data::{Mds, MdsDistance}`

## Types

- `MdsDistance`
  - `Manhattan`
  - `Euclidean`
  - `SquaredEuclidean`
  - `Max`
  - `Min`
  - `KullbackLeibler`
  - `Cosine`
  - `JensenShannon`
- `Mds`

## API

```rust
pub fn new() -> Result<Mds, &'static str>;

pub fn project(
    &mut self,
    data: &[f64],
    rows: usize,
    cols: usize,
    target_dims: usize,
    distance: MdsDistance,
) -> Result<Vec<f64>, &'static str>;
```

## Notes

- Input matrix layout is row-major.
- Output matrix layout is row-major with shape `rows x target_dims`.
- `target_dims` must be in `1..=rows`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/mds/+page.svx`.

- MDS projects points while preserving pairwise distance structure as closely as possible.
- Distance metric choice materially changes neighborhood geometry and final embedding.
- MDS is often used for exploratory visualization and structure-aware redistribution.
