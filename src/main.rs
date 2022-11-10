#![warn(missing_docs)]

//! # moving_asterisk
//!
//! A binary showing an asterisk moving from one place
//! to another in the terminal.

mod config;

use config::AnimationConfig;

use anyhow::{Context, Result};
use clap::Parser;

/// A program which depicts a character moving
/// from one place to another
#[derive(Parser, Debug)]
#[command(about)]
struct Cli {
    /// The amount of units the character will move through
    #[arg(short = 'd', long, default_value_t = 5)]
    distance: usize,

    /// The speed of the character in milliseconds
    #[arg(short, long)]
    speed: u64,

    /// Provide a custom character to animate
    #[arg(short, long = "char", default_value_t = '*')]
    character: char,

    /// Provide a custom starting char
    #[arg(short = 'S', long, default_value_t = '[')]
    start_char: char,

    /// Provide a custom end/goal char
    #[arg(short = 'E', long, default_value_t = ']')]
    end_char: char,

    /// Provide a custom spacer
    #[arg(short = 'R', long, default_value_t = ' ')]
    spacer: char,
}

#[doc(hidden)]
fn main() -> Result<()> {
    let args = Cli::try_parse().with_context(|| "Failed to parse your config :(")?;

    AnimationConfig::new()
        .with_char(args.character)
        .with_bounds((args.start_char, args.end_char))
        .spaced_with(args.spacer)
        .until(args.distance)
        .animate_with(args.speed)
        .with_context(|| "Failed to animate with your specified config :(")?;

    println!("The character has reached its goal!");
    Ok(())
}
