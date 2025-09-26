use std::collections::VecDeque;
use std::fmt;


use serde::{Deserialize, Serialize};
use crate::errors::{Result};
use crate::{Next, Period, Reset};

/// Cross Below Indicator.
///
/// Detects when a time series crosses below a specified threshold value.
/// Returns `true` when the previous value is greater than or equal to the threshold
/// and the current value is less than the threshold.
///
/// # Formula
///
/// For a given time series \( p_t \) and threshold \( b \):
/// - \( \text{CrossBelow}_t = (p_{t-1} \geq b) \land (p_t < b) \)
///
/// Where:
/// - \( p_t \) = value at time \( t \)
/// - \( b \) = threshold value
/// - \( \text{CrossBelow}_t \) = `true` if the series crosses below \( b \), `false` otherwise
///
/// # Parameters
///
/// * `threshold` - The threshold value to check for crossing (floating-point number).
///
/// # Example
///
/// ```
/// use ta_panther::indicators::CrossBelow;
/// use ta_panther::Next;
///
/// let mut cross = CrossBelow::new(10.0).unwrap();
/// println!("Input: 11.0, Output: {}", cross.next(11.0)); // false
/// println!("Input: 9.0, Output: {}", cross.next(9.0));  // true
/// println!("Input: 8.0, Output: {}", cross.next(8.0));  // false
/// println!("Input: 11.0, Output: {}", cross.next(11.0)); // false
/// println!("Input: 9.0, Output: {}", cross.next(9.0));  // true
/// ```
///
/// # Links
///
#[doc(alias = "CROSS_BELOW")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBelow {
    threshold: f64,
    deque: VecDeque<f64>,
}

impl CrossBelow {
    pub fn new(threshold: f64) -> Result<Self> {
        Ok(Self {
            threshold,
            deque: VecDeque::with_capacity(2),
        })
    }
}

impl Period for CrossBelow {
    fn period(&self) -> usize {
        2
    }
}

impl Next<f64> for CrossBelow {
    type Output = bool;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.deque.len() == 2 {
            self.deque.pop_front();
        }
        self.deque.push_back(input);

        if self.deque.len() < 2 {
            return false;
        }

        let prev = self.deque[0];
        let curr = self.deque[1];
        prev >= self.threshold && curr < self.threshold
    }
}

impl Reset for CrossBelow {
    fn reset(&mut self) {
        self.deque.clear();
    }
}

impl Default for CrossBelow {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl fmt::Display for CrossBelow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CROSS_BELOW({})", self.threshold)
    }
}