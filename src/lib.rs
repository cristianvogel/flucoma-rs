//! Safe Rust bindings for [flucoma-core](https://github.com/flucoma/flucoma-core)
//! audio analysis algorithms.

mod audio_transport;
mod bufstats;
mod envelope_seg;
mod kdtree;
mod loudness;
mod mel_bands;
mod multi_stats;
mod novelty_seg;
mod onset;
mod onset_seg;
mod running_stats;
mod stft;
mod transient_seg;

pub mod analyzation {
    pub use super::loudness::Loudness;
    pub use super::mel_bands::MelBands;
    pub use super::onset::{OnsetDetectionFunctions, OnsetFunction};
    pub use super::stft::{ComplexSpectrum, Istft, Stft, WindowType};
}

pub mod decomposition {
    pub use super::audio_transport::AudioTransport;
}

pub mod segmentation {
    pub use super::envelope_seg::EnvelopeSegmentation;
    pub use super::novelty_seg::NoveltySegmentation;
    pub use super::onset_seg::OnsetSegmentation;
    pub use super::transient_seg::TransientSegmentation;
}

pub mod search {
    pub use super::kdtree::KDTree;
}

pub mod data {
    pub use super::bufstats::{BufStat, BufStats, BufStatsConfig, BufStatsOutput, BufStatsSelect};
    pub use super::multi_stats::{MultiStats, MultiStatsConfig, MultiStatsOutput};
    pub use super::running_stats::RunningStats;
}
