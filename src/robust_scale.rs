use flucoma_sys::{
    robust_scaling_create, robust_scaling_destroy, robust_scaling_fit, robust_scaling_initialized,
    robust_scaling_process, FlucomaIndex,
};

use crate::matrix::Matrix;

// -------------------------------------------------------------------------------------------------

/// Percentile-based robust scaler for dataset-style matrices.
///
/// Scales each feature column by its interquartile range (or any custom
/// percentile pair): `(x − median) / (high_percentile − low_percentile)`.
/// Less sensitive to outliers than min-max or z-score scaling.
///
/// Fit once on training data, then call [`transform`](RobustScale::transform)
/// on any same-width matrix, or [`inverse_transform`](RobustScale::inverse_transform)
/// to recover the original scale.
///
/// Input/output layout is row-major: `[row0_col0, row0_col1, …, rowN_colM]`.
///
/// # Usage
/// ```no_run
/// use flucoma_rs::data::{Matrix, RobustScale};
///
/// let data = Matrix::from_vec(vec![1.0, 10.0, 3.0, 20.0, 5.0, 30.0], 3, 2).unwrap();
/// let mut r = RobustScale::new(25.0, 75.0).unwrap();
/// let scaled = r.fit_transform(&data).unwrap();
/// let restored = r.inverse_transform(&scaled).unwrap();
/// ```
///
/// See <https://learn.flucoma.org/reference/robustscale>
pub struct RobustScale {
    inner: *mut u8,
    low_percentile: f64,
    high_percentile: f64,
    cols: Option<usize>,
}

unsafe impl Send for RobustScale {}

impl RobustScale {
    /// Create a robust scaler using the given percentile range per feature.
    ///
    /// # Arguments
    /// * `low_percentile`  - Lower percentile bound, in `[0, 100]`.
    /// * `high_percentile` - Upper percentile bound, in `[0, 100]`. Must be ≥ `low_percentile`.
    ///
    /// # Errors
    /// Returns an error if either percentile is outside `[0, 100]` or if
    /// `low_percentile > high_percentile`.
    pub fn new(low_percentile: f64, high_percentile: f64) -> Result<Self, &'static str> {
        if !(0.0..=100.0).contains(&low_percentile) {
            return Err("low_percentile must be in [0, 100]");
        }
        if !(0.0..=100.0).contains(&high_percentile) {
            return Err("high_percentile must be in [0, 100]");
        }
        if low_percentile > high_percentile {
            return Err("low_percentile must be <= high_percentile");
        }
        let inner = robust_scaling_create();
        if inner.is_null() {
            return Err("failed to create RobustScaling instance");
        }
        Ok(Self {
            inner,
            low_percentile,
            high_percentile,
            cols: None,
        })
    }

    /// Fit the scaler to a row-major matrix.
    ///
    /// Computes per-column medians and percentile-based spread. Calling `fit`
    /// again on new data overwrites the previously learned statistics.
    pub fn fit(&mut self, data: &Matrix) -> Result<(), &'static str> {
        robust_scaling_fit(
            self.inner,
            self.low_percentile,
            self.high_percentile,
            data.data().as_ptr(),
            data.rows() as FlucomaIndex,
            data.cols() as FlucomaIndex,
        );
        self.cols = Some(data.cols());
        Ok(())
    }

    /// Scale a matrix using the fitted percentile statistics.
    ///
    /// # Errors
    /// Returns an error if the scaler has not been fitted yet, or if the
    /// matrix column count differs from the fitted feature dimension.
    pub fn transform(&self, data: &Matrix) -> Result<Matrix, &'static str> {
        self.process_internal(data, false)
    }

    /// Recover the original scale by reversing a previous [`transform`](Self::transform).
    ///
    /// # Errors
    /// Returns an error if the scaler has not been fitted yet, or if the
    /// matrix column count differs from the fitted feature dimension.
    pub fn inverse_transform(&self, data: &Matrix) -> Result<Matrix, &'static str> {
        self.process_internal(data, true)
    }

    /// Fit the scaler and transform the same matrix in one step.
    ///
    /// Equivalent to calling [`fit`](Self::fit) followed by
    /// [`transform`](Self::transform) on the same data.
    ///
    /// # Errors
    /// Propagates errors from [`fit`](Self::fit) or [`transform`](Self::transform).
    pub fn fit_transform(&mut self, data: &Matrix) -> Result<Matrix, &'static str> {
        self.fit(data)?;
        self.transform(data)
    }

    pub fn is_fitted(&self) -> bool {
        robust_scaling_initialized(self.inner)
    }

    fn process_internal(&self, data: &Matrix, inverse: bool) -> Result<Matrix, &'static str> {
        if !self.is_fitted() {
            return Err("robust scaler is not fitted");
        }
        if self.cols != Some(data.cols()) {
            return Err("cols must match fitted feature dimension");
        }
        let mut out = Matrix::new(data.rows(), data.cols());
        robust_scaling_process(
            self.inner,
            data.data().as_ptr(),
            data.rows() as FlucomaIndex,
            data.cols() as FlucomaIndex,
            out.data_mut().as_mut_ptr(),
            inverse,
        );
        Ok(out)
    }
}

impl Drop for RobustScale {
    fn drop(&mut self) {
        robust_scaling_destroy(self.inner);
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robust_scale_then_inverse_returns_input() {
        let data =
            Matrix::from_vec(vec![1.0, 10.0, 3.0, 20.0, 5.0, 30.0, 1000.0, -999.0], 4, 2).unwrap();
        let mut r = RobustScale::new(25.0, 75.0).unwrap();
        let scaled = r.fit_transform(&data).unwrap();
        let inv = r.inverse_transform(&scaled).unwrap();
        for (a, b) in data.data().iter().zip(inv.data().iter()) {
            assert!((a - b).abs() < 1e-8, "expected {a}, got {b}");
        }
    }

    #[test]
    fn transform_before_fit_fails() {
        let r = RobustScale::new(25.0, 75.0).unwrap();
        let data = Matrix::from_vec(vec![1.0, 2.0], 1, 2).unwrap();
        let err = r.transform(&data).unwrap_err();
        assert_eq!(err, "robust scaler is not fitted");
    }
}
