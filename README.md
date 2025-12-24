# rust-maze-solver
Maze generator and backtracking solver in Rust

A Rust implementation of a maze generator using the Randomized Depth-First Search (Recursive Backtracking) algorithm. 

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd maze_solver
```

2. Build the project:
```bash
cargo build
```

3. Run the maze generator:
```bash
cargo run
```

## Usage

### Basic Example

```rust
use maze_solver::Maze;

fn main() {
    // Generate a 21x21 maze
    let maze = Maze::generate(21, 21);
    
    // Display the maze
    maze.display();
}
```

### Programmatic Access

```rust
use maze_solver::Maze;

fn main() {
    let maze = Maze::generate(15, 15);
    
    // Access maze properties
    println!("Width: {}", maze.width());
    println!("Height: {}", maze.height());
    
    // Access the grid directly
    let grid = maze.grid();
    println!("Cell at (1,1): {}", grid[1][1]); // Should be ' ' (path)
    println!("Cell at (0,0): {}", grid[0][0]); // Should be '#' (wall)
}
```

### Custom Dimensions

```rust
let maze = Maze::generate(31, 41); // Generate a 31x41 maze
maze.display();
```

**Note**: If you provide even dimensions, they will be automatically adjusted to odd dimensions to ensure proper wall/path structure.

## License

This project is open source and available under the MIT License.


