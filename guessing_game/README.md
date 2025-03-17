# Improved Guessing Game

A feature-rich enhancement of the classic Rust Book guessing game with difficulty levels, time tracking, and a persistent highscore system.

## 🎮 Features

This implementation significantly expands upon the basic guessing game from "The Rust Programming Language" book with:

- **Multiple Difficulty Levels**
  - Easy: Guess a number between 1-50
  - Medium: Guess a number between 1-100
  - Hard: Guess a number between 1-200

- **Performance Tracking**
  - Counts the number of guesses needed
  - Measures the time taken to find the correct number
  - Calculate efficiency based on both metrics

- **Comprehensive Highscore System**
  - Separate leaderboards for each difficulty
  - Records player name, guess count, and completion time
  - Ranks players first by fewest guesses, then by fastest times
  - Displays top 5 scores in each difficulty category

- **Improved User Experience**
  - Play multiple rounds in a single session
  - Clear feedback on guesses
  - Option to quit at any time
  - Formatted highscore tables

## 🚀 Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/improved-guessing-game.git
   cd improved-guessing-game
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the game:
   ```
   cargo run --release
   ```

## 📖 How to Play

1. Select a difficulty level (1-3)
2. Guess a number within the given range
3. Receive feedback (too small, too big)
4. Continue guessing until you find the correct number
5. Enter your name to record your score
6. View the highscores
7. Choose to play again or exit

## 🔄 Differences from the Standard Rust Book Game

| Feature | Standard Rust Book Game | Improved Guessing Game |
|---------|-------------------------|------------------------|
| Range | Fixed 1-100 | Multiple difficulty levels |
| Attempts | Not tracked | Counted and displayed |
| Timing | Not implemented | Measures duration to win |
| Multiple Games | Not supported | Play multiple rounds |
| Highscores | Not implemented | Comprehensive system with rankings |
| Difficulty | Single level | Three progressive levels |
| Player Identity | Not tracked | Personalized highscores |

## 📝 Example Gameplay

```
Hello, let's play a guessing game!

Choose a difficulty level (1-3):
1. Easy
2. Medium
3. Hard
2

You selected Medium. The secret number will be between 1 and 100.
Guess a number between 1 and 100!

50
You guessed 50
Too small!

75
You guessed 75
Too big!

62
You guessed 62
Too small!

68
You guessed 68
You win! You took 4 guesses and 15 seconds.

Enter your name for the highscore:
Player1

===== HIGHSCORES =====

Medium Mode:
Name           Guesses    Time (s)   
-----------------------------------
1. Player1      4          15        

Do you want to play again? (yes/no)
```

## 🛠️ Future Improvements

- Saving highscores to a file for persistence between sessions
- Adding a graphical user interface
- Implementing more game modes (timed mode, limited guesses)
- Network multiplayer capabilities

## 📄 License

This project is available under the MIT License - feel free to use, modify, and distribute as you wish.

## 🙏 Acknowledgments

- The Rust Programming Language book for the original guessing game concept
- Rust community for continued support and inspiration
