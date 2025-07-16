# 🧠 Bitboard-Based Chess in Rust

This project implements a basic chessboard in **Rust** using **bitboards**, enums, and structs. It displays the starting state of a chess game with all pieces properly arranged and outputs the board to the console.

---

## 📁 File Structure

```
src/
├── main.rs
├── game.rs
├── knigthattacks.rs
├── movegenration.rs
├── pawnattack.rs
├── position.rs
├── rayattacks.rs
├── utils.rs
```

---

## 🧱 Core Concepts

### ✅ Bitboard (`u64`)

A **bitboard** is a 64-bit unsigned integer where each bit represents a square on the chessboard.

* Bit `0` → square `a1`
* Bit `1` → square `b1`
* ...
* Bit `63` → square `h8`

This is used to efficiently store and process piece positions.

---


### BOARD VIEW
![result](result.png)

---
---

### ATTACK PATTERN
![1](1.png)
![2](2.png)
![3](3.png)
![4](4.png)

---

### ♟️ FEN (Forsyth–Edwards Notation) Support

This project includes support for FEN (Forsyth–Edwards Notation), allowing the board state to be serialized and deserialized in a standard format used by most chess engines and GUIs.

The following example is from the FEN specification:[10]
Here is the FEN for the starting position:
```bash
rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
```
And after the move 1.e4:
```bash
rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1
```
And then after 1...c5:
```bash
rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2
```
And then after 2.Nf3:
```bash
rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
```
```

## ✅ Features:
Parse FEN strings to initialize the game board.

Generate FEN strings from the current game state.

Track and update metadata such as castling rights, en passant targets, halfmove clocks, and fullmove numbers.
```bash
<1> Piece placement
<2> Active color
<3> Castling availability
<4> En passant target square
<5> Halfmove clock
<6> Fullmove number
```
---

## 🚀 How to Run

1. Clone this repo:

```bash
git clone https://github.com/Sohardbegraj/chess.in.git
cd rust-bitboard-chess
```

2. Run the project:

```bash
cargo run
```

You’ll see the board printed in the terminal.

---

## 🧠 Next Steps / Ideas

* Add move generation
* Handle turns (white/black)
* Legal move validation
* GUI or TUI visualization
* Piece capturing and check/checkmate logic

---

