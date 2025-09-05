use std::collections::VecDeque;
use std::fmt;

/// Linear Regression Prediction (FORECAST).
///
/// This indicator uses linear regression to predict the next value in a time series
/// based on the previous `period` values. It fits a straight line to the data points
/// and extrapolates the next value.
///
/// # Formula
///
/// The linear regression line is defined as \( y = mx + b \), where:
/// - \( m \) (slope) = \( \frac{\text{cov}(x, y)}{\text{var}(x)} \)
/// - \( b \) (intercept) = \( \bar{y} - m \cdot \bar{x} \)
/// - Predicted value = \( m \cdot (n + 1) + b \)
///
/// Where:
/// - \( \text{cov}(x, y) \) = covariance between the time steps (\( x \)) and values (\( y \))
/// - \( \text{var}(x) \) = variance of the time steps (\( x \))
/// - \( \bar{x} \), \( \bar{y} \) = mean of \( x \) and \( y \)
/// - \( n \) = number of data points in the current window
/// - \( x = [1, 2, ..., \text{period}] \) = time steps
///
/// # Parameters
///
/// * `period` - Number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta_panther::indicators::LinearRegressionPrediction;
/// use ta_panther::Next;
///
/// let mut lrp = LinearRegressionPrediction::new(3).unwrap();
/// assert_eq!(lrp.next(1.0), 1.0); // First value
/// assert_eq!(lrp.next(2.0), 2.0); // Predict next based on [1.0, 2.0]
/// assert_eq!(lrp.next(3.0), 4.0); // Predict next based on [1.0, 2.0, 3.0]
/// assert_eq!(lrp.next(4.0), 5.0); // Predict next based on [2.0, 3.0, 4.0]
/// assert_eq!(lrp.next(5.0), 6.0); // Predict next based on [3.0, 4.0, 5.0]
/// ```
///
/// # Links
///
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::errors::{Result, TaError};
use crate::{Next, Period, Reset};

#[doc(alias = "FORECAST")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LinearRegressionPrediction {
    period: usize,
    deque: VecDeque<f64>,
    x: Vec<f64>, // 缓存自变量 x
    mean_x: f64, // 缓存 x 的均值
}

impl LinearRegressionPrediction {
    pub fn new(period: usize) -> Result<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter);
        }
        let x: Vec<f64> = (1..=period).map(|x| x as f64).collect();
        let mean_x = (period as f64 + 1.0) / 2.0;
        Ok(Self {
            period,
            deque: VecDeque::with_capacity(period),
            x,
            mean_x,
        })
    }
}

impl Next<f64> for LinearRegressionPrediction {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let value = input;
        if self.deque.len() == self.period {
            self.deque.pop_front();
        }
        self.deque.push_back(value);
        let slice = self.deque.iter().copied().collect::<Vec<_>>();
        if slice.is_empty() {
            return 0.0;
        }
        let n = slice.len() as f64;
        let mean_y = slice.iter().sum::<f64>() / n;
        let mut cov_xy = 0.0;
        let mut var_x = 0.0;
        for (xi, &yi) in self.x.iter().zip(slice.iter()) {
            cov_xy += (xi - self.mean_x) * (yi - mean_y);
            var_x += (xi - self.mean_x).powi(2);
        }
        let slope = if var_x != 0.0 { cov_xy / var_x } else { 0.0 };
        let intercept = mean_y - slope * self.mean_x;
        let result = slope * (n + 1.0) + intercept;
        result
    }
}

impl Period for LinearRegressionPrediction {
    fn period(&self) -> usize {
        self.period
    }
}

impl Reset for LinearRegressionPrediction {
    fn reset(&mut self) {
        self.deque.clear();
    }
}

impl Default for LinearRegressionPrediction {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for LinearRegressionPrediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FORECAST:{}", self.period)
    }
}
