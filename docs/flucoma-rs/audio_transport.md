# AudioTransport

Module path: `flucoma_rs::decomposition::AudioTransport`

## Type

- `AudioTransport`

## API

```rust
pub fn new(window_size: usize, fft_size: usize, hop_size: usize)
    -> Result<AudioTransport, &'static str>;

pub fn process_frame<'a>(
    &'a mut self,
    in1: &[f64],
    in2: &[f64],
    weight: f64,
) -> (&'a [f64], &'a [f64]);

pub fn window_size(&self) -> usize;
pub fn fft_size(&self) -> usize;
pub fn hop_size(&self) -> usize;
```

## Notes

- `weight` is clamped to `[0.0, 1.0]`.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/audiotransport/+page.svx`.

- AudioTransport is framed as spectral-domain morphing/cross-transport between sources.
- It depends on STFT-domain representations and inherits practical STFT parameter tradeoffs.
- Musical outcome depends strongly on analysis resolution and interpolation weighting.
