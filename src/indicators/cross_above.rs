use std::collections::VecDeque;
use std::fmt;


use serde::{Deserialize, Serialize};
use crate::errors::{Result};
use crate::{Next, Period, Reset, State};

/// Cross Above Indicator.
///
/// Detects when a time series crosses above a specified threshold value.
/// Returns `true` when the previous value is less than or equal to the threshold
/// and the current value is greater than the threshold.
///
/// # Formula
///
/// For a given time series \( p_t \) and threshold \( b \):
/// - \( \text{CrossAbove}_t = (p_{t-1} \leq b) \land (p_t > b) \)
///
/// Where:
/// - \( p_t \) = value at time \( t \)
/// - \( b \) = threshold value
/// - \( \text{CrossAbove}_t \) = `true` if the series crosses above \( b \), `false` otherwise
///
/// # Parameters
///
/// * `threshold` - The threshold value to check for crossing (floating-point number).
///
/// # Example
///
/// ```
/// use ta_panther::indicators::CrossAbove;
/// use ta_panther::Next;
///
/// let mut cross = CrossAbove::new(10.0).unwrap();
/// println!("Input: 9.0, Output: {}", cross.next(9.0));  // false
/// println!("Input: 11.0, Output: {}", cross.next(11.0)); // true
/// println!("Input: 12.0, Output: {}", cross.next(12.0)); // false
/// println!("Input: 9.0, Output: {}", cross.next(9.0));   // false
/// println!("Input: 11.0, Output: {}", cross.next(11.0)); // true
/// ```
///
/// # Links
///
#[doc(alias = "CROSS_ABOVE")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossAbove {
    threshold: f64,
    deque: VecDeque<f64>,
}

impl CrossAbove {
    pub fn new(threshold: f64) -> Result<Self> {
        Ok(Self {
            threshold,
            deque: VecDeque::with_capacity(2),
        })
    }

    pub fn from_state(threshold: f64, mut deque: VecDeque<f64>) -> Result<Self> {
        while deque.len() > 2 {
            deque.pop_front();
        }

        Ok(Self {
            threshold,
            deque
        })
    }
}

impl Period for CrossAbove {
    fn period(&self) -> usize {
        2
    }
}

impl Next<f64> for CrossAbove {
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
        prev <= self.threshold && curr > self.threshold
    }
}

impl State for CrossAbove {
    type Output = (f64, f64, f64);

    fn state(&self) -> Self::Output {
        (self.threshold, self.deque[0], self.deque[1])
    }
}

impl Reset for CrossAbove {
    fn reset(&mut self) {
        self.deque.clear();
    }
}

impl Default for CrossAbove {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl fmt::Display for CrossAbove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CROSS_ABOVE({})", self.threshold)
    }
}