#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum FrameType {
    Strike,
    Spare,
    Open,
}

use FrameType::*;

pub struct BowlingGame {
    game_is_done: bool,
    scores: Vec<u16>,
    frames: Vec<FrameType>,
    second_roll_is_due: bool,
    num_fill_balls: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            game_is_done: false,
            scores: Vec::new(),
            frames: Vec::new(),
            second_roll_is_due: false,
            num_fill_balls: 2,
        }
    }

    fn first_roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins == 10 {
            self.frames.push(Strike);
        } else {
            self.second_roll_is_due = true;
        }
        self.scores.push(pins);
        Ok(())
    }

    fn second_roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.scores[self.scores.len() - 1] + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.scores[self.scores.len() - 1] + pins == 10 {
            self.frames.push(Spare);
        } else {
            self.frames.push(Open);
            if self.frames.len() == 10 {
                self.game_is_done = true;
            }
        }
        self.scores.push(pins);
        self.second_roll_is_due = false;
        Ok(())
    }

    fn fill_ball(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames[self.frames.len() - 1] == Spare {
            self.scores.push(pins);
            self.game_is_done = true;
        } else if self.frames[self.frames.len() - 1] == Strike {
            self.scores.push(pins);
            self.num_fill_balls -= 1;
            if self.num_fill_balls == 0 {
                let penultimate = self.scores[self.scores.len() - 2];
                let ultimate = self.scores[self.scores.len() - 1];
                if penultimate + ultimate > 20 || (penultimate < 10 && penultimate + ultimate > 10)
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.game_is_done = true;
            }
        }
        Ok(())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.game_is_done {
            return Err(Error::GameComplete);
        } else {
            if self.frames.len() == 10 {
                self.fill_ball(pins)?;
            } else {
                if !self.second_roll_is_due {
                    self.first_roll(pins)?;
                } else {
                    self.second_roll(pins)?;
                }
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_is_done {
            return None;
        }
        let mut score = 0;
        let mut index = 0;
        for frame in self.frames.iter() {
            if frame == &Strike {
                score += self.scores[index] + self.scores[index + 1] + self.scores[index + 2];
                index += 1;
            } else if frame == &Spare {
                score += self.scores[index] + self.scores[index + 1] + self.scores[index + 2];
                index += 2;
            } else {
                score += self.scores[index] + self.scores[index + 1];
                index += 2;
            }
        }
        Some(score)
    }
}
