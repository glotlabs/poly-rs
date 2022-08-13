use std::convert::TryInto;
use std::ops::Sub;
use std::time;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Posix {
    milliseconds: i128,
}

impl Posix {
    pub fn from_millis(milliseconds: i128) -> Posix {
        Posix { milliseconds }
    }

    pub fn as_millis(&self) -> i128 {
        self.milliseconds
    }

    pub fn as_secs(&self) -> i128 {
        self.milliseconds / 1000
    }
}

impl Sub for Posix {
    type Output = time::Duration;

    fn sub(self, other: Self) -> Self::Output {
        let diff = self.milliseconds - other.milliseconds;
        time::Duration::from_millis(diff.try_into().unwrap_or(0))
    }
}
