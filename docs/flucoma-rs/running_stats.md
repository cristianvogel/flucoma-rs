# RunningStats

Module path: `flucoma_rs::data::RunningStats`

## Type

- `RunningStats`

## API

```rust
pub fn new(history_size: usize, input_size: usize) -> Result<RunningStats, &'static str>;
pub fn process<'a>(&'a mut self, input: &[f64]) -> (&'a [f64], &'a [f64]);
pub fn clear(&mut self);
pub fn history_size(&self) -> usize;
pub fn input_size(&self) -> usize;
```

## Notes

- `process` returns `(mean, sample_stddev)`.
- Returned slices borrow internal buffers valid until next `process` call.

## FluCoMa Reference Notes (Archival)

Source basis: `learn-website/src/routes/(content)/reference/stats/+page.svx`.

- Stats computes rolling mean and standard deviation over a bounded history window.
- Designed for streaming values (including multi-dimensional streams), unlike static-buffer summaries.
- Useful for smoothing/monitoring short-term behaviour in descriptor streams.
