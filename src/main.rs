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
    /// The amount of units the character will cross through
    #[arg(short = 'd', long)]
    distance: usize,

    /// The speed of the character in milliseconds
    #[arg(short, long)]
    speed: u64,

    /// Provide a custom character to animate
    #[arg(short, long = "char")]
    character: Option<char>,
}

#[doc(hidden)]
fn main() {
    let args = Cli::parse();

    let config = AnimationConfig::new(args.character, args.distance);

    if config.move_with(args.speed).is_ok() {
        println!("The asterisk reached its goal!");
    }
}

// TODO: Put this in separate module
// TODO: Change this config to a builder
/// A config struct containing all the needed context
/// for the movement animation.
struct AnimationConfig {
    used_char: Option<char>,
    distance: usize,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            used_char: None,
            distance: 5,
        }
    }
}

impl AnimationConfig {
    /// Create a new instance of `AnimationConfig`.
    fn new(ch: Option<char>, distance: usize) -> Self {
        Self {
            used_char: ch,
            distance,
        }
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
                character = self.used_char.unwrap_or('*'),
                lead_space = " ".repeat(distance - traveled)
            );
            stdout().flush()?;
            thread::sleep(Duration::from_millis(speed));
        }
        Ok(())
    }
}
