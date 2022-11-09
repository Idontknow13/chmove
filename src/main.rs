#![warn(missing_docs)]

//! # moving_asterisk
//!
//! A binary showing an asterisk moving from one place
//! to another in the terminal.

use clap::Parser;
use std::{
    io::{stdout, Result as IOResult, Write},
    thread,
    time::Duration,
};

/// A program which depicts a character moving
/// from one place to another
#[derive(Parser, Debug)]
#[command(about)]
struct Cli {
    /// The amount of units the character will traverse
    #[arg(short = 'd', long, default_value_t = 5)]
    distance: usize,

    /// The speed of the character in milliseconds
    #[arg(short, long)]
    speed: u64,

    /// Provide a custom character to animate
    #[arg(short, long = "char", default_value_t = '*')]
    character: char,
}

#[doc(hidden)]
fn main() {
    let args = Cli::parse();

    let config = AnimationConfig::init()
        .with_char(args.character)
        .until(args.distance);

    if config.move_with(args.speed).is_ok() {
        println!("The character reached its goal!");
    }
}

// TODO: Put this in separate module
// TODO: Change this config to a builder
/// A config struct containing all the needed context
/// for the movement animation.
struct AnimationConfig {
    character: char,
    distance: usize,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            character: '*',
            distance: 5,
        }
    }
}

impl AnimationConfig {
    /// Create a new instance of `AnimationConfig`
    fn init() -> Self {
        Self::default()
    }

    /// Specify a unique character for the config.
    fn with_char(mut self, ch: char) -> Self {
        self.character = ch;
        self
    }

    /// Specify a traveling distance for the animation.
    fn until(mut self, distance: usize) -> Self {
        self.distance = distance;
        self
    }

    /// Creates the moving animation for the asterisk given
    /// a certain distance and speed in ms.
    fn move_with(self, speed: u64) -> IOResult<()> {
        const CLEAR_LINE_ABOVE: &str = "\x1b[1A\x1b[2K";
        let distance = self.distance;

        for traveled in 0..=distance {
            if traveled > 0 {
                print!("{CLEAR_LINE_ABOVE}");
            }

            println!(
                "[{front_space}{character}{lead_space}]",
                front_space = " ".repeat(traveled),
                character = self.character,
                lead_space = " ".repeat(distance - traveled)
            );
            stdout().flush()?;
            thread::sleep(Duration::from_millis(speed));
        }
        Ok(())
    }
}
