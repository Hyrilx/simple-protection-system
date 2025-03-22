use std::convert::Into;
use std::ops::BitOr;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Right(u8);

impl From<u8> for Right {
    fn from(value: u8) -> Right {
        Right(value)
    }
}

impl Into<u8> for Right {
    fn into(self) -> u8 {
        self.0
    }
}

impl Right {
    pub fn read() -> Right {
        Right(1 << 0)
    }

    pub fn write() -> Right {
        Right(1 << 1)
    }

    fn execute() -> Right {
        Right(1 << 2)
    }

    pub fn empty() -> Right {
        Right(0)
    }

    pub fn full() -> Right {
        Right::read() | Right::write() | Right::execute()
    }

    pub fn contains(&self, other: &Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn turn_on(&mut self, other: &Self) {
        self.0 |= other.0;
    }

    pub fn turn_off(&mut self, other: &Self) {
        self.0 &= !other.0;
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_any(&self) -> bool {
        !self.is_empty()
    }
}

impl BitOr for Right {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Right(self.0 | rhs.0)
    }
}
