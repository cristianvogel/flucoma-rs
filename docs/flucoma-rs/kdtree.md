# KDTree

Module path: `flucoma_rs::search::KDTree`

## Types

- `KDTree`
- `KNNResult { distances: Vec<f64>, ids: Vec<String> }`

## API

```rust
pub fn new(dims: usize) -> KDTree;
pub fn add(&mut self, id: &str, data: &[f64]);
pub fn k_nearest(&self, input: &[f64], k: usize) -> KNNResult;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/kdtree/+page.svx`.

- KDTree is presented as a nearest-neighbour index over dataset points.
- Distance-based lookup depends heavily on preprocessing/scaling consistency.
- Typical workflow: fit/index descriptors first, then query in the same feature space.
