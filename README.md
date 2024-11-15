# Guessing Game in Rust

This is a simple command-line guessing game implemented in Rust, using the `rand` and `colored` crates for random number generation and enhanced terminal output.

## How It Works

- The program generates a random secret number between 1 and 10.
- The player inputs their guesses in the terminal.
- The program provides feedback on each guess:
  - **Too Small**: If the guess is less than the secret number (displayed in red).
  - **Too Big**: If the guess is greater than the secret number (displayed in red).
  - **Correct**: If the guess matches the secret number (displayed in green), and the game ends.

## Features

- **Random Number Generation**: Uses the `rand` crate to generate a random number.
- **Robust Input Handling**: Ensures valid input by parsing user input and prompting again if invalid.
- **Enhanced Feedback**: Uses the `colored` crate for color-coded feedback.

## Requirements

To run this program, ensure you have Rust installed. Install dependencies by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8"
colored = "2.0"







