#![allow(non_upper_case_globals)]
#![recursion_limit = "512"]
/// Raw bindings for flucoma-core algorithms via inline C++ (cpp! macros).
///
/// All handles are opaque `*mut u8` pointers. Do not use these functions
/// directly -- use the safe wrappers in the `flucoma-rs` crate instead.
use cpp::cpp;

/// Signed index type matching `ptrdiff_t` used by flucoma-core.
pub type FlucomaIndex = isize;

// -------------------------------------------------------------------------------------------------
// Cpp includes

cpp! {{
    #define FMT_HEADER_ONLY 1
    #include <complex>
    #include <flucoma/algorithms/public/Loudness.hpp>
    #include <flucoma/algorithms/public/STFT.hpp>
    #include <flucoma/algorithms/public/MelBands.hpp>
    #include <flucoma/algorithms/public/OnsetDetectionFunctions.hpp>
    #include <flucoma/algorithms/public/OnsetSegmentation.hpp>
    #include <flucoma/algorithms/public/EnvelopeSegmentation.hpp>
    #include <flucoma/algorithms/public/NoveltySegmentation.hpp>
    #include <flucoma/algorithms/public/TransientSegmentation.hpp>
    #include <flucoma/algorithms/public/AudioTransport.hpp>
    #include <flucoma/algorithms/public/KDTree.hpp>
    #include <flucoma/algorithms/public/MultiStats.hpp>
    #include <flucoma/algorithms/public/Normalization.hpp>
    #include <flucoma/algorithms/public/PCA.hpp>
    #include <flucoma/algorithms/public/RobustScaling.hpp>
    #include <flucoma/algorithms/public/Standardization.hpp>
    #include <flucoma/algorithms/public/RunningStats.hpp>
    using namespace fluid;
    using namespace fluid::algorithm;
}}

// -------------------------------------------------------------------------------------------------
// Loudness

pub fn loudness_create(max_size: FlucomaIndex) -> *mut u8 {
    unsafe {
        cpp!([max_size as "ptrdiff_t"] -> *mut u8 as "void*" {
            return static_cast<void*>(new Loudness(max_size));
        })
    }
}

pub fn loudness_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "Loudness*"] {
            delete ptr;
        })
    }
}

pub fn loudness_init(ptr: *mut u8, size: FlucomaIndex, sample_rate: f64) {
    unsafe {
        cpp!([ptr as "Loudness*", size as "ptrdiff_t", sample_rate as "double"] {
            ptr->init(size, sample_rate);
        })
    }
}

/// Fills `output[0] = loudness_dB`, `output[1] = peak_dB`.
/// `output` must point to at least 2 f64 values.
pub fn loudness_process_frame(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    output: *mut f64,
    weighting: bool,
    true_peak: bool,
) {
    unsafe {
        cpp!([
            ptr as "Loudness*",
            input as "const double*", input_len as "ptrdiff_t",
            output as "double*",
            weighting as "bool", true_peak as "bool"
        ] {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            FluidTensorView<double, 1> out_v(output, 0, 2);
            ptr->processFrame(in_v, out_v, weighting, true_peak);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// STFT

pub fn stft_create(
    window_size: FlucomaIndex,
    fft_size: FlucomaIndex,
    hop_size: FlucomaIndex,
    window_type: FlucomaIndex,
) -> *mut u8 {
    unsafe {
        cpp!([
            window_size as "ptrdiff_t", fft_size as "ptrdiff_t",
            hop_size as "ptrdiff_t", window_type as "ptrdiff_t"
        ] -> *mut u8 as "void*" {
            return static_cast<void*>(new STFT(window_size, fft_size, hop_size, window_type));
        })
    }
}

pub fn stft_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "STFT*"] {
            delete ptr;
        })
    }
}

/// `out_complex`: interleaved [re0, im0, re1, im1, ...], len = 2 * num_bins.
pub fn stft_process_frame(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    out_complex: *mut f64,
    num_bins: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "STFT*",
            input as "const double*", input_len as "ptrdiff_t",
            out_complex as "double*", num_bins as "ptrdiff_t"
        ] {
            auto* cptr = reinterpret_cast<std::complex<double>*>(out_complex);
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            FluidTensorView<std::complex<double>, 1> out_v(cptr, 0, num_bins);
            ptr->processFrame(in_v, out_v);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// ISTFT

pub fn istft_create(
    window_size: FlucomaIndex,
    fft_size: FlucomaIndex,
    hop_size: FlucomaIndex,
    window_type: FlucomaIndex,
) -> *mut u8 {
    unsafe {
        cpp!([
            window_size as "ptrdiff_t", fft_size as "ptrdiff_t",
            hop_size as "ptrdiff_t", window_type as "ptrdiff_t"
        ] -> *mut u8 as "void*" {
            return static_cast<void*>(new ISTFT(window_size, fft_size, hop_size, window_type));
        })
    }
}

pub fn istft_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "ISTFT*"] {
            delete ptr;
        })
    }
}

/// `in_complex`: interleaved [re0, im0, re1, im1, ...], len = 2 * num_bins.
pub fn istft_process_frame(
    ptr: *mut u8,
    in_complex: *const f64,
    num_bins: FlucomaIndex,
    output: *mut f64,
    output_len: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "ISTFT*",
            in_complex as "const double*", num_bins as "ptrdiff_t",
            output as "double*", output_len as "ptrdiff_t"
        ] {
            auto* cptr = reinterpret_cast<std::complex<double>*>(
                const_cast<double*>(in_complex));
            FluidTensorView<std::complex<double>, 1> in_v(cptr, 0, num_bins);
            FluidTensorView<double, 1> out_v(output, 0, output_len);
            ptr->processFrame(in_v, out_v);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// MelBands

pub fn melbands_create(max_bands: FlucomaIndex, max_fft: FlucomaIndex) -> *mut u8 {
    unsafe {
        cpp!([max_bands as "ptrdiff_t", max_fft as "ptrdiff_t"] -> *mut u8 as "void*" {
            return static_cast<void*>(new MelBands(max_bands, max_fft));
        })
    }
}

pub fn melbands_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "MelBands*"] {
            delete ptr;
        })
    }
}

pub fn melbands_init(
    ptr: *mut u8,
    lo_hz: f64,
    hi_hz: f64,
    n_bands: FlucomaIndex,
    n_bins: FlucomaIndex,
    sample_rate: f64,
    window_size: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "MelBands*",
            lo_hz as "double", hi_hz as "double",
            n_bands as "ptrdiff_t", n_bins as "ptrdiff_t",
            sample_rate as "double", window_size as "ptrdiff_t"
        ] {
            ptr->init(lo_hz, hi_hz, n_bands, n_bins, sample_rate, window_size);
        })
    }
}

pub fn melbands_process_frame(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    output: *mut f64,
    output_len: FlucomaIndex,
    mag_norm: bool,
    use_power: bool,
    log_output: bool,
) {
    unsafe {
        cpp!([
            ptr as "MelBands*",
            input as "const double*", input_len as "ptrdiff_t",
            output as "double*", output_len as "ptrdiff_t",
            mag_norm as "bool", use_power as "bool", log_output as "bool"
        ] {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            FluidTensorView<double, 1> out_v(output, 0, output_len);
            Allocator alloc{};
            ptr->processFrame(in_v, out_v, mag_norm, use_power, log_output, alloc);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// OnsetDetectionFunctions

pub fn onset_create(max_size: FlucomaIndex, max_filter_size: FlucomaIndex) -> *mut u8 {
    unsafe {
        cpp!([max_size as "ptrdiff_t", max_filter_size as "ptrdiff_t"] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(
                new OnsetDetectionFunctions(max_size, max_filter_size, alloc));
        })
    }
}

pub fn onset_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "OnsetDetectionFunctions*"] {
            delete ptr;
        })
    }
}

pub fn onset_init(
    ptr: *mut u8,
    window_size: FlucomaIndex,
    fft_size: FlucomaIndex,
    filter_size: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "OnsetDetectionFunctions*",
            window_size as "ptrdiff_t", fft_size as "ptrdiff_t", filter_size as "ptrdiff_t"
        ] {
            ptr->init(window_size, fft_size, filter_size);
        })
    }
}

/// Returns the onset detection value for this frame.
pub fn onset_process_frame(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    function: FlucomaIndex,
    filter_size: FlucomaIndex,
    frame_delta: FlucomaIndex,
) -> f64 {
    unsafe {
        cpp!([
            ptr as "OnsetDetectionFunctions*",
            input as "const double*", input_len as "ptrdiff_t",
            function as "ptrdiff_t", filter_size as "ptrdiff_t", frame_delta as "ptrdiff_t"
        ] -> f64 as "double" {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            Allocator alloc{};
            return ptr->processFrame(in_v, function, filter_size, frame_delta, alloc);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// OnsetSegmentation

pub fn onset_seg_create(max_size: FlucomaIndex, max_filter_size: FlucomaIndex) -> *mut u8 {
    unsafe {
        cpp!([max_size as "ptrdiff_t", max_filter_size as "ptrdiff_t"] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(
                new OnsetSegmentation(max_size, max_filter_size, alloc));
        })
    }
}

pub fn onset_seg_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "OnsetSegmentation*"] {
            delete ptr;
        })
    }
}

pub fn onset_seg_init(
    ptr: *mut u8,
    window_size: FlucomaIndex,
    fft_size: FlucomaIndex,
    filter_size: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "OnsetSegmentation*",
            window_size as "ptrdiff_t", fft_size as "ptrdiff_t", filter_size as "ptrdiff_t"
        ] {
            ptr->init(window_size, fft_size, filter_size);
        })
    }
}

/// Returns 1.0 if an onset is detected, 0.0 otherwise.
pub fn onset_seg_process_frame(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    function: FlucomaIndex,
    filter_size: FlucomaIndex,
    threshold: f64,
    debounce: FlucomaIndex,
    frame_delta: FlucomaIndex,
) -> f64 {
    unsafe {
        cpp!([
            ptr as "OnsetSegmentation*",
            input as "const double*", input_len as "ptrdiff_t",
            function as "ptrdiff_t", filter_size as "ptrdiff_t",
            threshold as "double", debounce as "ptrdiff_t", frame_delta as "ptrdiff_t"
        ] -> f64 as "double" {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            Allocator alloc{};
            return ptr->processFrame(in_v, function, filter_size, threshold, debounce, frame_delta, alloc);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// EnvelopeSegmentation

pub fn env_seg_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new EnvelopeSegmentation());
        })
    }
}

pub fn env_seg_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "EnvelopeSegmentation*"] {
            delete ptr;
        })
    }
}

pub fn env_seg_init(ptr: *mut u8, floor: f64, hi_pass_freq: f64) {
    unsafe {
        cpp!([
            ptr as "EnvelopeSegmentation*",
            floor as "double", hi_pass_freq as "double"
        ] {
            ptr->init(floor, hi_pass_freq);
        })
    }
}

/// Process a single audio sample. Returns 1.0 on onset, 0.0 otherwise.
pub fn env_seg_process_sample(
    ptr: *mut u8,
    sample: f64,
    on_threshold: f64,
    off_threshold: f64,
    floor: f64,
    fast_ramp_up: FlucomaIndex,
    slow_ramp_up: FlucomaIndex,
    fast_ramp_down: FlucomaIndex,
    slow_ramp_down: FlucomaIndex,
    hi_pass_freq: f64,
    debounce: FlucomaIndex,
) -> f64 {
    unsafe {
        cpp!([
            ptr as "EnvelopeSegmentation*",
            sample as "double",
            on_threshold as "double", off_threshold as "double",
            floor as "double",
            fast_ramp_up as "ptrdiff_t", slow_ramp_up as "ptrdiff_t",
            fast_ramp_down as "ptrdiff_t", slow_ramp_down as "ptrdiff_t",
            hi_pass_freq as "double", debounce as "ptrdiff_t"
        ] -> f64 as "double" {
            return ptr->processSample(sample, on_threshold, off_threshold, floor,
                fast_ramp_up, slow_ramp_up, fast_ramp_down, slow_ramp_down,
                hi_pass_freq, debounce);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// NoveltySegmentation

pub fn novelty_seg_create(
    max_kernel_size: FlucomaIndex,
    max_dims: FlucomaIndex,
    max_filter_size: FlucomaIndex,
) -> *mut u8 {
    unsafe {
        cpp!([
            max_kernel_size as "ptrdiff_t", max_dims as "ptrdiff_t", max_filter_size as "ptrdiff_t"
        ] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(
                new NoveltySegmentation(max_kernel_size, max_dims, max_filter_size, alloc));
        })
    }
}

pub fn novelty_seg_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "NoveltySegmentation*"] {
            delete ptr;
        })
    }
}

pub fn novelty_seg_init(
    ptr: *mut u8,
    kernel_size: FlucomaIndex,
    filter_size: FlucomaIndex,
    n_dims: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "NoveltySegmentation*",
            kernel_size as "ptrdiff_t", filter_size as "ptrdiff_t", n_dims as "ptrdiff_t"
        ] {
            Allocator alloc{};
            ptr->init(kernel_size, filter_size, n_dims, alloc);
        })
    }
}

/// Returns 1.0 at novelty slice points, 0.0 otherwise.
pub fn novelty_seg_process_frame(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    threshold: f64,
    min_slice_length: FlucomaIndex,
) -> f64 {
    unsafe {
        cpp!([
            ptr as "NoveltySegmentation*",
            input as "const double*", input_len as "ptrdiff_t",
            threshold as "double", min_slice_length as "ptrdiff_t"
        ] -> f64 as "double" {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            Allocator alloc{};
            return ptr->processFrame(in_v, threshold, min_slice_length, alloc);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// TransientSegmentation

pub fn transient_seg_create(
    max_order: FlucomaIndex,
    max_block_size: FlucomaIndex,
    max_pad_size: FlucomaIndex,
) -> *mut u8 {
    unsafe {
        cpp!([
            max_order as "ptrdiff_t", max_block_size as "ptrdiff_t", max_pad_size as "ptrdiff_t"
        ] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(
                new TransientSegmentation(max_order, max_block_size, max_pad_size, alloc));
        })
    }
}

pub fn transient_seg_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "TransientSegmentation*"] {
            delete ptr;
        })
    }
}

pub fn transient_seg_init(
    ptr: *mut u8,
    order: FlucomaIndex,
    block_size: FlucomaIndex,
    pad_size: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "TransientSegmentation*",
            order as "ptrdiff_t", block_size as "ptrdiff_t", pad_size as "ptrdiff_t"
        ] {
            ptr->init(order, block_size, pad_size);
        })
    }
}

pub fn transient_seg_set_detection_params(
    ptr: *mut u8,
    power: f64,
    thresh_hi: f64,
    thresh_lo: f64,
    half_window: FlucomaIndex,
    hold: FlucomaIndex,
    min_segment: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "TransientSegmentation*",
            power as "double", thresh_hi as "double", thresh_lo as "double",
            half_window as "ptrdiff_t", hold as "ptrdiff_t", min_segment as "ptrdiff_t"
        ] {
            ptr->setDetectionParameters(power, thresh_hi, thresh_lo, half_window, hold, min_segment);
        })
    }
}

/// Input length must be `input_size` (hop + pad). Output length must be `hop_size`.
/// Each output sample is 1.0 (transient onset) or 0.0.
pub fn transient_seg_process(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    output: *mut f64,
    output_len: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "TransientSegmentation*",
            input as "const double*", input_len as "ptrdiff_t",
            output as "double*", output_len as "ptrdiff_t"
        ] {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            FluidTensorView<double, 1> out_v(output, 0, output_len);
            Allocator alloc{};
            ptr->process(in_v, out_v, alloc);
        })
    }
}

pub fn transient_seg_hop_size(ptr: *mut u8) -> FlucomaIndex {
    unsafe {
        cpp!([ptr as "TransientSegmentation*"] -> FlucomaIndex as "ptrdiff_t" {
            return ptr->hopSize();
        })
    }
}

pub fn transient_seg_input_size(ptr: *mut u8) -> FlucomaIndex {
    unsafe {
        cpp!([ptr as "TransientSegmentation*"] -> FlucomaIndex as "ptrdiff_t" {
            return ptr->inputSize();
        })
    }
}

// -------------------------------------------------------------------------------------------------
// AudioTransport

pub fn audio_transport_create(max_fft_size: FlucomaIndex) -> *mut u8 {
    unsafe {
        cpp!([max_fft_size as "ptrdiff_t"] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(new AudioTransport(max_fft_size, alloc));
        })
    }
}

pub fn audio_transport_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "AudioTransport*"] {
            delete ptr;
        })
    }
}

pub fn audio_transport_init(
    ptr: *mut u8,
    window_size: FlucomaIndex,
    fft_size: FlucomaIndex,
    hop_size: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "AudioTransport*",
            window_size as "ptrdiff_t", fft_size as "ptrdiff_t", hop_size as "ptrdiff_t"
        ] {
            ptr->init(window_size, fft_size, hop_size);
        })
    }
}

/// Fills `output` (length = 2 * frame_len): first half is interpolated audio,
/// second half is the squared window for overlap-add normalization.
pub fn audio_transport_process_frame(
    ptr: *mut u8,
    in1: *const f64,
    in2: *const f64,
    frame_len: FlucomaIndex,
    weight: f64,
    output: *mut f64,
) {
    unsafe {
        cpp!([
            ptr as "AudioTransport*",
            in1 as "const double*", in2 as "const double*",
            frame_len as "ptrdiff_t",
            weight as "double",
            output as "double*"
        ] {
            FluidTensorView<double, 1> in1_v(const_cast<double*>(in1), 0, frame_len);
            FluidTensorView<double, 1> in2_v(const_cast<double*>(in2), 0, frame_len);
            FluidTensorView<double, 2> out_v(output, 0, 2, frame_len);
            Allocator alloc{};
            ptr->processFrame(in1_v, in2_v, weight, out_v, alloc);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// KDTree

pub fn kdtree_create(dims: FlucomaIndex) -> *mut u8 {
    unsafe {
        cpp!([dims as "ptrdiff_t"] -> *mut u8 as "void*" {
            // Construct from an empty DataSet so KDTree dimensions are initialized.
            KDTree::DataSet dataSet(dims);
            return static_cast<void*>(new KDTree(dataSet));
        })
    }
}

pub fn kdtree_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "KDTree*"] {
            delete ptr;
        })
    }
}

pub fn kdtree_add_node(ptr: *mut u8, id: *const u8, data: *const f64, len: FlucomaIndex) {
    unsafe {
        cpp!([ptr as "KDTree*", id as "const char*", data as "const double*", len as "ptrdiff_t"] {
            FluidTensorView<double, 1> data_v(const_cast<double*>(data), 0, len);
            // Rebuild from DataSet because KDTree::addNode currently uses
            // invalid shared_ptr ownership internally.
            auto flat = ptr->toFlat();
            KDTree::DataSet dataSet(flat.ids, flat.data);
            dataSet.add(std::string(id), data_v);
            *ptr = KDTree(dataSet);
        })
    }
}

/// Simple kNearest binding.
/// Returns distances and IDs.
/// To return IDs, we'll pass a buffer of pointers to C strings.
pub fn kdtree_k_nearest(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    k: FlucomaIndex,
    radius: f64,
    out_distances: *mut f64,
    out_ids: *mut *const u8,
) {
    unsafe {
        cpp!([
            ptr as "KDTree*",
            input as "const double*", input_len as "ptrdiff_t",
            k as "ptrdiff_t", radius as "double",
            out_distances as "double*",
            out_ids as "const char**"
        ] {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            Allocator alloc{};
            auto result = ptr->kNearest(in_v, k, radius, alloc);
            for(fluid::index i = 0; i < static_cast<fluid::index>(result.first.size()); ++i) {
                out_distances[i] = result.first[i];
                out_ids[i] = result.second[i]->c_str();
            }
        })
    }
}

// -------------------------------------------------------------------------------------------------
// MultiStats

pub fn multistats_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new MultiStats());
        })
    }
}

pub fn multistats_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "MultiStats*"] {
            delete ptr;
        })
    }
}

pub fn multistats_init(
    ptr: *mut u8,
    num_derivatives: FlucomaIndex,
    low_percentile: f64,
    middle_percentile: f64,
    high_percentile: f64,
) {
    unsafe {
        cpp!([
            ptr as "MultiStats*",
            num_derivatives as "ptrdiff_t",
            low_percentile as "double",
            middle_percentile as "double",
            high_percentile as "double"
        ] {
            ptr->init(num_derivatives, low_percentile, middle_percentile, high_percentile);
        })
    }
}

pub fn multistats_process(
    ptr: *mut u8,
    input: *const f64,
    num_channels: FlucomaIndex,
    num_frames: FlucomaIndex,
    output: *mut f64,
    output_cols: FlucomaIndex,
    outliers_cutoff: f64,
    weights: *const f64,
    weights_len: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "MultiStats*",
            input as "const double*",
            num_channels as "ptrdiff_t",
            num_frames as "ptrdiff_t",
            output as "double*",
            output_cols as "ptrdiff_t",
            outliers_cutoff as "double",
            weights as "const double*",
            weights_len as "ptrdiff_t"
        ] {
            FluidTensorView<double, 2> in_v(
                const_cast<double*>(input),
                0,
                num_channels,
                num_frames
            );
            FluidTensorView<double, 2> out_v(output, 0, num_channels, output_cols);
            if (weights_len > 0 && weights != nullptr) {
                RealVectorView weight_v(const_cast<double*>(weights), 0, weights_len);
                ptr->process(in_v, out_v, outliers_cutoff, weight_v);
            } else {
                RealVectorView no_weights(nullptr, 0, 0);
                ptr->process(in_v, out_v, outliers_cutoff, no_weights);
            }
        })
    }
}

// -------------------------------------------------------------------------------------------------
// RunningStats

pub fn running_stats_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new RunningStats());
        })
    }
}

pub fn running_stats_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "RunningStats*"] {
            delete ptr;
        })
    }
}

pub fn running_stats_init(ptr: *mut u8, history_size: FlucomaIndex, input_size: FlucomaIndex) {
    unsafe {
        cpp!([
            ptr as "RunningStats*",
            history_size as "ptrdiff_t",
            input_size as "ptrdiff_t"
        ] {
            ptr->init(history_size, input_size);
        })
    }
}

pub fn running_stats_process(
    ptr: *mut u8,
    input: *const f64,
    input_len: FlucomaIndex,
    mean_out: *mut f64,
    stddev_out: *mut f64,
) {
    unsafe {
        cpp!([
            ptr as "RunningStats*",
            input as "const double*",
            input_len as "ptrdiff_t",
            mean_out as "double*",
            stddev_out as "double*"
        ] {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            FluidTensorView<double, 1> mean_v(mean_out, 0, input_len);
            FluidTensorView<double, 1> std_v(stddev_out, 0, input_len);
            ptr->process(in_v, mean_v, std_v);
        })
    }
}

// -------------------------------------------------------------------------------------------------
// Normalization

pub fn normalization_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new Normalization());
        })
    }
}

pub fn normalization_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "Normalization*"] {
            delete ptr;
        })
    }
}

pub fn normalization_fit(
    ptr: *mut u8,
    min: f64,
    max: f64,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "Normalization*",
            min as "double", max as "double",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            ptr->init(min, max, in_v);
        })
    }
}

pub fn normalization_process(
    ptr: *mut u8,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
    output: *mut f64,
    inverse: bool,
) {
    unsafe {
        cpp!([
            ptr as "Normalization*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t",
            output as "double*",
            inverse as "bool"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            FluidTensorView<double, 2> out_v(output, 0, rows, cols);
            ptr->process(in_v, out_v, inverse);
        })
    }
}

pub fn normalization_initialized(ptr: *mut u8) -> bool {
    unsafe {
        cpp!([ptr as "Normalization*"] -> bool as "bool" {
            return ptr->initialized();
        })
    }
}

// -------------------------------------------------------------------------------------------------
// Standardization

pub fn standardization_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new Standardization());
        })
    }
}

pub fn standardization_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "Standardization*"] {
            delete ptr;
        })
    }
}

pub fn standardization_fit(
    ptr: *mut u8,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "Standardization*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            ptr->init(in_v);
        })
    }
}

pub fn standardization_process(
    ptr: *mut u8,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
    output: *mut f64,
    inverse: bool,
) {
    unsafe {
        cpp!([
            ptr as "Standardization*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t",
            output as "double*",
            inverse as "bool"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            FluidTensorView<double, 2> out_v(output, 0, rows, cols);
            ptr->process(in_v, out_v, inverse);
        })
    }
}

pub fn standardization_initialized(ptr: *mut u8) -> bool {
    unsafe {
        cpp!([ptr as "Standardization*"] -> bool as "bool" {
            return ptr->initialized();
        })
    }
}

// -------------------------------------------------------------------------------------------------
// RobustScaling

pub fn robust_scaling_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new RobustScaling());
        })
    }
}

pub fn robust_scaling_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "RobustScaling*"] {
            delete ptr;
        })
    }
}

pub fn robust_scaling_fit(
    ptr: *mut u8,
    low: f64,
    high: f64,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
) {
    unsafe {
        cpp!([
            ptr as "RobustScaling*",
            low as "double", high as "double",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            ptr->init(low, high, in_v);
        })
    }
}

pub fn robust_scaling_process(
    ptr: *mut u8,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
    output: *mut f64,
    inverse: bool,
) {
    unsafe {
        cpp!([
            ptr as "RobustScaling*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t",
            output as "double*",
            inverse as "bool"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            FluidTensorView<double, 2> out_v(output, 0, rows, cols);
            ptr->process(in_v, out_v, inverse);
        })
    }
}

pub fn robust_scaling_initialized(ptr: *mut u8) -> bool {
    unsafe {
        cpp!([ptr as "RobustScaling*"] -> bool as "bool" {
            return ptr->initialized();
        })
    }
}

// -------------------------------------------------------------------------------------------------
// PCA

pub fn pca_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new PCA());
        })
    }
}

pub fn pca_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "PCA*"] {
            delete ptr;
        })
    }
}

pub fn pca_fit(ptr: *mut u8, input: *const f64, rows: FlucomaIndex, cols: FlucomaIndex) {
    unsafe {
        cpp!([
            ptr as "PCA*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            ptr->init(in_v);
        })
    }
}

pub fn pca_transform(
    ptr: *mut u8,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
    output: *mut f64,
    k: FlucomaIndex,
    whiten: bool,
) -> f64 {
    unsafe {
        cpp!([
            ptr as "PCA*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t",
            output as "double*",
            k as "ptrdiff_t",
            whiten as "bool"
        ] -> f64 as "double" {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            FluidTensorView<double, 2> out_v(output, 0, rows, k);
            return ptr->process(in_v, out_v, k, whiten);
        })
    }
}

pub fn pca_inverse_transform(
    ptr: *mut u8,
    input: *const f64,
    rows: FlucomaIndex,
    cols: FlucomaIndex,
    output: *mut f64,
    out_cols: FlucomaIndex,
    whiten: bool,
) {
    unsafe {
        cpp!([
            ptr as "PCA*",
            input as "const double*",
            rows as "ptrdiff_t", cols as "ptrdiff_t",
            output as "double*",
            out_cols as "ptrdiff_t",
            whiten as "bool"
        ] {
            FluidTensorView<double, 2> in_v(const_cast<double*>(input), 0, rows, cols);
            FluidTensorView<double, 2> out_v(output, 0, rows, out_cols);
            ptr->inverseProcess(in_v, out_v, whiten);
        })
    }
}

pub fn pca_initialized(ptr: *mut u8) -> bool {
    unsafe {
        cpp!([ptr as "PCA*"] -> bool as "bool" {
            return ptr->initialized();
        })
    }
}

pub fn pca_dims(ptr: *mut u8) -> FlucomaIndex {
    unsafe {
        cpp!([ptr as "PCA*"] -> FlucomaIndex as "ptrdiff_t" {
            return ptr->dims();
        })
    }
}
