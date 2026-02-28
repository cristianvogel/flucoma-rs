# FluCoMa Wrapper API Docs

This folder documents the public Rust wrappers exposed by `flucoma-rs`.

All paths and links are relative, so this renders correctly on crates.io and GitHub.

## Source Policy (Archival Notes)

The extra “FluCoMa Reference Notes (Archival)” sections in this folder are derived **only** from:

- `https://github.com/flucoma/learn-website`
- `src/routes/(content)/reference/**/+page.svx`

Snapshot used while writing these notes: `learn-website` commit `46b2742`.

## Acknowledgments

FluCoMa reference content and conceptual explanations are by the FluCoMa project contributors.
These archival summaries are included here for educational continuity and posterity in case upstream web pages move or disappear.

## analyzation

- [Loudness](./loudness.md)
- [MelBands](./mel_bands.md)
- [OnsetDetectionFunctions](./onset_detection.md)
- [Stft / Istft](./stft_istft.md)

## decomposition

- [AudioTransport](./audio_transport.md)

## segmentation

- [EnvelopeSegmentation](./envelope_segmentation.md)
- [NoveltySegmentation](./novelty_segmentation.md)
- [OnsetSegmentation](./onset_segmentation.md)
- [TransientSegmentation](./transient_segmentation.md)

## search

- [KDTree](./kdtree.md)

## data

- [BufStats](./bufstats.md)
- [MultiStats](./multistats.md)
- [Normalize](./normalize.md)
- [Standardize](./standardize.md)
- [RobustScale](./robust_scale.md)
- [RunningStats](./running_stats.md)
- [Pca](./pca.md)

## Notes

- Matrix inputs in `data` wrappers are row-major unless stated otherwise.
- Buffer-style wrappers (e.g. `BufStats`) document channel-major layout explicitly.
