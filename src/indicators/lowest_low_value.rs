#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::errors::{Result, TaError};
use crate::{Next, Period, Reset};

/// Lowest Low Value (LLV).
///
/// Computes the lowest value over a specified period in a time series.
///
/// # Parameters
///
/// * _period_ - Number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta_panther::indicators::LowestLowValue;
/// use ta_panther::Next;
///
/// let mut llv = LowestLowValue::new(3).unwrap();
/// assert_eq!(llv.next(10.0), 10.0);
/// assert_eq!(llv.next(8.0), 8.0);
/// assert_eq!(llv.next(12.0), 8.0);
/// assert_eq!(llv.next(7.0), 7.0);
/// ```
///
/// # Links
///
#[doc(alias = "LLV")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LowestLowValue {
    period: usize,
    index: usize,
    count: usize,
    deque: Box<[f64]>,
}

impl LowestLowValue {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                deque: vec![f64::INFINITY; period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for LowestLowValue {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for LowestLowValue {
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
            .fold(f64::INFINITY, |a, &b| a.min(b))
    }
}


impl Reset for LowestLowValue {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        for i in 0..self.period {
            self.deque[i] = f64::INFINITY;
        }
    }
}

impl Default for LowestLowValue {
    fn default() -> Self {
        Self::new(7).unwrap()
    }
}

impl fmt::Display for LowestLowValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LLV:{}", self.period)
    }
}
