use crate::time::frequency::Frequency;
use crate::time::time_unit::TimeUnit;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Period {
    length: i32,
    units: TimeUnit,
}

impl Period {
    pub fn new(length: i32, units: TimeUnit) -> Period {
        Period { length, units }
    }

    pub fn from_frequency(frequency: Frequency) -> Period {
        unimplemented!()
    }

    pub fn length(&self) -> i32 {
        self.length
    }
    pub fn units(&self) -> TimeUnit {
        self.units
    }
}
