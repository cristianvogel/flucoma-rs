# TransientSegmentation

Module path: `flucoma_rs::segmentation::TransientSegmentation`

## Type

- `TransientSegmentation`

## API

```rust
pub fn new(order: usize, block_size: usize, pad_size: usize)
    -> Result<TransientSegmentation, &'static str>;

pub fn set_detection_parameters(
    &mut self,
    power: f64,
    thresh_hi: f64,
    thresh_lo: f64,
    half_window: usize,
    hold: usize,
    min_segment: usize,
);

pub fn process(&mut self, input: &[f64]) -> Vec<f64>;
pub fn hop_size(&self) -> usize;
pub fn input_size(&self) -> usize;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/transientslice/+page.svx`.

- Transient slicing focuses on abrupt signal changes characteristic of transients.
- Parameterization controls sensitivity and minimum spacing of detections.
- It is most useful when the target material has clear attack structures.
