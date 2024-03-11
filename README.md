# Conway's Game of Life in Rust

## Overview
An implementation of Conway's Game of Life using Rust. This simulation creates a grid of cells that evolve at each step according to a set of rules. Cells can either be alive or dead, and their state in the next generation depends on the number of live neighbors.

## Features
- Grid initialization with dead cells.
- Randomization of the grid's initial state.
- Calculation of live neighbors for each cell.
- Evolution of the grid based on Conway's rules.
- Console-based visualization of the game state.

## Requirements
- Rust programming language
- Cargo (Rust's package manager)

## Getting Started

1. **Install Rust and Cargo**
   
   Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

2. **Clone the Repository**   
git clone https://github.com/sturla-freyr/conway.git
cd conway

3. **Run the Game**
cargo run
