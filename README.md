# 2048

2048 game implemented in terminal in Rust 🦀

![image](https://github.com/Martan03/2048/assets/46300167/ebdd7bd9-be3a-492b-bb9b-c48364fa9272)

## Contents
- [Installation](#installation)
- [Usage](#usage)
- [Detailed description](#detailed-description)
    - [Game goal](#game-goal)
    - [Joining tiles](#joining-tiles)
    - [Game end](#game-end)
    - [Controls](#controls)
- [Technologies](#technologies)
- [Links](#links)

## Installation
You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:
```
cargo build -r
```
After its done compiling, you can start it in `./target/release/tui2048`

## Usage
Start 2048 game:
```
./tui2048
```

## Detailed description

### Game goal

If you don't know what 2048 game is, your goal is to get tile with 2048 value.
In order to get one, you need to combine tiles with the same value. Combining
tiles is really easy. When they're next to each other, you can "push" them
together and they will join to one tile with value equal to their sum.

### Joining tiles

You can use arrow keys to move all tiles in corresponding direction (for
example, up arrow will move all tiles up as much as possible). When two tiles
with the same value are next to each other in the corresponding direction, they
will join and therefore you get one with greater value.

### Game end

I already mentioned the goal of the game - getting tile with value of 2048 -
but what can also happen is that you fill the whole board and you can't join
any tiles. This is game over. You can then restart the game with `r` key.

### Controls
- `Arrow keys`: moving tiles
- `Esc` / `q`: exits the game
- `r`: restarts the game

## Technologies
I used these libraries:
- [crossterm](https://crates.io/crates/crossterm)
  - Creating key listeners
- [termint](https://crates.io/crates/termint)
  - Creating TUI itself
- [rand](https://crates.io/crates/rand)
  - Generating random number

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [tui2048](https://github.com/Martan03/tui2048)
- **Author website:** [martan03.github.io](https://martan03.github.io)

