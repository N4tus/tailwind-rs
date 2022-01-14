use std::cmp::Ordering;

use super::*;

impl From<u32> for NumericValue {
    fn from(n: u32) -> Self {
        Self::Number { n: n as f32, negative: false, can_be_negative: false }
    }
}

impl From<i32> for NumericValue {
    fn from(n: i32) -> Self {
        Self::Number { n: n as f32, negative: false, can_be_negative: false }
    }
}

impl Display for NumericValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number { n, .. } => write!(f, "{}", n.abs()),
            Self::Keyword(value) => write!(f, "{}", value),
            Self::Arbitrary(value) => value.write(f),
        }
    }
}
