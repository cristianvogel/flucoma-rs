# Grid

Module path: `flucoma_rs::data::Grid`

## Types

- `Grid`

## API

```rust
pub fn process(
    input: &[f64],
    rows: usize,
    over_sample: usize,
    extent: usize,
    axis: usize,
) -> Result<Vec<f64>, &'static str>;
```

## Notes

- Input layout is row-major 2D points: `[x0, y0, x1, y1, ...]`.
- Output is also row-major 2D points with length `rows * 2`.
- `axis` is `0` or `1`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/grid/+page.svx`.

- Grid redistributes 2D point clouds into a structured layout useful for visualization and browsing.
- Oversampling and extent parameters trade speed for placement quality.
- Typical usage is after dimensionality reduction (for example PCA/UMAP) when preparing display coordinates.
