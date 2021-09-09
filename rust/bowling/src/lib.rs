#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

type Result<T> = std::result::Result<T, Error>;

const FRAMES_NUMBER: usize = 10;

#[derive(Debug)]
pub struct BowlingGame {
    current_frame: usize, // 1..10
    pins_left: u16,
    frames: [Frame; FRAMES_NUMBER],
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 1,
            pins_left: 10,
            frames: [Frame::NoRolls; 10],
        }
    }

    fn is_complete(&self) -> bool {
        self.frames[FRAMES_NUMBER - 1].score().is_some()
    }

    pub fn roll(&mut self, pins: u16) -> Result<()> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        } else {
            self.pins_left -= pins;
        }

        for frame in self.frames[..self.current_frame].iter_mut() {
            frame.update_with_roll(pins);
        }

        if self.frames[self.current_frame - 1].is_finished() {
            // last frame?
            if self.current_frame == FRAMES_NUMBER {
                // no more pins? renew
                if self.pins_left == 0 {
                    self.pins_left = 10;
                }
            } else {
                // Move to the next frame and renew pins
                self.current_frame += 1;
                self.pins_left = 10;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.frames.iter().rev().map(|f| f.score()).sum()
    }
}

use frame::Frame;

mod frame {
    use Frame::*;

    #[derive(Debug, Clone, Copy)]
    pub enum Frame {
        NoRolls,
        SingleRoll(u16),
        Open(u16),
        Spare,
        SpareCalculated(u16),
        Strike,
        StrikeSingleRoll(u16),
        StrikeCalculated(u16),
    }

    impl Frame {
        pub fn is_finished(&self) -> bool {
            match self {
                NoRolls | SingleRoll(_) => false,
                _ => true,
            }
        }

        pub fn score(&self) -> Option<u16> {
            match self {
                Open(x) | SpareCalculated(x) | StrikeCalculated(x) => Some(*x),
                _ => None,
            }
        }

        pub fn update_with_roll(&mut self, pins: u16) {
            *self = match &self {
                NoRolls => match pins {
                    x if x < 10 => SingleRoll(x),
                    10 => Strike,
                    _ => unreachable!(),
                },

                SingleRoll(x) => match *x + pins {
                    x if x < 10 => Open(x),
                    10 => Spare,
                    _ => unreachable!(),
                },

                Spare => SpareCalculated(10 + pins),
                Strike => StrikeSingleRoll(10 + pins),
                StrikeSingleRoll(x) => StrikeCalculated(*x + pins),
                _ => return,
            };
        }
    }
}
