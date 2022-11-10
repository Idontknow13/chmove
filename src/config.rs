//! Houses the `AnimationConfig` struct used to create
//! custom specifications for the chmove animation.

use anyhow::Result;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

/// A config struct containing all the needed context
/// for the movement animation.
pub struct AnimationConfig {
    character: char,
    begin_and_end: (char, char),
    spacer: char,
    distance: usize,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            character: '*',
            begin_and_end: ('|', '|'),
            spacer: ' ',
            distance: 5,
        }
    }
}

impl AnimationConfig {
    /// Create a new instance of `AnimationConfig`
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a unique character for the config.
    pub fn with_char(mut self, ch: char) -> Self {
        self.character = ch;
        self
    }

    /// Specify a traveling distance for the animation.
    pub fn until(mut self, distance: usize) -> Self {
        self.distance = distance;
        self
    }

    /// Specify a beginning character.
    pub fn start_at(mut self, start: char) -> Self {
        self.begin_and_end.0 = start;
        self
    }

    /// Specify an end/goal character.
    pub fn end_at(mut self, end: char) -> Self {
        self.begin_and_end.1 = end;
        self
    }

    /// Specify a beginning and end character using a tuple.
    pub fn with_bounds(self, begin_and_end: (char, char)) -> Self {
        self.start_at(begin_and_end.0).end_at(begin_and_end.1)
    }

    /// Specify a spacer character used to show the traveled
    /// and untraveled parts.
    pub fn spaced_with(mut self, spacer: char) -> Self {
        self.spacer = spacer;
        self
    }

    /// Creates the moving animation for the character given
    /// a config.
    pub fn animate_with(self, speed: u64) -> Result<()> {
        const CLEAR_LINE_ABOVE: &str = "\x1b[1A\x1b[2K";

        let distance = self.distance;
        let spacer = self.spacer.to_string();

        for traveled in 0..=distance {
            if traveled > 0 {
                print!("{CLEAR_LINE_ABOVE}");
            }

            println!(
                "{begin}{front_space}{character}{lead_space}{end}",
                begin = self.begin_and_end.0,
                front_space = spacer.repeat(traveled),
                character = self.character,
                lead_space = spacer.repeat(distance - traveled),
                end = self.begin_and_end.1,
            );
            stdout().flush()?;
            thread::sleep(Duration::from_millis(speed));
        }
        Ok(())
    }
}
