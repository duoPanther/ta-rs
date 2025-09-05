use std::error::Error;
use std::fmt::{Display, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, TaError>;
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TaError {
    InvalidParameter,
    DataItemIncomplete,
    DataItemInvalid,
}

impl Display for TaError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            TaError::InvalidParameter => write!(f, "invalid parameter"),
            TaError::DataItemIncomplete => write!(f, "data item is incomplete"),
            TaError::DataItemInvalid => write!(f, "data item is invalid"),
        }
    }
}

impl Error for TaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TaError::InvalidParameter => None,
            TaError::DataItemIncomplete => None,
            TaError::DataItemInvalid => None,
        }
    }
}
