## Blanks: a Leetcode Game

A Rust implementation of a game to practice Leetcode problems by filling in missing characters.

### Installation

1. Make sure you have Rust installed. If not, install it from [https://rustup.rs/](https://rustup.rs/)
2. Clone the repository or navigate to the project directory
3. Build the project: `cargo build --release`
4. The executable will be available in `target/release/blanks`

### Usage

```
blanks --language [language] --diff [difficulty] --problem [number] --mode [mode]
```

Example:
```
blanks --language python --diff 0.3 --mode sequential
```

### Gameplay
Running blanks will take you to a random leetcode problem and remove a proportion of the characters from the problem solution. 
You will then be prompted to fill in the blanks by just directly editing the file. 
The game will keep track of time to complete the problem and after you fill in all the blanks continue to adding blanks to the next problem. 
Stopping the executible will restore the directory to its original state and output your score.
The score is calculated by the time it took to complete the problem and the number of blanks you filled in correctly.

### Options
- If you have run blanks before when specifying the language and mode, then running blanks again will run the same language and mode as before without having to specify the options.
- language: Any lanuage in the root directory's folder is suported. Defaults to python.
- diff: The proportion of characters that are removed from the file (from 0 to 1). Defaults to 0.2.
- problem: If given a number it will start the game with that problem number. Defaults to a random.
- mode: 'random' or 'sequential'. Defaults to random.
