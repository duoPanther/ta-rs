
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::errors::{Result, TaError};
use crate::{Next, Period, Reset};

/// Highest High Value (HHV).
///
/// Computes the highest value over a specified period in a time series.
///
/// # Parameters
///
/// * _period_ - Number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta_panther::indicators::HighestHighValue;
/// use ta_panther::Next;
///
/// let mut hhv = HighestHighValue::new(3).unwrap();
/// assert_eq!(hhv.next(10.0), 10.0);
/// assert_eq!(hhv.next(12.0), 12.0);
/// assert_eq!(hhv.next(8.0), 12.0);
/// assert_eq!(hhv.next(13.0), 13.0);
/// ```
///
/// # Links
///
#[doc(alias = "HHV")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighestHighValue {
    period: usize,
    index: usize,
    count: usize,
    deque: Box<[f64]>,
}

impl HighestHighValue {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                deque: vec![f64::NEG_INFINITY; period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for HighestHighValue {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for HighestHighValue {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.deque[self.index] = input;
        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };
        if self.count < self.period {
            self.count += 1;
        }
        self.deque[..self.count]
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}

impl Reset for HighestHighValue {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        for i in 0..self.period {
            self.deque[i] = f64::NEG_INFINITY;
        }
    }
}

impl Default for HighestHighValue {
    fn default() -> Self {
        Self::new(7).unwrap()
    }
}

impl fmt::Display for HighestHighValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HHV:{}", self.period)
    }
}
