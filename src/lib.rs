//! Safe Rust bindings for [flucoma-core](https://github.com/flucoma/flucoma-core)
//! audio analysis algorithms.

mod audio_transport;
mod bufstats;
mod dataset_query;
mod envelope_seg;
mod grid;
mod kdtree;
mod kmeans;
mod loudness;
mod mel_bands;
mod mds;
mod multi_stats;
mod normalize;
mod novelty_seg;
mod onset;
mod onset_seg;
mod pca;
mod robust_scale;
mod running_stats;
mod standardize;
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
    pub use super::dataset_query::{
        ComparisonOp, DataSetQuery, DataSetQueryResult, QueryCondition,
    };
    pub use super::grid::Grid;
    pub use super::kmeans::{KMeans, KMeansConfig, KMeansInit, KMeansResult, SKMeans};
    pub use super::mds::{Mds, MdsDistance};
    pub use super::multi_stats::{MultiStats, MultiStatsConfig, MultiStatsOutput};
    pub use super::normalize::Normalize;
    pub use super::pca::{Pca, PcaConfig, PcaScaler};
    pub use super::robust_scale::RobustScale;
    pub use super::running_stats::RunningStats;
    pub use super::standardize::Standardize;
}
