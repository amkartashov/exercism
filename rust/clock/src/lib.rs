#![deny(missing_docs)]

//! This crate is for https://exercism.io/tracks/rust/exercises/clock

use std::fmt;

/// Struct Clock describes day time without the dates.
#[derive(Debug, Default, PartialEq)]
pub struct Clock(internal::Clock);

impl Clock {
    /// create new Clock
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(internal::Clock::new(hours, minutes))
    }

    /// create new Clock by adding minutes
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self(self.0.add_minutes(minutes))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.0.display_hours(),
            self.0.display_minutes()
        )
    }
}

mod internal {

    const HOURS_IN_DAY: i32 = 24;
    const MINUTES_IN_HOUR: i32 = 60;
    const MINUTES_IN_DAY: i32 = MINUTES_IN_HOUR * HOURS_IN_DAY;

    /// internal implementation of Clock.
    /// Reasons:
    ///   * disallow direct construction (in this crate)
    ///   * make sure invariants are preserved: 0 <= minutes < MINUTES_IN_DAY
    #[derive(Debug, Default, PartialEq)]
    pub(crate) struct Clock {
        minutes: u16, // private field
    }

    impl Clock {
        pub(crate) fn new(hours: i32, minutes: i32) -> Self {
            // get remainder to prevent overflow
            // u16 is enough so we can be sure type cast is lossless
            let hours = hours.rem_euclid(HOURS_IN_DAY) as u16;
            let minutes = minutes.rem_euclid(MINUTES_IN_DAY) as u16;
            Clock::_new(hours, minutes)
        }

        /// construct Clock from u16 values
        /// caller should make sure that:
        /// * 0 <= hours < HOURS_IN_DAY
        /// * 0 <= minutest < MINUTES_IN_DAY
        fn _new(hours: u16, minutes: u16) -> Self {
            Clock {
                // Using Euclidian remainder to ensure we have positive values
                minutes: (hours * (MINUTES_IN_HOUR as u16) + minutes)
                    .rem_euclid(MINUTES_IN_DAY as u16),
            }
        }

        pub(crate) fn display_minutes(&self) -> u8 {
            (self.minutes % MINUTES_IN_HOUR as u16) as u8
        }

        pub(crate) fn display_hours(&self) -> u8 {
            (self.minutes / MINUTES_IN_HOUR as u16) as u8
        }

        pub(crate) fn add_minutes(&self, minutes: i32) -> Self {
            // get remainder to prevent overflow
            // u16 is enough so we can be sure type cast is lossless and sum won't overflow
            let minutes = minutes.rem_euclid(MINUTES_IN_DAY) as u16;
            Clock::_new(0, self.minutes + minutes)
        }
    }
}
